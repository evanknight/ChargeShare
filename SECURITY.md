# Security policy

ChargeShare is an early-stage public planning repository, not a production service.
No version is currently suitable for real credentials, vehicle access, or personal
data. Only synthetic examples belong in this repository.

## Reporting a vulnerability

Do not publish credentials, personal data, or exploitable details in an issue or
pull request. Use GitHub's private **Report a vulnerability** option on this
repository's Security tab if it is available. If no private channel is available,
open a minimal issue asking the maintainer to establish one, without sensitive
details. Do not assume private reporting is enabled.

## If a secret is exposed

1. Revoke or rotate the credential immediately at its issuer.
2. Tell the maintainer through a private channel. Do not paste the secret again.
3. Assess use and scope, remove the exposure, and coordinate any history cleanup.
   Removing a file or rewriting Git history does not revoke a secret or erase
   copies, caches, or forks.
4. Re-run the staged and complete-history scans before publishing again.

See [the repository security guide](docs/security.md) for contributor safeguards
and their limits.
