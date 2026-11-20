# Red-team tests

Each test is an escape attempt that must fail. Every guarantee in `docs/DESIGN.md` gets at least one.

Planned:

- Read `~/.ssh` and `/proc/1/environ` from inside the sandbox (G1)
- Symlink and hard-link traversal out of the workspace (G1, G4)
- Rewrite the host binary through `/proc/self/exe` (G1)
- Fork bomb and memory bomb (G3)
- Leftover processes and mounts after OOM kill (G3)
- `curl` a fake API key to an external host (G2)
- Reach `127.0.0.1` and `169.254.169.254` (G2)
- DNS rebinding to a private address (G2)
- `rm -rf` the workspace, then roll back (G4)
- `ptrace`, `mount`, `bpf`, `io_uring_setup` syscalls (all)
