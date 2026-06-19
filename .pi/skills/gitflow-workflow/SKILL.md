---
name: gitflow-workflow
description: Project-specific Git workflow rules for branching, merges, and commit message formatting. Use when creating branches, planning Git operations, or answering questions about repository workflow.
---

# Project Gitflow Workflow

## Purpose
This skill defines the required Git workflow for this project.

If any default or built-in Git workflow guidance conflicts with this skill, this skill takes precedence.

## Branch model
The repository must always use these long-lived branches:
- `main`
- `dev`

Rules:
- Never commit directly to `main`.
- Never commit directly to `dev`.
- `main` and `dev` must only receive changes through merges.

## Allowed working branches
All work must be done on a short-lived branch created from the appropriate base branch. Feature branches must be created from `dev`.

Allowed branch types:
- `feature/`
- `hotfix/`

Rules:
- Every non-merge code change must be made on either a `feature/*` branch or a `hotfix/*` branch.
- Create `feature/*` branches from `dev` and verify or record the branch base before implementation starts.
- Do not create work directly on `main` or `dev`.
- Do not use version numbers, ticket numbers, iteration numbers, or commit counters in branch names unless the user explicitly requires them for a separate process.

## Branch naming
Branch names must follow this format:

- `feature/<work-being-done>`
- `hotfix/<work-being-done>`

Examples:
- `feature/my-awesome-work`
- `hotfix/fix-startup-crash`

Naming rules:
- Use lowercase words.
- Separate words with hyphens.
- Keep the name descriptive but concise.
- Do not append version numbers.
- Do not append commit iteration numbers.

## Merge expectations
- Merge completed feature work into `dev`.
- Merge completed hotfix work through the project's release process, but never by committing directly to `main` or `dev`.
- Preserve the rule that `main` and `dev` receive merges only.
- After any merge, push the resulting branch to `origin` immediately when `origin` is configured.
- If `origin` is not configured, treat the repository as local-only and record push status as `N/A (local-only repository)`.
- If a required push fails, do not treat the merge checkpoint as complete; record the failure and remediate before proceeding.

## Remote backup expectations
- After every commit on `feature/*` or `hotfix/*`, push that branch to `origin` immediately when `origin` is configured.
- If the branch has no upstream yet, set it on first push (for example `git push -u origin <branch>`).
- If `origin` is not configured, treat the repository as local-only and record push status as `N/A (local-only repository)`.
- If a required push fails, do not treat the commit checkpoint as complete; record the failure and remediate before continuing phase progression or merge steps.

## Commit message rules
Commit messages must:
- be short
- briefly describe the work being done
- use correct capitalization in the title line
- start the title line with a capitalized first word, for example `Add plugin compile prompt`
- preserve the source's original capitalization when naming a file, class, property, function, symbol, or code snippet
- not use prefixes like `feat(...)`, `fix(...)`, or similar conventional commit prefixes unless the user explicitly asks for them
- not include author or co-author lines
- include a list of the files changed

### Commit message format
Use this structure:

```text
Short description of work

Files changed:
- path/to/file1
- path/to/file2
```

Example:

```text
Add plugin compile prompt

Files changed:
- .pi/prompts/compile-project.md
- AGENTS.md
```

## How to apply this skill
When asked to help with Git workflow in this project:
1. Ensure work is being done on a `feature/*` or `hotfix/*` branch.
2. Refuse or redirect any request that would commit directly to `main` or `dev`.
3. Recommend merges instead of direct commits for `main` and `dev`.
4. After each commit or merge, require a push to `origin` when `origin` exists.
5. If no `origin` exists, explicitly record push state as `N/A (local-only repository)`.
6. If push fails, record the failure and keep the checkpoint in a non-complete state until push remediation succeeds.
7. Generate commit messages using the required short format, proper title capitalization, and the changed file list.
8. Preserve exact capitalization for any source names referenced in the commit title or body.
9. Do not add commit prefixes, author lines, or co-author lines.

## Summary
Required policy:
- Long-lived branches: `main`, `dev`
- No direct commits to `main` or `dev`
- Work only on `feature/*` or `hotfix/*`
- Branch naming: `<branch-type>/<work-being-done>`
- Push to `origin` after every commit and merge when `origin` exists
- Local-only repos must record push state as `N/A (local-only repository)`
- Push failures must be recorded and remediated before checkpoints are considered complete
- Short commit messages
- Capitalized commit titles
- Preserve source capitalization for referenced code/file names
- Commit messages must include changed files
- No author or co-author lines
