#!/usr/bin/env python3
"""Survey the top-level fields of the items in PDX script files.

Usage: python3 toplevel.py FILE... [--field NAME]
Without --field, prints how many items use each top-level field.
With --field, prints the values of that field, with counts.

As a library: items(path) returns [(key, value)] for the top level of a file, where a
value is a string or a nested [(key, value)] list (key None for loose values and blocks).
Much faster than nested regexes on large files.
"""
import collections, re, signal, sys

signal.signal(signal.SIGPIPE, signal.SIG_DFL)

TOK = re.compile(r'"[^"]*"|[{}]|[=<>!?]+|[^\s{}=<>!?]+')

def items(path):
    text = re.sub(r"#.*", "", open(path, encoding="utf-8-sig").read())
    toks = TOK.findall(text)
    pos = 0

    def block():
        nonlocal pos
        out = []
        while pos < len(toks) and toks[pos] != "}":
            tok = toks[pos]
            pos += 1
            if tok == "{":
                out.append((None, block()))
                pos += 1
            elif pos < len(toks) and toks[pos] in ("=", "?=", "<", ">", "<=", ">=", "!="):
                pos += 1
                if pos < len(toks) and toks[pos] == "{":
                    pos += 1
                    value = block()
                    pos += 1
                else:
                    value = toks[pos] if pos < len(toks) else ""
                    pos += 1
                out.append((tok, value))
            else:
                out.append((None, tok))
        return out

    return block()

def main():
    args = sys.argv[1:]
    field = None
    if "--field" in args:
        i = args.index("--field")
        field = args[i + 1]
        del args[i : i + 2]
    counts = collections.Counter()
    for path in args:
        for _, body in items(path):
            if not isinstance(body, list):
                continue
            if field:
                for k, v in body:
                    if k == field:
                        counts[v if isinstance(v, str) else "{...}"] += 1
            else:
                counts["(items)"] += 1
                counts.update({k for k, _ in body if k})
    for name, n in counts.most_common():
        print(n, name)

if __name__ == "__main__":
    main()
