#!/usr/bin/env python3
"""Compare the defines of two game versions with tiger's defines table.

Usage: python3 defines_diff.py OLD_ROOT NEW_ROOT [TABLE]
A root is a directory with `game/` and `jomini/` in it (the game install, or a frozen copy).
TABLE defaults to src/ck3/tables/defines.rs.
Prints the defines added and removed between the versions, and how they relate to the table.
"""
import glob, re, sys


def block_end(text, i):
    """Return the index just after the `}` that closes the block whose body starts at `i`."""
    depth = 1
    while depth:
        depth += text[i] == "{"
        depth -= text[i] == "}"
        i += 1
    return i


def parse(root):
    """Return {"NGroup|KEY": value} for all defines under the root."""
    out = {}
    files = glob.glob(f"{root}/game/common/defines/**/*.txt", recursive=True)
    files += glob.glob(f"{root}/jomini/common/defines/**/*.txt", recursive=True)
    for f in files:
        text = re.sub(r"#[^\n]*", "", open(f, encoding="utf-8-sig", errors="replace").read())
        for group in re.finditer(r"\b(N[A-Za-z0-9_]+)\s*=\s*\{", text):
            body = text[group.end() : block_end(text, group.end()) - 1]
            depth = 0
            for m in re.finditer(r"\{|\}|([A-Za-z_0-9]+)\s*=\s*(\{|\"[^\"]*\"|[^\s{}]+)", body):
                if m.group(0) == "{":
                    depth += 1
                elif m.group(0) == "}":
                    depth -= 1
                elif depth == 0 and m.group(1):
                    value = m.group(2)
                    if value == "{":
                        depth += 1
                        value = "{" + body[m.end() : block_end(body, m.end()) - 1].strip() + "}"
                    out[f"{group.group(1)}|{m.group(1)}"] = value
    return out


def table(path):
    """Return {"NGroup|KEY": type} from the table, including entries that rustfmt split over lines."""
    text = open(path, encoding="utf-8").read()
    return dict(re.findall(r'\(\s*"(N[^"]+)",\s*(DefineType::\w+(?:\([^)]*\))?)\s*,?\s*\)', text))


if __name__ == "__main__":
    old, new = parse(sys.argv[1]), parse(sys.argv[2])
    tab = table(sys.argv[3] if len(sys.argv) > 3 else "src/ck3/tables/defines.rs")
    added = sorted(set(new) - set(old))
    removed = sorted(set(old) - set(new))
    print(f"old {len(old)}, new {len(new)}, table {len(tab)}")
    print(f"\n## added ({len(added)}; not in table: {sum(k not in tab for k in added)})")
    for k in added:
        print(f"{k} = {new[k][:60]}{'' if k not in tab else '   [in table: ' + tab[k] + ']'}")
    print(f"\n## removed ({len(removed)}; still in table: {sum(k in tab for k in removed)})")
    for k in removed:
        print(f"{k}{'   [in table]' if k in tab else ''}")
    stale = sorted(k for k in tab if k not in new and k not in old)
    print(f"\n## in table, in neither version ({len(stale)})")
    print("\n".join(stale))
