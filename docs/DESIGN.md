# agentjail design

## Goal

Let an AI agent run arbitrary code on a developer machine without being able to hurt that machine,
with millisecond startup and one-command rollback of any change the agent made.

## Threat model

**Attacker:** the code running inside the sandbox. Assume the agent has been fully taken over by
prompt injection. It can write, compile and run any C, Rust, Python or shell program, run as many
processes as the sandbox allows, and try any syscall.

**Trusted:** the Linux kernel, the `agentjail` binary, the `box-proxy` process and the host user who
starts the sandbox.

**Out of scope:** kernel 0-days, hardware side channels (Spectre-class), physical access, and a
malicious host user.

### The four guarantees

The sandbox must make each of these impossible:

| # | Must never happen | Main defences | Milestone |
|---|---|---|---|
| G1 | Read host secrets (`~/.ssh`, `~/.aws`, browser profiles, env vars of other processes) | User + mount namespace, minimal read-only rootfs, `pivot_root`, closed inherited file descriptors, Landlock | M1, M2 |
| G2 | Send data to an unapproved network destination | Network namespace with no interfaces; only exit is a Unix socket to `box-proxy` with domain allowlist, SSRF guard and secret scanning | M4 |
| G3 | Exhaust host CPU, memory or process table | cgroups v2: `memory.max`, `cpu.max`, `pids.max`; cleanup with no leftover processes or mounts | M2 |
| G4 | Damage the original workspace files | OverlayFS: workspace is a read-only `lowerdir`, all writes go to `upperdir`; changes reach the host only through an explicit `commit` | M3 |

Seccomp-BPF (blocking `ptrace`, `mount`, `bpf`, `io_uring_setup`, `kexec_load` and similar)
supports all four by shrinking the kernel attack surface.

## Architecture

```
 host                                   sandbox (new user/pid/mnt/ipc/uts/net namespaces)
┌──────────────────────────┐           ┌────────────────────────────────────┐
│ agentjail (box-cli)      │  clone    │ agent process                      │
│  └─ box-core             │ ────────▶ │  rootfs: /usr /lib /bin read-only  │
│      namespaces, cgroup, │           │  /workspace: OverlayFS             │
│      seccomp, overlay    │           │  HTTP(S)_PROXY → unix socket       │
│                          │           └───────────────┬────────────────────┘
│ box-proxy (tokio)        │ ◀──── unix domain socket ─┘
│  allowlist, SSRF, DLP    │ ────▶ approved internet hosts only
└──────────────────────────┘
```

## Key decisions

| Decision | Choice | Why |
|---|---|---|
| Language | Rust | No hidden runtime threads, so `unshare(CLONE_NEWUSER)` works; static binary; memory safety without GC |
| Privileges | Rootless (user namespaces) | Agents run as normal users; no setuid binary to attack |
| Filesystem | Kernel OverlayFS in a user namespace (5.11+), `fuse-overlayfs` fallback | Copy-on-write snapshots at near-native speed |
| Proxy process | Separate from the sandbox launcher | `fork`/`unshare` must happen before tokio starts its thread pool |
| Syscall filter | `seccompiler` | Small, audited, used by Firecracker |

## Open questions

- Snapshot granularity: one OverlayFS layer per command, or on demand?
- How to stack many snapshot layers without slowing lookups.
- TLS: tunnel with `CONNECT` and inspect only the destination, or terminate TLS to scan bodies.
