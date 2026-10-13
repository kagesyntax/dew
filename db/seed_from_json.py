#!/usr/bin/env python3
"""Generate INSERT statements for Turso from a manual JSON file.

Usage: python3 db/seed_from_json.py assets/manual/manual.json | turso db shell <db-name>
"""
import json, sys

def q(s):
    return "'" + str(s).replace("'", "''") + "'"

manual = json.load(open(sys.argv[1], encoding="utf-8"))
for l in manual["lessons"]:
    print(
        "INSERT OR REPLACE INTO lessons (id, unit_no, study_no, lesson_date, title, payload, published) VALUES "
        f"({q(l['id'])}, {int(l['unit_number'])}, {int(l['study_number'])}, {q(l['date'])}, {q(l['title'])}, {q(json.dumps(l, ensure_ascii=False))}, 1);"
    )
