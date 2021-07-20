#!/usr/bin/env python3

import pathlib
import json
import argparse


def main():
    repo_root = pathlib.Path(__name__).parent
    runs_root = repo_root / "runs"
    default_run_path = sorted(runs_root.glob("*.json"))[-1]

    parser = argparse.ArgumentParser()
    parser.add_argument("--run", metavar="PATH", type=pathlib.Path, default=default_run_path, help="Default: %(default)s")
    args = parser.parse_args()

    data = json.loads(args.run.read_text())
    cases = sorted(data["libs"].values(), key=lambda c: (c["name"] if c["name"] else "", c["version"]))

    print(" | ".join(["Crate"] + [case["name"] if case["name"] else "null" for case in cases]))
    print("-|-".join(["-"] + ["-" for case in cases]))
    print(" | ".join(["Binary Overhead (release)"] + [str(fmt_size(case, cases[0])) for case in cases]))
    print(" | ".join(["Build Time (debug)"] + [str(fmt_time(case)) for case in cases]))
    print(" | ".join(["Dependencies"] + [str(case["deps"]) for case in cases]))
    print(" | ".join(["Version"] + [case["version"] if case["version"] else "-" for case in cases]))
    print()
    print(f"**System: {data['os']} {data['os_ver']} ({data['arch']})**")


def fmt_time(case):
    value = case["build"]["results"][0]["median"]
    return "{:.1f}".format(value)


def fmt_size(case, null_case):
    delta = (case["size"] - null_case["size"]) / 1024
    return "{:,.1f} KiB".format(delta)


if __name__ == "__main__":
    main()
