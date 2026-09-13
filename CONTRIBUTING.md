# Contributing to qvm-ctl

Thanks for your interest in contributing to `qvm-ctl`.

## Project Philosophy

`qvm-ctl` is intentionally minimal and objective. Its sole purpose is to manage QEMU/KVM virtual machine lifecycles cleanly from the terminal without libvirt or daemon bloat.

To maintain simplicity:
- We prefer simple shell functions and standard system tools over complex abstractions.
- We avoid feature bloat. If a proposed feature can be easily solved outside of `qvm` using standard UNIX tools, it probably doesn't belong in core.
- Every addition must keep the tool fast, robust, and dependable.

---

## Suggesting Features

If you have an idea for a feature or enhancement:

1. **Open an issue first.** Explain what problem you are trying to solve and how you envision the solution working.
2. **Discuss with maintainers.** We will evaluate whether the idea fits the minimal and objective scope of the project.
3. **Submit a pull request.** Once agreed upon, you are welcome to submit a PR implementing the change.

Opening an issue first saves everyone time and avoids work on PRs that may be closed for being out of scope.

---

## Reporting Bugs

Before reporting a bug, ensure the issue isn't caused by missing system dependencies (like `OVMF` or `swtpm`) or permission issues with `/dev/kvm`.

When opening a bug report, please include:
- Your Linux distribution and kernel version (`uname -r`).
- Versions of QEMU (`qemu-system-x86_64 --version`) and `qvm` (`qvm version`).
- The exact command you ran.
- What you expected to happen vs. what actually happened (including output from `qemu.log` if relevant).

---

## Pull Request Guidelines

If you are fixing a bug or submitting an agreed-upon feature:

1. **Keep it focused:** One bug fix or feature per pull request. Avoid mixing unrelated formatting changes or refactors with logic changes.
2. **Follow existing style:** Write clean, readable Bash matching the style of `src/qvm-ctl.sh`.
3. **Verify locally before pushing:** GitHub Actions runs our CI suite (ShellCheck and smoke tests) on every pull request. Running these checks locally ensures your PR is ready:
   - ShellCheck:
     ```bash
     shellcheck src/qvm-ctl.sh
     ```
   - Smoke tests:
     ```bash
     bash test/smoke.sh
     ```
4. **Ensure CI passes:** All automated checks in GitHub Actions must pass before a PR can be merged.
5. **Update documentation:** If your change modifies or introduces commands or configuration options, update the corresponding page under `docs/src/`.
