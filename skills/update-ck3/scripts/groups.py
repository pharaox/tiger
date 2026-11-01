"""Summarize a tiger --json report file into groups of (severity, key, normalized message)."""
import collections, json, re, sys

def norm(msg):
    msg = re.sub(r"`[^`]*`", "`X`", msg)
    msg = re.sub(r"\b\d+(\.\d+)?\b", "N", msg)
    return msg

def groups(path):
    c = collections.Counter()
    for r in json.load(open(path, encoding="utf-8")):
        c[(r["severity"], r["key"], norm(r["message"]))] += 1
    return c

if __name__ == "__main__":
    src, out = sys.argv[1], sys.argv[2]
    c = groups(src)
    by_key = collections.Counter()
    for (sev, key, _), n in c.items():
        by_key[(sev, key)] += n
    with open(out, "w", encoding="utf-8") as f:
        f.write(f"total reports: {sum(c.values())}, groups: {len(c)}\n\n## by severity/key\n")
        for (sev, key), n in by_key.most_common():
            f.write(f"{n:7}  {sev:8} {key}\n")
        f.write("\n## by group\n")
        for (sev, key, msg), n in c.most_common():
            f.write(f"{n:7}  {sev:8} {key:22} {msg}\n")
