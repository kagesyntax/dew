//! Scripture-aware text. Any reference inside manual text ("Read Heb 11:1-3,6") becomes a
//! tappable link. What a tap does follows Settings → Scripture Links:
//! * Preview: a bottom sheet with the verses and "Open in Bible"
//! * Inline: the verses expand right under the paragraph (the "drop-down")
//! * Open Bible: jump to the reader, with a pill that returns to the exact lesson position

use crate::bible::{passage_verses, translation, PassageVerse};
use crate::components::sheet::{Sheet, SheetTitle};
use crate::platform::js;
use crate::routes::Route;
use crate::state::{navigation, settings::LinkMode, use_app, AppState};
use church_core::scripture::{segment, Reference, Segment};
use dioxus::prelude::*;
use dioxus_icons::lucide::{BookOpen, ChevronDown, Copy, X};

/// Present in a lesson so links know where "Back to …" should return.
#[derive(Clone, PartialEq)]
pub struct LessonContext {
    pub label: String,
    pub route: Route,
}

/// The single preview sheet for the app; any `ScriptureText` can open it.
#[derive(Clone, Copy)]
pub struct Preview(pub Signal<Option<Reference>>, pub Signal<Option<LessonContext>>);

#[component]
pub fn PreviewHost(children: Element) -> Element {
    let preview = use_context_provider(|| Preview(Signal::new(None), Signal::new(None)));
    rsx! {
        {children}
        PassageSheet { target: preview.0, origin: preview.1 }
    }
}

fn open_in_bible(app: AppState, lesson: Option<LessonContext>, reference: &Reference) {
    let target = Route::reader_focus(reference.first());
    match lesson {
        Some(ctx) => navigation::jump_with_return(app, ctx.label, ctx.route, target),
        None => {
            navigator().push(target);
        }
    }
}

#[component]
pub fn ScriptureText(text: String, #[props(default)] class: String) -> Element {
    let app = use_app();
    let mode = app.settings.read().link_mode;
    let preview = try_use_context::<Preview>();
    let lesson = try_use_context::<LessonContext>();
    let mut inline: Signal<Option<Reference>> = use_signal(|| None);
    let segments = segment(&text);

    rsx! {
        p { class: "{class}",
            for (i, seg) in segments.into_iter().enumerate() {
                match seg {
                    Segment::Text(t) => rsx! { span { key: "{i}", "{t}" } },
                    Segment::Ref { text, reference } => {
                        let is_open = inline.read().as_ref() == Some(&reference);
                        let lesson = lesson.clone();
                        rsx! {
                            button {
                                key: "{i}",
                                class: "press inline rounded-[5px] px-0.5 font-semibold text-accent decoration-accent/40 underline-offset-[3px] hover:underline",
                                "aria-expanded": if mode == LinkMode::Inline { if is_open { "true" } else { "false" } } else { "" },
                                onclick: move |_| {
                                    js::haptic();
                                    match mode {
                                        LinkMode::Sheet => match preview {
                                            Some(mut p) => {
                                                p.1.set(lesson.clone());
                                                p.0.set(Some(reference.clone()));
                                            }
                                            None => open_in_bible(app, lesson.clone(), &reference),
                                        },
                                        LinkMode::Inline => {
                                            let next = if inline.peek().as_ref() == Some(&reference) { None } else { Some(reference.clone()) };
                                            inline.set(next);
                                        }
                                        LinkMode::Navigate => open_in_bible(app, lesson.clone(), &reference),
                                    }
                                },
                                "{text}"
                                if mode == LinkMode::Inline {
                                    ChevronDown { size: 14, stroke_width: "2.5", class: if is_open { "ml-0.5 inline rotate-180 transition-transform" } else { "ml-0.5 inline transition-transform" } }
                                }
                            }
                        }
                    }
                }
            }
        }
        if let Some(r) = inline.read().clone() {
            InlinePassage { key: "{r.label()}", reference: r }
        }
    }
}

/// The "drop-down" under a paragraph.
#[component]
fn InlinePassage(reference: Reference) -> Element {
    let app = use_app();
    let lesson = try_use_context::<LessonContext>();
    let r2 = reference.clone();
    rsx! {
        div { class: "fade-in my-3 rounded-xl border-l-[3px] border-accent bg-surface-2 py-3 pr-3 pl-4",
            div { class: "mb-1.5 flex items-center justify-between gap-2",
                span { class: "text-[13px] font-semibold uppercase tracking-wide text-label-2", "{reference.label()}" }
                button {
                    class: "press flex items-center gap-1 text-[13px] font-semibold text-accent",
                    onclick: move |_| open_in_bible(app, lesson.clone(), &r2),
                    BookOpen { size: 14, stroke_width: "2.3" }
                    "Open"
                }
            }
            PassageText { reference, compact: true }
        }
    }
}

/// Loads and renders the verses of a reference in the reader's serif style.
#[component]
pub fn PassageText(reference: Reference, #[props(default)] compact: bool) -> Element {
    let app = use_app();
    let reference_sig = use_signal(|| reference.clone());
    let verses = use_resource(move || {
        let tr = app.settings.read().translation.clone();
        let r = reference_sig.read().clone();
        async move { passage_verses(&tr, &r).await }
    });
    let size = if compact { "text-[16px] leading-relaxed" } else { "reader-text" };
    let show_numbers = app.settings.read().verse_numbers;

    rsx! {
        match &*verses.read() {
            None => rsx! { p { class: "animate-pulse text-label-3", "Loading…" } },
            Some(Err(e)) => rsx! { p { class: "text-[15px] text-label-2", "{e}" } },
            Some(Ok(list)) => {
                let multi_chapter = list.first().map(|f| f.chapter) != list.last().map(|l| l.chapter);
                rsx! {
                    p { class: "font-serif {size}",
                        for v in list.iter() {
                            span { key: "{v.chapter}-{v.verse}",
                                if show_numbers {
                                    sup { class: "mr-0.5 font-sans text-[0.62em] font-semibold text-label-2",
                                        if multi_chapter && v.verse == 1 { "{v.chapter}:" }
                                        "{v.verse}"
                                    }
                                }
                                "{v.text} "
                            }
                        }
                    }
                }
            }
        }
    }
}

fn plain_text(reference: &Reference, verses: &[PassageVerse], tr: &str) -> String {
    let body = verses.iter().map(|v| v.text.as_str()).collect::<Vec<_>>().join(" ");
    format!("{} — {} ({})", body, reference.label(), translation(tr).abbreviation)
}

#[component]
fn PassageSheet(target: Signal<Option<Reference>>, origin: Signal<Option<LessonContext>>) -> Element {
    let app = use_app();
    let open = target.read().is_some();
    let current = target.read().clone();
    let tr = app.settings.read().translation.clone();
    let abbr = translation(&tr).abbreviation;

    rsx! {
        Sheet {
            open,
            on_open_change: move |v: bool| if !v { target.set(None) },
            "data-side": "bottom",
            if let Some(r) = current {
                div { class: "flex items-start justify-between gap-3 px-5 pt-1",
                    div {
                        SheetTitle { class: "text-[22px] font-bold tracking-tight", "{r.label()}" }
                        p { class: "text-[13px] font-medium text-label-2", "{translation(&tr).name}" }
                    }
                    button {
                        class: "press rounded-full bg-fill p-1.5 text-label-2",
                        aria_label: "Close",
                        onclick: move |_| target.set(None),
                        X { size: 18, stroke_width: "2.6" }
                    }
                }
                div { class: "px-5", PassageText { key: "{r.label()}-{tr}", reference: r.clone() } }
                div { class: "grid grid-cols-[1fr_auto] gap-2 px-5 pt-1",
                    button {
                        class: "press flex h-12 items-center justify-center gap-2 rounded-xl bg-accent text-[17px] font-semibold text-white",
                        onclick: {
                            let r = r.clone();
                            move |_| {
                                target.set(None);
                                open_in_bible(app, origin.peek().clone(), &r);
                            }
                        },
                        BookOpen { size: 19, stroke_width: "2.3" }
                        "Open in Bible"
                    }
                    button {
                        class: "press flex h-12 w-12 items-center justify-center rounded-xl bg-fill text-accent",
                        aria_label: "Copy {abbr}",
                        onclick: {
                            let r = r.clone();
                            let tr = tr.clone();
                            move |_| {
                                let r = r.clone();
                                let tr = tr.clone();
                                spawn(async move {
                                    if let Ok(v) = passage_verses(&tr, &r).await {
                                        js::copy_text(&plain_text(&r, &v, &tr));
                                    }
                                });
                            }
                        },
                        Copy { size: 19, stroke_width: "2.2" }
                    }
                }
            }
        }
    }
}
