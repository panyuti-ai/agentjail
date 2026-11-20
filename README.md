# agentjail

Run untrusted AI agent code in a rootless Linux sandbox that cannot read your secrets, reach
unapproved hosts, exhaust your machine or damage your workspace, and roll back any change in one
command.

> Status: early development. Nothing works yet. See the roadmap below.

```sh
agentjail run --workspace ./demo -- bash
```

## Why

AI coding agents run shell commands on your machine. One prompt injection in a web page or a
dependency is enough to turn that agent against you. agentjail assumes the agent is already
compromised and limits what it can do. The threat model is in [docs/DESIGN.md](docs/DESIGN.md).

## Roadmap

- [ ] M1: rootless isolation (user, PID, mount, IPC, UTS namespaces, `pivot_root`)
- [ ] M2: resource limits and defence in depth (cgroups v2, seccomp, Landlock)
- [ ] M3: copy-on-write snapshots, rollback, diff and commit (OverlayFS)
- [ ] M4: zero-trust egress proxy (allowlist, SSRF guard, secret scanning)
- [ ] M5: red-team escape tests, benchmarks against Docker, agent integration demo

## Requirements

- Linux kernel 5.15 or newer with cgroups v2 (Ubuntu 24.04 recommended; WSL2 works)
- Rust 1.80 or newer

## Layout

| Path | Purpose |
|---|---|
| `crates/box-core` | Isolation engine |
| `crates/box-proxy` | Egress proxy |
| `crates/box-cli` | `agentjail` command |
| `tests/integration` | Lifecycle, snapshot and rollback tests |
| `tests/redteam` | Escape attempts that must fail |
| `benches` | Criterion benchmarks |
| `docs` | Design and benchmark reports |

## License

MIT
