# Sweep Contributing Guide

Hey there! Thank you for your interest in contributing to Sweep. Before submitting your contribution, please make sure to read this document.

The following types are used across branch names, commit messages and pull requests:

| Type       | Description                                |
| ---------- | ------------------------------------------ |
| `feat`     | A new feature                              |
| `fix`      | A bug fix                                  |
| `chore`    | Maintenance or housekeeping                |
| `docs`     | Documentation changes                      |
| `refactor` | Code restructuring without behavior change |
| `ci`       | CI/CD pipeline changes                     |

## Commit Messages

Commit messages must follow this pattern: `<type>: <description>`.

Good examples:

- `feat: add sliding context window`
- `docs: add contributing guide`

Bad examples:

- `added sliding context window`
- `fix stuff`
- `WIP`

## Pull Requests

Pull request titles must follow this pattern: `<type>: <title>`.

Good examples:

- `feat: sliding context window`
- `docs: add contributing guide`

Bad examples:

- `sliding context window`
- `added new channel edit tool`

If your PR resolves an issue, [include closing keywords](https://docs.github.com/en/issues/tracking-your-work-with-issues/using-issues/linking-a-pull-request-to-an-issue#linking-a-pull-request-to-an-issue-using-a-keyword).

## Branch Naming

Branches must follow this pattern: `<type>/<description>`. If you are creating a branch for an issue, name it: `<type>/<issue_id>-<issue_title>`.

Good examples:

- `feat/sliding-context-window`
- `feat/2-message-react-tool`

Bad examples:

- `2-message-react-tool`
- `sliding-context-window`
- `ideas/new-tools`
