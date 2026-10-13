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
use dioxus_icons::lucide::{BookOpen, ChevronDown, ChevronRight, ChevronUp, Copy, X};

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

/// A paragraph of manual text with tappable references.
///
/// Renders as a block (`div`) so an inline expansion always opens *below* the paragraph at full
/// width, whatever layout the caller sits in. `prefix` (bold) and `suffix` (secondary) sit on the
/// same line as the text, e.g. "Faith — …" or "Hebrews 11:1 (KJV)".
#[component]
pub fn ScriptureText(
    text: String,
    #[props(default)] class: String,
    #[props(default)] prefix: Option<String>,
    #[props(default)] suffix: Option<String>,
) -> Element {
    let app = use_app();
    let mode = app.settings.read().link_mode;
    let preview = try_use_context::<Preview>();
    let lesson = try_use_context::<LessonContext>();
    let mut inline: Signal<Option<Reference>> = use_signal(|| None);
    let segments = segment(&text);

    rsx! {
        div { class: "min-w-0 {class}",
        p {
            if let Some(p) = &prefix {
                span { class: "font-semibold", "{p} — " }
            }
            for (i, seg) in segments.into_iter().enumerate() {
                match seg {
                    Segment::Text(t) => rsx! { span { key: "{i}", "{t}" } },
                    Segment::Ref { text, reference } => {
                        let is_open = inline.read().as_ref() == Some(&reference);
                        let lesson = lesson.clone();
                        rsx! {
                            button {
                                key: "{i}",
                                class: if is_open { "press inline whitespace-nowrap rounded-[6px] bg-accent/12 px-1 font-semibold text-accent" } else { "press inline whitespace-nowrap rounded-[6px] px-0.5 font-semibold text-accent decoration-accent/40 underline-offset-[3px] hover:underline" },
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
                                    ChevronDown { size: 13, stroke_width: "2.8", class: if is_open { "ml-0.5 inline -mt-0.5 rotate-180 transition-transform" } else { "ml-0.5 inline -mt-0.5 transition-transform" } }
                                }
                            }
                        }
                    }
                }
            }
            if let Some(sfx) = &suffix {
                span { class: "text-label-2", " {sfx}" }
            }
        }
        if let Some(r) = inline.read().clone() {
            InlinePassage { key: "{r.label()}", reference: r, on_close: move |_| inline.set(None) }
        }
        }
    }
}

/// The "drop-down" under a paragraph: a full-width card in the reader's serif, long passages
/// folded to the first few verses with "Show All".
#[component]
fn InlinePassage(reference: Reference, on_close: EventHandler<()>) -> Element {
    let app = use_app();
    let lesson = try_use_context::<LessonContext>();
    let r2 = reference.clone();
    let label = reference.label();
    let abbr = translation(&app.settings.read().translation).abbreviation;
    rsx! {
        div { class: "inline-passage fade-in mt-2.5 mb-3 rounded-2xl bg-fill px-4 pt-3 pb-1 text-left font-normal not-italic text-label",
            div { class: "mb-1 flex items-baseline justify-between gap-3",
                p { class: "text-[15px] font-semibold", "{label}" }
                span { class: "text-[12px] font-bold tracking-wide text-label-2", "{abbr}" }
            }
            PassageText { reference, compact: true, fold: true }
            div { class: "-mx-1 mt-1 flex items-center justify-between border-t border-separator",
                button {
                    class: "press flex h-11 items-center gap-1 px-1 text-[15px] font-medium text-accent",
                    onclick: move |_| open_in_bible(app, lesson.clone(), &r2),
                    "Open in Bible"
                    ChevronRight { size: 16, stroke_width: "2.4" }
                }
                button {
                    class: "press flex h-11 items-center gap-1 px-1 text-[15px] text-label-2",
                    onclick: move |_| on_close.call(()),
                    "Hide"
                    ChevronUp { size: 16, stroke_width: "2.4" }
                }
            }
        }
    }
}

/// Loads and renders the verses of a reference in the reader's serif style.
#[component]
pub fn PassageText(reference: Reference, #[props(default)] compact: bool, #[props(default)] fold: bool) -> Element {
    let app = use_app();
    let mut show_all = use_signal(|| false);
    let reference_sig = use_signal(|| reference.clone());
    let verses = use_resource(move || {
        let tr = app.settings.read().translation.clone();
        let r = reference_sig.read().clone();
        async move { passage_verses(&tr, &r).await }
    });
    let size = if compact { "text-[16px] leading-[1.7] text-label" } else { "reader-text" };
    let show_numbers = app.settings.read().verse_numbers;

    rsx! {
        match &*verses.read() {
            None => rsx! { p { class: "animate-pulse text-label-3", "Loading…" } },
            Some(Err(e)) => rsx! { p { class: "text-[15px] text-label-2", "{e}" } },
            Some(Ok(list)) => {
                let multi_chapter = list.first().map(|f| f.chapter) != list.last().map(|l| l.chapter);
                const FOLD_AT: usize = 4;
                let folded = fold && !show_all() && list.len() > FOLD_AT + 1;
                let shown = if folded { &list[..FOLD_AT] } else { &list[..] };
                let total = list.len();
                rsx! {
                    p { class: "font-serif {size}",
                        for v in shown.iter() {
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
                        if folded { span { class: "text-label-3", "…" } }
                    }
                    if folded {
                        button {
                            class: "press mt-1 mb-1 text-[15px] font-medium text-accent",
                            onclick: move |_| show_all.set(true),
                            "Show all {total} verses"
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
