# ChargeShare contributor guidance

This is a public, planning-first repository. Read README.md, docs/security.md and the active OpenSpec change before editing.

- Write application source in Rust. Node.js/npm is for OpenSpec and repository development tooling only.
- Keep changes inside this repository. Do not import private repositories, personal correspondence or live vehicle data.
- Use synthetic examples only. Never commit secrets, real VINs, home locations, bills, receipts, account identifiers or contact details.
- Do not register a Tesla app, pair keys, authorize OAuth, deploy, incur costs, or add vehicle-control features without a separate explicit request.
- Preserve the distinction between proposed behavior and implemented functionality. Keep unimplemented OpenSpec tasks unchecked.
- Use the generated OpenSpec skills under .agents/skills. Start with a proposal and scenarios, then implement an approved change.
- Run npm run spec:validate and the security checks described in docs/security.md before a commit. Review the exact staged diff and paths as well as scanner output. Do not bypass failing hooks.
- Treat every energy total as a measured or estimated quantity with a stated boundary and quality. Never advertise utility-meter accuracy without evidence.
