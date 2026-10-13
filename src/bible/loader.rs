//! Loads one book of one translation from the bundled assets (`assets/bible/<tr>/<BOOK>.json`)
//! and caches it in memory. Uses Dioxus' cross-platform asset resolver, so the same code reads
//! over HTTP on the web and from the bundle on desktop/mobile.

use church_core::scripture::{books, Reference};
use dioxus::prelude::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

static BIBLE_DIR: Asset = asset!("/assets/bible", AssetOptions::folder());

/// chapters -> verses (0-based vectors; verse n is at index n-1)
pub type BookText = Rc<Vec<Vec<String>>>;

thread_local! {
    static CACHE: RefCell<HashMap<(String, String), BookText>> = RefCell::new(HashMap::new());
}

pub async fn load_book(translation: &str, book_id: &str) -> Result<BookText, String> {
    let key = (translation.to_string(), book_id.to_string());
    if let Some(hit) = CACHE.with(|c| c.borrow().get(&key).cloned()) {
        return Ok(hit);
    }
    let path = format!("{BIBLE_DIR}/{translation}/{book_id}.json");
    let bytes =
        dioxus::asset_resolver::read_asset_bytes(&path).await.map_err(|e| format!("Could not load {book_id}: {e}"))?;
    let parsed: Vec<Vec<String>> =
        serde_json::from_slice(&bytes).map_err(|e| format!("Corrupt data for {book_id}: {e}"))?;
    let text = Rc::new(parsed);
    CACHE.with(|c| c.borrow_mut().insert(key, text.clone()));
    Ok(text)
}

#[derive(Debug, Clone, PartialEq)]
pub struct PassageVerse {
    pub chapter: u16,
    pub verse: u16,
    pub text: String,
}

/// Resolve every verse of a (possibly multi-part) reference.
pub async fn passage_verses(translation: &str, reference: &Reference) -> Result<Vec<PassageVerse>, String> {
    let mut out = Vec::new();
    for p in &reference.passages {
        let book = books::by_id(p.book.id).ok_or("Unknown book")?;
        let text = load_book(translation, book.id).await?;
        for ch in p.chapter..=p.end_chapter {
            let Some(verses) = text.get(ch as usize - 1) else { continue };
            for (i, t) in verses.iter().enumerate() {
                let v = i as u16 + 1;
                if p.contains(ch, v) {
                    out.push(PassageVerse { chapter: ch, verse: v, text: t.clone() });
                }
            }
        }
    }
    Ok(out)
}
