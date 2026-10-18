# Security Model

Write this doc early and keep it current.

## Risk Classes

| Risk class | Examples | Policy |
|---|---|---|
| Read-only | Read an allowed file, search, check calendar | Auto-approve within allowlists |
| Reversible write | Create a note, edit a file in a workspace folder | Allow within allowlist; log; support undo where possible |
| Destructive / external | Delete files, send email, arbitrary shell, spend money | Explicit human confirmation every time |

## Rules

- Use allowlists (directories, commands, domains), never denylists.
- Defend against prompt injection: content from web pages, emails and files must not silently trigger privileged tools.
- Keep secrets out of prompts and logs. Load from env or OS keychain; wrap in a secret type (e.g. `secrecy`).
- Keep an audit log of every tool call with arguments and results.
- Sandbox risky execution: separate process with dropped privileges, container, or WASM.
- Never commit secrets, local config, audit logs or memory databases.

## Approval Gate

- If the approval gate or permission policy can't initialize, block destructive and external tools. Never fall back to "allow everything".
- Destructive call blocks until approved. With the gate broken, destructive tools are refused.
