# Commercial model — Orbe

**Short version:** the code is free (AGPL-3.0-only). We sell *hosting and
operations*, not the code. Self-hosting is legal and encouraged; running a
modified copy as a closed service is not.

## The two things we sell

| Offering | What it is | Price model |
|---|---|---|
| **Orbe (self-hosted)** | The repo, as-is, AGPL-3.0-only | Free |
| **Orbe Cloud** | We run it for you: hosting, database, backups, upgrades, onboarding, support | Monthly subscription |

The monthly license you sell is a **service subscription** (hosting + SLA +
onboarding + support), not a license fee on the code. This distinction is what
keeps the project genuinely open source: nobody has to pay to run Orbe.

## What AGPL-3.0 gives us — and what it does not

AGPL §13 is the clause that matters for a hosted product:

> If you modify the Program, your modified version must prominently offer all
> users interacting with it remotely through a computer network … an opportunity
> to receive the Corresponding Source of your version.

| Scenario | Allowed under AGPL-3.0? |
|---|---|
| You install Orbe for your own shop | ✅ yes, free, no conditions |
| You modify Orbe for yourself and host it internally | ✅ yes — but you must publish your modifications to your users |
| You fork Orbe, change it, and sell it as a hosted SaaS | ❌ no — the modifications must be published to your users |
| You run Orbe unmodified and charge for hosting it | ✅ yes — the license places no restriction on commercial use or on charging for it (§4) |
| A competitor sells "Orbe + onboarding" with no changes | ⚠️ legally yes — this is where the **trademark** and the **service** are our moat, not the code |

**What AGPL does not protect:** someone else deploying the unmodified program
and selling hosting around it. Nothing in the license prevents that, and trying
to prevent it with license terms would make the project not-open-source. We
protect that case with trademarks (`Orbe`, logo, screenshots) and with the
hosted offering being genuinely better than a self-install.

## If we ever want to sell on-prem/white-label

Dual licensing: AGPL-3.0-only for the community, plus a **commercial license**
for organisations that want to keep modifications private or ship under their
own brand. That requires a copyright assignment or CLA from contributors
(or keeping copyright in a single entity). Until then, the repo is maintained by
one entity and the commercial license can be granted directly.

## Trademark policy

- "Orbe" in Latin script, the logo, and the icon set are reserved.
- You may use the name to state compatibility ("compatible with Orbe") or in
  unmodified redistributions, as AGPL §7(e) allows.
- You may not use the name or logo for a modified derivative without permission.

## What the UI must show (AGPL §13 + §5)

If Orbe is offered over a network, the interface must offer its users access to
the corresponding source. Concretely, the hosted deployment must show a
"Source code" link pointing at the public repository, and any modified
deployment must link at its own source. This is a one-line component, but it is
a legal requirement — track it as a task before the first paid deployment.

## House rules derived from this

- Never add a clause that restricts commercial use of the unmodified program.
- Never ship a build whose modifications cannot be obtained by its users.
- Keep the hosted offering's value in operations (reliability, migrations,
  backups, onboarding, support) — never in hiding the source.

## References

- AGPL-3.0 text: `LICENSE` (canonical copy: <https://www.gnu.org/licenses/agpl-3.0.txt>)
- AGPL §4 (may charge for copies), §5 (modifications must carry notices),
  §7(e) (trademark), §13 (network interaction → offer the source)
- Comparable models: Mastodon, Nextcloud, GitLab (AGPL + hosted service)
