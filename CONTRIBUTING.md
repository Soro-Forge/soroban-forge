# Contributing

Thank you for considering a contribution. This project is structured to make contributing low-risk and quick to review.

## The short version

1. Find an open issue and comment asking to be assigned. Wait to be assigned before starting.
2. Fork the repository and branch from `main`.
3. **Work only inside the folder named in the issue.** This is the one rule that matters.
4. Open a pull request with `Closes #<issue>` in the description.

## Scope discipline

Every issue names exactly one directory. Your pull request must not modify files outside it.

This is strictly enforced, and it exists to protect you as much as the project: a change confined to one folder cannot break anything else, cannot conflict with another contributor, and can be reviewed and merged the same day. Pull requests that reach outside their folder will be asked to narrow their scope.

If you believe your change genuinely requires touching shared code, open a separate issue proposing it rather than including it.

## What a good contribution looks like

- One directory
- A check implemented as a pure function over source text
- Tests covering the defect being detected **and** the clean case that must not trigger a false positive
- At least one committed fixture file demonstrating each
- A short `README.md` in the tool folder explaining what the check catches and why it matters

Small is good. A focused hundred-line contribution with real tests is worth more here than a sprawling one.

## Before you open a pull request

```bash
cargo fmt --all
cargo clippy --all-targets -- -D warnings
cargo test
```

All three must pass. Please paste the test output into your pull request description.

## Pull request description

Tell us what you did, why, and how you verified it. Confirm that your changes are limited to the issue's folder.

## Code of conduct

Be decent to each other. Assume good faith. Reviews are about the code.
