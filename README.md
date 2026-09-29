Вот краткий README (~50 строк) — написан так, чтобы его понимали и англоязычный скринер, и ИИ-агенты (которые и есть целевая аудитория проекта):

```markdown
# forge

**A Rust CLI that turns a Git repository into structured, validated context
for AI coding agents.**

AI coding agents fail on context, not on code. `forge` keeps the specification
layer of a project — tasks, decisions, guardrails — as plain Markdown, with
Git as the single source of truth. Humans and agents (Cline, Claude Code)
work with the same files; history, review, and rollback come from Git for free.

## Principles

- **Git is the source of truth.** A task's status is its folder. Moving a task
  is a rename, recorded in history. No `status` / `uuid` / `created` fields
  that can drift out of sync.
- **One ID per fact.** Through-numbered IDs, issued once, greppable across
  the whole history (`task(001): ...`).
- **Spec first, code second.** `forge` manages specifications; code generation
  stays with the external agent (ADR-001).
- **Deterministic checks, no LLM in the loop.** Validation and hygiene rules
  run locally, in plain Rust.

## Install

```bash
cargo install --git https://github.com/gera-gas/forge
```

## Usage

```bash
forge init                        # scaffold forge-src/ + AGENTS.md in a repo
forge task new "Kanban board"     # create a task, assign the next free ID
forge task move 001 4_in_progress
forge task list --format json     # machine-readable output for agents
```

## Layout

```
forge-src/
├── 3_todo/            # status = folder
├── 4_in_progress/
├── 5_done/
├── arch.md            # ADRs, append-only
└── workflows/         # agent guardrails: git.md, agent.md
AGENTS.md              # entry point for coding agents → forge-src/README.md
```

## Roadmap

- [ ] Auto-commits on task mutations (`task(001): move 001 -> 4_in_progress`)
- [ ] `forge task show` — task biography from Git history
- [ ] `forge status` / `forge log` — board summary and event stream
- [ ] `forge validate` — frontmatter schema, broken references
- [ ] `forge san` — hygiene: stale tasks, WIP limits, duplicate titles, context budget

## Status

Work in progress, actively used on real projects. Feedback welcome — open an issue.
```
