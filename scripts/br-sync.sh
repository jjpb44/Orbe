#!/usr/bin/env bash
# br → remote issue tracker sync.
#
# The local `br` tracker is private (`.beads/` is not committed). This script
# mirrors the issues you care about into the public tracker so clients and
# contributors can see them — one-way, br → remote, idempotent.
#
#   scripts/br-sync.sh --dry-run          # show what would happen
#   scripts/br-sync.sh                    # create/update remote issues
#   scripts/br-sync.sh --repo SethDev/Orbe --forge gitea   # Codeberg/upd
#
# Mapping lives in `.beads/remote-map.json` (local, untracked).
set -uo pipefail
cd "$(dirname "$0")/.." || exit 1

REPO="jjpb44/Orbe"
FORGE="github"      # github | gitea (Codeberg, upd.dev — needs a token)
DRY=0
MAP=".beads/remote-map.json"

while [[ $# -gt 0 ]]; do
    case "$1" in
        --dry-run) DRY=1; shift ;;
        --repo) REPO="$2"; shift 2 ;;
        --forge) FORGE="$2"; shift 2 ;;
        -h|--help) sed -n '2,20p' "$0"; exit 0 ;;
        *) echo "unknown arg: $1"; exit 2 ;;
    esac
done

command -v br >/dev/null || { echo "br not installed"; exit 1; }
[[ -f .beads/remote-map.json ]] || echo '{}' > "$MAP"

python3 - "$REPO" "$FORGE" "$DRY" "$MAP" <<'PY'
import json, os, subprocess, sys

repo, forge, dry, map_path = sys.argv[1], sys.argv[2], sys.argv[3] == "1", sys.argv[4]

issues = json.loads(subprocess.run(
    ["br", "list", "--json"], capture_output=True, text=True, check=True
).stdout).get("issues", [])

state = json.loads(open(map_path).read() or "{}")


def gh(*args):
    return subprocess.run(["gh", *args], capture_output=True, text=True)


def gitea(*args):
    # Codeberg / upd.dev are Forgejo (Gitea-compatible) — REST API.
    host = "codeberg.org" if repo.startswith("SethDev") else "ssh.upd.dev"
    owner, _, name = repo.partition("/")
    token = os.environ.get("FORGE_TOKEN", "")
    url = f"https://{host}/api/v1/repos/{owner}/{name}/issues"
    return subprocess.run(
        ["curl", "-sS", "-X", "POST", url,
         "-H", f"Authorization: token {token}",
         "-H", "Content-Type: application/json",
         "-d", json.dumps(payload)],
        capture_output=True, text=True,
    )


def body_for(issue):
    return (
        f"<!-- orbe-br-id: {issue['id']} -->\n\n"
        f"**br id** `{issue['id']}` · **type** `{issue.get('issue_type','task')}` · "
        f"**priority** `{issue.get('priority',2)}`\n\n"
        f"{issue.get('description','') or '_No description._'}\n\n"
        "---\n<sub>Mirrored from the local `br` tracker by `scripts/br-sync.sh`. "
        "The br tracker itself is not published.</sub>"
    )


def title_for(issue):
    return f"[{issue.get('issue_type','task')}] {issue['title']}"


created = updated = closed = 0
for issue in issues:
    bid = issue["id"]
    title = title_for(issue)
    body = body_for(issue)
    status = issue.get("status", "open")
    entry = state.get(bid)

    if entry is None:
        if dry:
            print(f"  would create  {bid} → {title}")
        else:
            if forge == "github":
                res = gh("issue", "create", "--repo", repo, "--title", title, "--body", body)
            else:
                res = gitea("create", payload={"title": title, "body": body})
            if res.returncode == 0:
                num = ""
                if forge == "github":
                    num = res.stdout.strip().split("/")[-1]
                else:
                    try:
                        num = str(json.loads(res.stdout).get("number", ""))
                    except Exception:
                        num = ""
                state[bid] = {"number": num, "repo": repo, "title": title}
                created += 1
                print(f"  created      {bid} → #{num} {title}")
            else:
                print(f"  FAILED       {bid}: {res.stderr.strip()[:120]}")
        continue

    if entry.get("title") != title and not dry:
        if forge == "github":
            gh("issue", "edit", str(entry["number"]), "--repo", repo, "--title", title, "--body", body)
        updated += 1
        print(f"  updated      {bid} → #{entry['number']} {title}")
    elif entry.get("title") != title:
        print(f"  would update {bid} → {title}")

    if status == "closed" and not dry:
        if forge == "github":
            res = gh("issue", "close", str(entry["number"]), "--repo", repo)
            if res.returncode == 0:
                closed += 1
                print(f"  closed       {bid} → #{entry['number']}")

if not dry:
    open(map_path, "w").write(json.dumps(state, indent=2) + "\n")
    print(f"\n  created {created} · updated {updated} · closed {closed}")
else:
    print("\n  dry run — nothing written")
PY
