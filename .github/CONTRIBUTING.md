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

## Branches

The `main` branch is the single source of truth for active development. External contributors should fork the repository and create their feature branches on their fork before submitting a Pull Request to our `main` branch.

### Branch Naming

Branches must follow this pattern: `<type>/<description>`. If you are creating a branch for an issue, name it: `<type>/<issue_id>-<issue_title>`. Release branches must follow this pattern: `release/MAJOR.MINOR.x` (like: `release/0.2.x`).

Good examples:

- `feat/sliding-context-window`
- `feat/2-message-react-tool`
- `docs/10-capabilities-section`

Bad examples:

- `2-message-react-tool`
- `sliding-context-window`
- `ideas/new-tools`

## Releases

*(Maintainer Note):* Before releasing a new minor or major version, a maintainer will create a branch named `release/MAJOR.MINOR.x` from the current `main` branch to freeze the release candidate and handle patch deployment. The releases must be made from the release branch instead of the `main` branch.

## Bugfixes

If you are fixing a critical bug found in the current production release, please target your Pull Request directly to the active `release/MAJOR.MINOR.x` branch. A maintainer will handle merging the fix down into `main`.

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
