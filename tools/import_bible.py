#!/usr/bin/env python3
"""Convert a full-Bible JSON (scrollmapper/bible_databases or midvash/bible-data format) into
per-book asset files.

Output layout (consumed by src/bible/loader.rs):
  assets/bible/<code>/<book_id>.json   -> [[verse, ...], ...]  (chapters -> verses)

Usage: python3 tools/import_bible.py KJV.json kjv
Only public-domain translations may be bundled (KJV, ASV, BSB, WEB...).
"""
import json, sys, pathlib

BOOK_IDS = ["GEN","EXO","LEV","NUM","DEU","JOS","JDG","RUT","1SA","2SA","1KI","2KI","1CH","2CH","EZR","NEH","EST","JOB","PSA","PRO","ECC","SNG","ISA","JER","LAM","EZK","DAN","HOS","JOL","AMO","OBA","JON","MIC","NAM","HAB","ZEP","HAG","ZEC","MAL","MAT","MRK","LUK","JHN","ACT","ROM","1CO","2CO","GAL","EPH","PHP","COL","1TH","2TH","1TI","2TI","TIT","PHM","HEB","JAS","1PE","2PE","1JN","2JN","3JN","JUD","REV"]

def main(src, code):
    data = json.load(open(src, encoding="utf-8"))
    books = data["books"]
    assert len(books) == 66, f"expected 66 books, got {len(books)}"
    out = pathlib.Path(__file__).resolve().parent.parent / "assets" / "bible" / code
    out.mkdir(parents=True, exist_ok=True)
    for bid, book in zip(BOOK_IDS, books):
        chapters = []
        for ch in sorted(book["chapters"], key=lambda c: c["chapter"]):
            verses = [v["text"].strip() for v in sorted(ch["verses"], key=lambda v: v.get("verse", v.get("number")))]
            chapters.append(verses)
        (out / f"{bid}.json").write_text(json.dumps(chapters, ensure_ascii=False, separators=(",", ":")), encoding="utf-8")
    print(f"wrote {len(books)} books to {out}")

if __name__ == "__main__":
    main(sys.argv[1], sys.argv[2])
