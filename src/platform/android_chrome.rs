//! Android edge-to-edge system bars, drawn by the app instead of the OS.
//!
//! Dioxus hosts the UI in a WebView, and by default the window stops at the system bars so Android
//! paints them itself: a grey or black strip that does not match Dew's theme. CSS alone cannot
//! fix this, because the Android WebView reports 0px for the status and navigation bars until
//! Chrome WebView 144, so the app goes natively through JNI:
//!
//! 1. [`apply_system_bars`] asks the window to draw behind the bars and to leave them transparent.
//! 2. [`push_insets`] reads the real insets and writes them into the `--dew-inset-*` CSS
//!    variables, which `assets/app.css` merges with `env(safe-area-inset-*)` through `max()`.
//!
//! Every call runs on Android's UI thread via wry's dispatch queue and has to survive an older OS,
//! so each step clears a pending exception and carries on: a missing API degrades to the previous
//! behaviour rather than crashing the app.

use dioxus::mobile::wry::prelude::dispatch;
use dioxus::prelude::*;
use jni::objects::{JObject, JValue};
use jni::sys::jint;
use jni::JNIEnv;
use std::convert::TryInto;

/// How many times to re-read the insets while the first layout has not happened yet.
/// `getRootWindowInsets` returns null until then, so a single early call would leave the bars
/// unpadded for the rest of the session.
const INSET_RETRIES: u32 = 5;
const RETRY_DELAY_MS: u64 = 100;

/// Clears a pending exception so the next JNI call is safe. Returns true if one was pending, which
/// means the call that raised it failed and its result must not be trusted.
fn take_exception(env: &mut JNIEnv) -> bool {
    match env.exception_check() {
        Ok(true) => {
            // The description needs more JNI work and we are already on a recovery path.
            let _ = env.exception_clear();
            true
        }
        _ => false,
    }
}

/// Calls a method returning `int`, e.g. `getSystemWindowInsetTop()I`.
fn call_i32<'a, 'b>(env: &mut JNIEnv<'a>, obj: &JObject<'b>, name: &str, sig: &str) -> Option<i32> {
    let res = env.call_method(obj, name, sig, &[]).ok()?;
    if take_exception(env) {
        return None;
    }
    res.i().ok()
}

/// Calls a `void` method, e.g. `setStatusBarColor(I)V`.
fn call_void<'a, 'b>(env: &mut JNIEnv<'a>, obj: &JObject<'b>, name: &str, sig: &str, args: &[JValue<'a, '_>]) -> bool {
    match env.call_method(obj, name, sig, args) {
        Ok(_) => !take_exception(env),
        Err(_) => false,
    }
}

/// Calls a method returning an object, e.g. `getWindow()Landroid/view/Window;`.
fn call_obj<'a, 'b>(env: &mut JNIEnv<'a>, obj: &JObject<'b>, name: &str, sig: &str) -> Option<JObject<'a>> {
    let res = env.call_method(obj, name, sig, &[]).ok()?;
    if take_exception(env) {
        return None;
    }
    res.try_into().ok()
}

/// Calls a method taking arguments and returning an object,
/// e.g. `getInsets(I)Landroid/graphics/Insets;`.
fn call_obj_args<'a, 'b>(
    env: &mut JNIEnv<'a>,
    obj: &JObject<'b>,
    name: &str,
    sig: &str,
    args: &[JValue<'a, '_>],
) -> Option<JObject<'a>> {
    let res = env.call_method(obj, name, sig, args).ok()?;
    if take_exception(env) {
        return None;
    }
    res.try_into().ok()
}

/// Reads a public `int` field, e.g. `android.graphics.Insets.top`.
fn field_i32<'a, 'b>(env: &mut JNIEnv<'a>, obj: &JObject<'b>, name: &str) -> Option<i32> {
    let res = env.get_field(obj, name, "I").ok()?;
    if take_exception(env) {
        return None;
    }
    res.i().ok()
}

/// Reads a public `float` field, e.g. `DisplayMetrics.density`.
fn field_f32<'a, 'b>(env: &mut JNIEnv<'a>, obj: &JObject<'b>, name: &str) -> Option<f32> {
    let res = env.get_field(obj, name, "F").ok()?;
    if take_exception(env) {
        return None;
    }
    res.f().ok()
}

/// `android.os.Build.VERSION.SDK_INT`, or 0 when it cannot be read.
fn sdk_int(env: &mut JNIEnv<'_>) -> i32 {
    let Ok(version) = env.find_class("android/os/Build$VERSION") else {
        let _ = take_exception(env);
        return 0;
    };
    field_i32(env, &version, "SDK_INT").unwrap_or(0)
}

/// `android.view.WindowInsets.Type.systemBars() | ...displayCutout()`.
///
/// This is the API 30 replacement for the old `getSystemWindowInset*` getters.
fn system_bar_mask<'a>(env: &mut JNIEnv<'a>) -> Option<i32> {
    let cls = env.find_class("android/view/WindowInsets$Type").ok()?;
    if take_exception(env) {
        return None;
    }
    let mut mask: jint = 0;
    for name in ["systemBars", "displayCutout"] {
        let res = env.call_static_method(&cls, name, "()I", &[]).ok()?;
        if take_exception(env) {
            return None;
        }
        mask |= res.i().ok()?;
    }
    Some(mask)
}

/// The four insets in CSS pixels, top / bottom / left / right.
fn read_insets<'a, 'b>(
    env: &mut JNIEnv<'a>,
    activity: &JObject<'b>,
    window: &JObject<'b>,
) -> Option<(f32, f32, f32, f32)> {
    let decor = call_obj(env, window, "getDecorView", "()Landroid/view/View;")?;
    let insets = call_obj(env, &decor, "getRootWindowInsets", "()Landroid/view/WindowInsets;")?;
    if insets.is_null() {
        return None;
    }

    let raw: (i32, i32, i32, i32) = if sdk_int(env) >= 30 {
        let mask = system_bar_mask(env)?;
        let ins = call_obj_args(env, &insets, "getInsets", "(I)Landroid/graphics/Insets;", &[JValue::Int(mask)])?;
        // `getInsets` returns an immutable Insets snapshot; read its four fields.
        (
            field_i32(env, &ins, "top").unwrap_or(0),
            field_i32(env, &ins, "bottom").unwrap_or(0),
            field_i32(env, &ins, "left").unwrap_or(0),
            field_i32(env, &ins, "right").unwrap_or(0),
        )
    } else {
        (
            call_i32(env, &insets, "getSystemWindowInsetTop", "()I").unwrap_or(0),
            call_i32(env, &insets, "getSystemWindowInsetBottom", "()I").unwrap_or(0),
            call_i32(env, &insets, "getSystemWindowInsetLeft", "()I").unwrap_or(0),
            call_i32(env, &insets, "getSystemWindowInsetRight", "()I").unwrap_or(0),
        )
    };

    // Insets arrive in physical pixels; CSS wants them divided by the display density.
    let density = call_obj(env, activity, "getResources", "()Landroid/content/res/Resources;")
        .and_then(|res| call_obj(env, &res, "getDisplayMetrics", "()Landroid/util/DisplayMetrics;"))
        .and_then(|m| field_f32(env, &m, "density"))
        .filter(|d| *d > 0.0)
        .unwrap_or(1.0);
    let px = |v: i32| v as f32 / density;
    Some((px(raw.0), px(raw.1), px(raw.2), px(raw.3)))
}

/// Makes the window draw behind the system bars, leaves them transparent and points the bar icons
/// at the current theme.
///
/// `bg_argb` is Dew's background as `0xAARRGGBB`. It is used only when the platform refuses to go
/// edge-to-edge, so the strips still match the theme instead of showing the OS grey.
/// `light_theme` means a light background, which needs dark icons.
pub fn apply_system_bars(bg_argb: u32, light_theme: bool) {
    dispatch(move |env, activity, _webview| {
        let sdk = sdk_int(env);
        let Some(window) = call_obj(env, activity, "getWindow", "()Landroid/view/Window;") else {
            return;
        };

        // 1. Draw behind the bars. Android 15+ (API 35) enforces this and ignores the setters.
        let edge_to_edge = if sdk >= 30 {
            call_void(env, &window, "setDecorFitsSystemWindows", "(Z)V", &[JValue::Bool(0)])
        } else if let Some(decor) = call_obj(env, &window, "getDecorView", "()Landroid/view/View;") {
            // LAYOUT_STABLE | LAYOUT_HIDE_NAVIGATION | LAYOUT_FULLSCREEN
            let mut flags: jint = 0x100 | 0x200 | 0x400;
            // LIGHT_STATUS_BAR, then LIGHT_NAVIGATION_BAR (API 26+) for dark icons.
            if light_theme {
                flags |= 0x2000;
                if sdk >= 26 {
                    flags |= 0x10;
                }
            }
            call_void(env, &decor, "setSystemUiVisibility", "(I)V", &[JValue::Int(flags)])
        } else {
            false
        };

        // 2. Transparent bars, or the theme background when edge-to-edge was refused. Without the
        //    contrast calls below, Android draws its own scrim over three-button navigation,
        //    which is the grey strip we are here to remove.
        let colour: jint = if edge_to_edge { 0 } else { bg_argb as jint };
        call_void(env, &window, "setStatusBarColor", "(I)V", &[JValue::Int(colour)]);
        call_void(env, &window, "setNavigationBarColor", "(I)V", &[JValue::Int(colour)]);
        if sdk >= 29 {
            call_void(env, &window, "setNavigationBarContrastEnforced", "(Z)V", &[JValue::Bool(0)]);
            call_void(env, &window, "setStatusBarContrastEnforced", "(Z)V", &[JValue::Bool(0)]);
        }

        // 3. Icon colour: dark icons on a light theme, light icons on a dark one.
        if sdk >= 30 {
            if let Some(ctl) = call_obj(env, &window, "getInsetsController", "()Landroid/view/WindowInsetsController;")
            {
                // APPEARANCE_LIGHT_STATUS_BARS | APPEARANCE_LIGHT_NAVIGATION_BARS
                let mask: jint = 8 | 16;
                let appearance: jint = if light_theme { mask } else { 0 };
                call_void(env, &ctl, "setSystemBarsAppearance", "(II)V", &[JValue::Int(appearance), JValue::Int(mask)]);
            }
        }
    });
}

/// Pushes the native insets into the `--dew-inset-*` variables on the document root.
///
/// Retries while the insets are still null, because the first call can beat the first layout.
pub fn push_insets() {
    push_insets_attempt(0);
}

fn push_insets_attempt(attempt: u32) {
    dispatch(move |env, activity, webview| {
        // The webview is null until the page exists; there is nothing to inject into yet.
        if webview.is_null() {
            return;
        }
        let Some(window) = call_obj(env, activity, "getWindow", "()Landroid/view/Window;") else {
            return;
        };
        let Some((top, bottom, left, right)) = read_insets(env, activity, &window) else {
            retry(attempt);
            return;
        };

        let js = format!(
            "(function(){{var s=document.documentElement.style;\
             s.setProperty('--dew-inset-top','{top:.2}px');\
             s.setProperty('--dew-inset-bottom','{bottom:.2}px');\
             s.setProperty('--dew-inset-left','{left:.2}px');\
             s.setProperty('--dew-inset-right','{right:.2}px');}})();"
        );
        let Ok(script) = env.new_string(&js) else { return };
        let _ = env.call_method(
            webview,
            "evaluateJavascript",
            "(Ljava/lang/String;Landroid/webkit/ValueCallback;)V",
            &[JValue::Object(script.as_ref()), JValue::Object(&JObject::null())],
        );
        let _ = take_exception(env);
    });
}

/// Re-arms the inset read while the first layout is still pending, then gives up.
fn retry(attempt: u32) {
    if attempt >= INSET_RETRIES {
        return;
    }
    let next = attempt + 1;
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(RETRY_DELAY_MS));
        push_insets_attempt(next);
    });
}

/// Watches for rotation and other resizes, re-reading the insets each time.
///
/// The CSS insets go stale on rotation because the notch and home indicator move to different
/// edges, so `env()` alone is not enough on Android. Must be called once, from a component.
pub fn watch_insets() {
    spawn(async move {
        let mut ev = document::eval(
            r#"
            if (window.__dewInsetWatch) removeEventListener('resize', window.__dewInsetWatch);
            window.__dewInsetWatch = () => dioxus.send(0);
            addEventListener('resize', window.__dewInsetWatch, { passive: true });
            "#,
        );
        while ev.recv::<u8>().await.is_ok() {
            // The new size is only known after the webview has laid out again.
            retry(0);
        }
    });
}
