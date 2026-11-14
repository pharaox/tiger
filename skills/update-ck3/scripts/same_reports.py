#!/usr/bin/env python3
"""Compare the exact reports of two tiger --json runs.

Usage: python3 same_reports.py OLD.json NEW.json [--list N]
Prints how many reports differ, by key, message and locations, and in full (including severity
and info), and whether the order is the same. With --list, prints the N largest groups of
differing reports on each side. Use it to show that a refactor didn't change the reports;
compare_runs.py compares groups instead.
"""
import collections, json, signal, sys

signal.signal(signal.SIGPIPE, signal.SIG_DFL)

def ident(r):
    locs = [(l["path"], l["linenr"], l.get("column")) for l in r.get("locations", [])]
    return json.dumps([r["key"], r["message"], locs])

def full(r):
    return json.dumps(r, sort_keys=True)

def area(r):
    p = r["locations"][0]["path"].split("/") if r.get("locations") else ["?"]
    return "/".join(p[:3] if p[0] == "common" else p[:2])

if __name__ == "__main__":
    args = sys.argv[1:]
    n = 0
    if "--list" in args:
        i = args.index("--list")
        n = int(args[i + 1])
        del args[i:i + 2]
    if len(args) != 2:
        sys.exit(__doc__)
    old, new = (json.load(open(p, encoding="utf-8")) for p in args)
    a, b = collections.Counter(map(ident, old)), collections.Counter(map(ident, new))
    fa, fb = collections.Counter(map(full, old)), collections.Counter(map(full, new))
    print(f"{len(old)} -> {len(new)} reports")
    print(f"differing by key, message and locations: {sum((a - b).values())} only in old, "
          f"{sum((b - a).values())} only in new")
    print(f"differing in full: {sum((fa - fb).values())} only in old, {sum((fb - fa).values())} only in new")
    print(f"same order: {list(map(full, old)) == list(map(full, new))}")
    if n:
        for name, c in (("only in old", fa - fb), ("only in new", fb - fa)):
            groups = collections.Counter()
            for k, count in c.items():
                r = json.loads(k)
                groups[(r["severity"], r["key"], area(r), r["message"][:100])] += count
            print(name)
            for g, count in groups.most_common(n):
                print(f"  {count:6}  {' | '.join(g)}")
