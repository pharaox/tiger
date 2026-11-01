"""Attribute each changed group to vanilla or to a tiger step, from the runs made along the way.

Usage: python3 timeline.py V_old.json V_new0.json STEP1.json ... CURRENT.json [--up|--down] [--key REGEX]
Groups are (key, normalized message) across areas, as in compare_runs.py. For each group whose count
differs between V_old and CURRENT, prints the counts in V_old, V_new0 and CURRENT, and the steps
where the count changed. "vanilla" means no step changed it and V_new0 already had the CURRENT
count: the old tiger reports the same on the new vanilla, so vanilla caused the change.
"""
import collections, json, re, signal, sys
from compare_runs import norm

signal.signal(signal.SIGPIPE, signal.SIG_DFL)

def groups(path):
    c = collections.Counter()
    for r in json.load(open(path, encoding="utf-8")):
        c[(r["key"], norm(r["message"]))] += 1
    return c

if __name__ == "__main__":
    args = sys.argv[1:]
    key = args.pop(args.index("--key") + 1) if "--key" in args else ""
    direction = "up" if "--up" in args else "down" if "--down" in args else ""
    paths = [a for a in args if not a.startswith("--") and a != key]
    runs = [(p, groups(p)) for p in paths]
    old, new0, cur = runs[0][1], runs[1][1], runs[-1][1]
    rows = []
    for g in old.keys() | cur.keys():
        o, n = old[g], cur[g]
        if o == n or not re.search(key, g[0]) or (direction == "up" and n < o) or (direction == "down" and n > o):
            continue
        steps = [f"{p.rsplit('/', 1)[-1].removesuffix('.json')}:{c[g]}"
                 for (_, prev), (p, c) in zip(runs[1:], runs[2:]) if c[g] != prev[g]]
        who = "vanilla" if not steps and new0[g] == n else "tiger"
        rows.append((n - o, o, new0[g], n, who, steps, g))
    for d, o, v, n, who, steps, (k, msg) in sorted(rows, key=lambda r: -abs(r[0])):
        print(f"{d:+7} {o:6} {v:6} {n:6} {who:7} {k:20} {' '.join(steps)[:60]:60} {msg[:100]}")
