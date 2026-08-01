---
name: Tool issue
about: A scoped, folder-isolated unit of work
title: "[V1][analysis] <Tool Name> - <Aspect>"
labels: ""
assignees: ""
---

## Goal

<One sentence describing what this issue produces.>

Tool: **<Tool Name>**

Release tier: **V1**

Audience: **analysis**

## Repository evidence

<Point at the concrete gap this addresses. Cite a file, a function, or an observed failure. An issue without evidence is not ready to publish.>

## Implementation folder

All work for this issue must stay inside:

```text
tools/v1/analysis/<tool-name>/
```

Do not modify the workspace manifest beyond adding this crate, other tools, CI configuration, shared documentation, or any directory not named above.

## Deliverables

- <Deliverable one>
- <Deliverable two>
- <Deliverable three>

## Acceptance criteria

- [ ] The check is a pure function over source text and does not require the input to compile
- [ ] Findings include a line number and a human-readable explanation
- [ ] Tests cover both the defective case and a clean case that must not trigger
- [ ] Fixtures are committed for each
- [ ] Files changed are limited to the folder named above
- [ ] The contribution is reviewable as a self-contained change

## Contributor notes

Keep the work small and reviewable. Prefer local fixtures, local tests and folder-local helpers. If this tool needs to connect to anything else in the repository, write that as a follow-up issue rather than adding it here.
