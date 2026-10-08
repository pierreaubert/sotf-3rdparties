#!/usr/bin/env python3
"""Migrate a consumer repo from `xxx` to `sotf-xxx` collection deps (path flow).

Usage:
  migrate-consumer.py [--apply] --collection DIR CONSUMER_REPO

Default is --check (report only); --apply rewrites the consumer manifests.

Path flow (this script's default): every dependency on a collection crate
becomes a direct `path` dependency with `package = "sotf-..."` (the dependency
key is preserved, so no Rust source changes are needed). `[patch]` entries
pointing into the collection become dead in a path-based graph, so they are
removed; entries already keyed by the new `sotf-...` name (registry-flow local
overrides) are kept.

Rules:
- `foo = { path = "../sotf-3rdparties/foo" }` gains `package = "sotf-foo"`
  (keys stay, so `use foo::` keeps working) and a `version` when missing.
- A registry dep on an old collection name that has a matching `[patch]`
  entry (proof the fork was intended) is converted to a path dep and the
  patch entry is removed.
- A registry dep on an old collection name WITHOUT a patch entry is left
  alone and reported for human review (it may intentionally use upstream,
  e.g. the objc/block/async-task crates that must stay upstream until their
  own dependents are forked).
- `[patch]` entries with old-name keys pointing into the collection are
  removed (post-rename they would be name-mismatch errors). Entries that only
  redirected transitive dependencies are flagged so the loss of upstream
  unification can be reviewed.
"""
import os
import re
import sys
import tomllib
from pathlib import Path

HEADER_RE = re.compile(r"^(\s*)(\[{1,2})([^\]]+)(\]{1,2})\s*(?:#.*)?$")


def split_header(inner):
    parts, cur, quote = [], "", None
    i = 0
    while i < len(inner):
        ch = inner[i]
        if quote:
            if ch == "\\" and quote == '"' and i + 1 < len(inner):
                cur += inner[i + 1]
                i += 2
                continue
            if ch == quote:
                quote = None
            else:
                cur += ch
        elif ch in "\"'":
            quote = ch
        elif ch == ".":
            parts.append(cur.strip())
            cur = ""
        else:
            cur += ch
        i += 1
    parts.append(cur.strip())
    return parts


def parse_headers(lines):
    out = []
    for i, line in enumerate(lines):
        m = HEADER_RE.match(line.rstrip("\n"))
        if m:
            out.append((i, "array" if m.group(2) == "[[" else "table", split_header(m.group(3))))
    return out


def span_of(headers, hpos, nlines):
    start = headers[hpos][0]
    end = headers[hpos + 1][0] if hpos + 1 < len(headers) else nlines
    return start, end


def dep_tables(data):
    out = []
    for sec in ("dependencies", "dev-dependencies", "build-dependencies"):
        deps = data.get(sec, {})
        if isinstance(deps, dict):
            for k, v in deps.items():
                out.append(([sec], k, v))
    ws = data.get("workspace", {}).get("dependencies", {})
    if isinstance(ws, dict):
        for k, v in ws.items():
            out.append((["workspace", "dependencies"], k, v))
    targets = data.get("target", {})
    if isinstance(targets, dict):
        for cfg, t in targets.items():
            if isinstance(t, dict):
                for sec in ("dependencies", "dev-dependencies", "build-dependencies"):
                    deps = t.get(sec, {})
                    if isinstance(deps, dict):
                        for k, v in deps.items():
                            out.append((["target", cfg, sec], k, v))
    return out


def brace_end(lines, si, col):
    depth, instr, i, j = 0, None, si, col
    while True:
        line = lines[i]
        k = j
        while k < len(line):
            ch = line[k]
            if instr:
                if ch == "\\" and instr == '"':
                    k += 1
                elif ch == instr:
                    instr = None
            elif ch in "\"'":
                instr = ch
            elif ch == "#":
                break
            elif ch == "{":
                depth += 1
            elif ch == "}":
                depth -= 1
                if depth == 0:
                    return i
            k += 1
        i += 1
        j = 0


def main():
    args = sys.argv[1:]
    apply = "--apply" in args
    args = [a for a in args if a != "--apply"]
    if "--collection" not in args or len(args) < 3:
        raise SystemExit("usage: migrate-consumer.py [--apply] --collection DIR CONSUMER_REPO")
    ci = args.index("--collection")
    collection = Path(args[ci + 1]).resolve()
    consumer = Path(args[-1]).resolve()
    assert (collection / "FORKS.json").is_file(), f"not a collection: {collection}"

    # index collection: dir -> (new_name, version); oldname -> (dir, new, ver)
    col, oldmap = {}, {}
    for p in sorted(collection.rglob("Cargo.toml")):
        if "target/" in p.parts:
            continue
        d = tomllib.loads(p.read_text())
        n = d.get("package", {}).get("name")
        if not isinstance(n, str):
            continue
        v = d.get("package", {}).get("version")
        if isinstance(v, dict):
            v = None  # workspace-inherited; resolved below if needed
        rel = str(p.parent.relative_to(collection))
        col[rel] = (n, v)
        if n.startswith("sotf-") and len(n) > 5:
            oldmap[n[5:]] = (rel, n, v)
    # resolve inherited versions
    for rel, (n, v) in list(col.items()):
        if v is None:
            here = collection / rel
            for parent in [here, *here.parents]:
                if collection not in parent.parents and parent != collection:
                    continue
                w = tomllib.loads((parent / "Cargo.toml").read_text()) if (parent / "Cargo.toml").is_file() else {}
                wv = w.get("workspace", {}).get("package", {}).get("version")
                if isinstance(wv, str):
                    col[rel] = (n, wv)
                    if n.startswith("sotf-"):
                        r, _, _ = oldmap[n[5:]]
                        oldmap[n[5:]] = (r, n, wv)
                    break

    skip = {"target", "tmp", ".worktrees", "audit", "artifacts", ".git"}
    manifests = sorted(p for p in consumer.rglob("Cargo.toml")
                       if not any(part in skip for part in p.relative_to(consumer).parts))
    # collect patch entries repo-wide: [(manifest, src, key, path, target_rel|None)]
    patches = []
    for m in manifests:
        data = tomllib.loads(m.read_text())
        for src, entries in data.get("patch", {}).items():
            if not isinstance(entries, dict):
                continue
            for key, val in entries.items():
                if isinstance(val, dict) and "path" in val:
                    tgt = (m.parent / val["path"]).resolve()
                    try:
                        trel = str(tgt.relative_to(collection))
                    except ValueError:
                        continue
                    if trel in col or trel + "/Cargo.toml":
                        patches.append((m, src, key, val["path"], trel if trel in col else None))
    patched_keys = {key for _, _, key, _, _ in patches}

    report, warnings, changed = [], [], []

    # repo-wide direct dependency packages (a root patch may serve member deps)
    repo_pkgs: set[str] = set()
    for m in manifests:
        for _parts, _key, _val in dep_tables(tomllib.loads(m.read_text())):
            repo_pkgs.add(_val.get("package", _key) if isinstance(_val, dict) else _key)

    for m in manifests:
        lines = m.read_text().splitlines(keepends=True)
        headers = parse_headers(lines)
        data = tomllib.loads("".join(lines))
        dirty = False

        def find_table(parts):
            for hpos, (_i, kind, hp) in enumerate(headers):
                if kind == "table" and hp == parts:
                    return hpos
            return None

        def reindex():
            return parse_headers(lines)

        # 1. path deps into the collection: fix package + version
        for parts, key, val in dep_tables(data):
            if not (isinstance(val, dict) and "path" in val):
                continue
            tgt = (m.parent / val["path"]).resolve()
            try:
                trel = str(tgt.relative_to(collection))
            except ValueError:
                continue
            if trel not in col:
                warnings.append(f"{m}: path dep {key} -> {trel} is not a collection crate")
                continue
            new_name, new_ver = col[trel]
            if val.get("package", key) == new_name and ("version" in val or new_ver is None):
                continue
            hpos = find_table(parts + [key])
            if hpos is not None:  # long form
                hi = headers[hpos][0]
                for kk, vv in (("package", new_name), ("version", new_ver)):
                    if kk == "version" and ("version" in val or vv is None):
                        continue
                    if kk == "package" and val.get("package") == new_name:
                        continue
                    lines.insert(hi + 1, f'{kk} = "{vv}"\n')
                    headers = reindex()
                    report.append(f"{m.relative_to(consumer)}: dep {key}: {kk} = {vv}")
                    dirty = True
                continue
            hpos = find_table(parts)
            start, end = span_of(headers, hpos, len(lines))
            pat = re.compile(r"^(\s*)([\"']?)%s\2\s*=" % re.escape(key))
            li = next((i for i in range(start + 1, end) if pat.match(lines[i])), None)
            assert li is not None, f"{m}: cannot locate dep {key}"
            ob = lines[li].index("{")
            ei = brace_end(lines, li, ob)
            span = "".join(lines[li:ei + 1])
            if val.get("package", key) != new_name:
                if re.search(r"[\"']?package[\"']?\s*=", span):
                    span = re.sub(r"([\"']?package[\"']?\s*=\s*\")[^\"]*(\")",
                                  r"\g<1>%s\2" % new_name, span, count=1)
                else:
                    span = span.replace("{", '{ package = "%s", ' % new_name, 1)
                report.append(f"{m.relative_to(consumer)}: dep {key}: package = {new_name}")
                dirty = True
            if "version" not in val and new_ver is not None:
                span = span.replace("{", '{ version = "%s", ' % new_ver, 1)
                report.append(f"{m.relative_to(consumer)}: dep {key}: version = {new_ver}")
                dirty = True
            new_lines = span.splitlines(keepends=True)
            if span.endswith("\n") and not new_lines[-1].endswith("\n"):
                new_lines[-1] += "\n"
            lines[li:ei + 1] = new_lines
            headers = reindex()
            data = tomllib.loads("".join(lines))

        # 2. registry deps on old names with a patch entry: convert to path
        for parts, key, val in dep_tables(data):
            pkg = val.get("package", key) if isinstance(val, dict) else key
            if pkg not in oldmap:
                continue
            if isinstance(val, dict) and ("path" in val or "git" in val or val.get("workspace") is True):
                continue
            if pkg not in patched_keys and key not in patched_keys:
                warnings.append(f"{m.relative_to(consumer)}: still on upstream `{pkg}` "
                                f"(no patch entry; left for review)")
                continue
            trel, new_name, _ = oldmap[pkg]
            # path from this manifest to the collection target
            rel = os.path.relpath(collection / trel, m.parent).replace(os.sep, "/")
            req = val if isinstance(val, str) else val.get("version")
            assert isinstance(req, str), f"{m}: dep {key} has no plain version req"
            hpos = find_table(parts)
            start, end = span_of(headers, hpos, len(lines))
            pat = re.compile(r"^(\s*)([\"']?)%s\2\s*=" % re.escape(key))
            li = next((i for i in range(start + 1, end) if pat.match(lines[i])), None)
            assert li is not None, f"{m}: cannot locate dep {key}"
            if isinstance(val, str):
                lines[li] = re.sub(r"=\s*\"[^\"]*\"",
                                   '= { version = "%s", package = "%s", path = "%s" }'
                                   % (req, new_name, rel), lines[li], count=1)
            else:
                ob = lines[li].index("{")
                ei = brace_end(lines, li, ob)
                span = "".join(lines[li:ei + 1])
                span = span.replace("{", '{ package = "%s", path = "%s", ' % (new_name, rel), 1)
                new_lines = span.splitlines(keepends=True)
                if span.endswith("\n") and not new_lines[-1].endswith("\n"):
                    new_lines[-1] += "\n"
                lines[li:ei + 1] = new_lines
            headers = reindex()
            report.append(f"{m.relative_to(consumer)}: dep {key}: converted to path {rel} ({new_name})")
            dirty = True
            data = tomllib.loads("".join(lines))

        # 3. dead patch entries: old-name keys with collection paths are removed;
        #    entries already keyed by the new name are live registry-flow overrides.
        for (pm, src, pkey, _ppath, trel) in [x for x in patches if x[0] == m]:
            if trel is None:
                warnings.append(f"{m.relative_to(consumer)}: patch {src}/{pkey} target has no manifest; left")
                continue
            new_name = col[trel][0]
            if pkey == new_name:
                report.append(f"{m.relative_to(consumer)}: patch {src}/{pkey} kept (registry-flow override)")
                continue
            # delete inline entry or long-form header block
            hpos = find_table(["patch", src, pkey])
            if hpos is not None:
                s, e = span_of(headers, hpos, len(lines))
                del lines[s:e]
                headers = reindex()
            else:
                hpos = find_table(["patch", src])
                s, e = span_of(headers, hpos, len(lines))
                pat = re.compile(r"^(\s*)([\"']?)%s\2\s*=" % re.escape(pkey))
                li = next((i for i in range(s + 1, e) if pat.match(lines[i])), None)
                assert li is not None, f"{m}: cannot locate patch {pkey}"
                del lines[li]
                headers = reindex()
            if pkey in repo_pkgs or new_name in repo_pkgs:
                why = "direct dep is now path-based"
            else:
                why = (f"transitive-only redirect; transitive users now resolve upstream "
                       f"{pkey} (review if the fork delta matters there)")
            report.append(f"{m.relative_to(consumer)}: patch {src}/{pkey} removed ({why})")
            dirty = True
        # drop emptied [patch...] headers
        while True:
            dropped = False
            for hpos, (_i, kind, parts) in enumerate(list(headers)):
                if kind != "table" or not parts or parts[0] != "patch":
                    continue
                s, e = span_of(headers, hpos, len(lines))
                if any("=" in ln for ln in lines[s + 1:e]):
                    continue
                del lines[headers[hpos][0]]
                headers = reindex()
                dropped = True
                break
            if not dropped:
                break
            dirty = True

        if dirty:
            tomllib.loads("".join(lines))  # validate
            changed.append(str(m.relative_to(consumer)))
            if apply:
                m.write_text("".join(lines))

    print(f"consumer: {consumer}")
    print(f"collection: {collection}")
    print(f"mode: {'APPLY' if apply else 'CHECK (no writes)'}")
    print(f"manifests scanned: {len(manifests)}")
    print(f"\nchanges ({len(report)}):")
    for r in report:
        print(f"  - {r}")
    if warnings:
        print(f"\nwarnings ({len(warnings)}):")
        for w in sorted(set(warnings)):
            print(f"  ! {w}")
    print(f"\nmanifests changed: {len(changed)}")
    if not apply and report:
        print("re-run with --apply to write.")


if __name__ == "__main__":
    main()
