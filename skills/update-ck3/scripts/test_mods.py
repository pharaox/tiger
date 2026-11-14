#!/usr/bin/env python3
"""Run the saved test mods and compare their reports with their expected.txt.

Usage: python3 test_mods.py TIGER GAME_DIR [--write] [MOD_DIR...]
A test mod is a folder with a descriptor.mod, the test files and an expected.txt. Without
MOD_DIR, runs every folder in .claude/test-mods. Tiger runs single-threaded with an empty config.
--write saves the reports as expected.txt instead of comparing.
"""
import difflib, json, os, signal, subprocess, sys, tempfile

signal.signal(signal.SIGPIPE, signal.SIG_DFL)

TEST_MODS = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), "../../../test-mods"))

def fmt(loc):
    pos = f":{loc['linenr']}:{loc['column']}" if loc.get("linenr") else ""
    return loc["path"] + pos

def reports(tiger, game, mod):
    with tempfile.NamedTemporaryFile(suffix=".conf") as conf:
        env = dict(os.environ, RAYON_NUM_THREADS="1")
        cmd = [tiger, "--game", game, "--config", conf.name, "--json", os.path.join(mod, "descriptor.mod")]
        out = subprocess.run(cmd, capture_output=True, text=True, env=env, check=True).stdout
    lines = []
    for r in json.loads(out):
        locs = r.get("locations") or [{"path": "?"}]
        text = f"{r['severity']} {r['key']} {fmt(locs[0])}: {r['message']}"
        for loc in locs[1:]:
            text += f"\n    {fmt(loc)}" + (f" ({loc['tag']})" if loc.get("tag") else "")
        lines.append(text)
    return "".join(line + "\n" for line in sorted(lines))

if __name__ == "__main__":
    args = sys.argv[1:]
    write = "--write" in args
    args = [a for a in args if a != "--write"]
    if len(args) < 2:
        sys.exit(__doc__)
    tiger, game, mods = args[0], args[1], args[2:]
    if not mods:
        mods = sorted(os.path.join(TEST_MODS, d) for d in os.listdir(TEST_MODS)
                      if os.path.isfile(os.path.join(TEST_MODS, d, "descriptor.mod")))
    failed = 0
    for mod in mods:
        got = reports(tiger, game, mod)
        path = os.path.join(mod, "expected.txt")
        if write:
            open(path, "w", encoding="utf-8").write(got)
            print(f"wrote {path} ({len(got.splitlines())} lines)")
            continue
        want = open(path, encoding="utf-8").read() if os.path.exists(path) else ""
        if got == want:
            print(f"ok    {os.path.basename(mod)}")
        else:
            failed += 1
            print(f"DIFF  {os.path.basename(mod)}")
            sys.stdout.writelines(difflib.unified_diff(want.splitlines(True), got.splitlines(True),
                                                       "expected.txt", "now", n=0))
    sys.exit(1 if failed else 0)
