#!/usr/bin/env python3
"""Check fork documentation and the aggregate's active local patch paths."""

import json
from pathlib import Path
import subprocess
import tomllib


def main() -> None:
    collection = Path(__file__).resolve().parents[1]
    aggregate = collection.parent
    inventory = json.loads((collection / "FORKS.json").read_text())
    errors: list[str] = []
    directories: set[str] = set()
    required = ["upstream_url", "upstream_revision_status", "reason", "retained_changes",
                "owner", "review_cadence_days", "removal_condition", "verification_command"]
    for fork in inventory["forks"]:
        name = fork["directory"]
        if name in directories:
            errors.append(f"Duplicate fork: {name}")
        directories.add(name)
        for field in required:
            if not fork.get(field):
                errors.append(f"{name}: missing {field}")
        for relative in [name + "/Cargo.toml", fork["maintenance_document"], fork["detail_document"]]:
            if not (collection / relative).is_file():
                errors.append(f"{name}: missing {relative}")
        if not fork["license_files"] and not fork.get("license_status"):
            errors.append(f"{name}: no preserved license files or documented license evidence gap")
        for relative in fork["license_files"]:
            if not (collection / name / relative).is_file():
                errors.append(f"{name}: missing license {relative}")
    repos = ["sotf", "sotf-daw", "sotf-capture", "sotf-systemwide", "autoeq",
             "math-audio", "gpui-toolkit", "sofa-reader", "symphonia-add-ons"]
    patches = 0
    for repo in repos:
        folder = aggregate / repo
        if not folder.exists():
            # This collection can be checked standalone; consumer checks need the aggregate.
            continue
        tracked = subprocess.check_output(["git", "-C", str(folder), "ls-files", "-z"], text=True)
        for relative in tracked.split("\0"):
            if not relative.endswith("Cargo.toml") or any(
                part in {"audit", "artifacts", "target"} for part in Path(relative).parts
            ):
                continue
            manifest = folder / relative
            if not manifest.exists():
                continue
            config = tomllib.loads(manifest.read_text())
            for source, entries in config.get("patch", {}).items():
                for package, dependency in entries.items():
                    patches += 1
                    if not isinstance(dependency, dict) or "path" not in dependency:
                        errors.append(f"{manifest}: {source}/{package} is not a local path")
                        continue
                    target = (manifest.parent / dependency["path"]).resolve()
                    if collection not in target.parents:
                        errors.append(f"{manifest}: {package} resolves outside the collection: {target}")
                    elif not (target / "Cargo.toml").is_file():
                        errors.append(f"{manifest}: missing patched crate {target}")
                    elif target.relative_to(collection).parts[0] not in directories:
                        errors.append(f"{manifest}: undocumented patch {target}")
    if errors:
        raise SystemExit("\n".join(errors))
    print(f"Verified {len(directories)} fork records, licenses and {patches} active local patch declarations.")


if __name__ == "__main__":
    main()
