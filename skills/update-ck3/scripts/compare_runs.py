#!/usr/bin/env python3
"""Compare two tiger --json runs by group: (key, area, normalized message).

Usage: python3 compare_runs.py OLD.json NEW.json [--min N] [--area PREFIX]
Prints the groups whose counts changed, biggest change first.
"""
import collections, json, re, signal, sys

signal.signal(signal.SIGPIPE, signal.SIG_DFL)

def norm(m):
    m = re.sub(r"`[^`]*`", "`X`", m)
    m = re.sub(r"^(.*?) (\S+) not defined in (\S+)$", r"\1 X not defined in \3", m)
    m = re.sub(r"localization key \S+$", "localization key X", m)
    m = re.sub(r"^missing [a-z_, ]+ localization", "missing <langs> localization", m)
    m = re.sub(r"\b\d+(\.\d+)?\b", "N", m)
    return m

def area(r):
    p = r["locations"][0]["path"].split("/") if r.get("locations") else ["?"]
    return "/".join(p[:3] if p[0] == "common" else p[:2])

def groups(path):
    c = collections.Counter()
    for r in json.load(open(path, encoding="utf-8")):
        c[(r["key"], area(r), norm(r["message"]))] += 1
    return c

if __name__ == "__main__":
    args = sys.argv[1:]
    minimum = int(args[args.index("--min") + 1]) if "--min" in args else 1
    prefix = args[args.index("--area") + 1] if "--area" in args else ""
    old, new = groups(args[0]), groups(args[1])
    print(f"total {sum(old.values())} -> {sum(new.values())}")
    rows = [(new[g] - old[g], old[g], new[g], g) for g in old.keys() | new.keys()
            if old[g] != new[g] and g[1].startswith(prefix)]
    for d, o, n, (key, ar, msg) in sorted(rows, key=lambda r: -abs(r[0])):
        if abs(d) >= minimum:
            print(f"{d:+8} {o:7} -> {n:<7} {key:22} {ar:32} {msg[:120]}")
