# Benchmarks

No results yet. Planned for milestone 5.

| Metric | agentjail | `docker run` | How measured |
|---|---|---|---|
| Cold start to first instruction | | | criterion, 100 runs |
| Rollback after `rm -rf ./*` | | n/a | criterion |
| I/O overhead compiling a large project | | | wall time vs. host |

Each result will list the kernel version, CPU and filesystem it was measured on.
