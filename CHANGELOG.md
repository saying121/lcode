<!-- markdownlint-disable MD024 -->

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.9.7]

### Added

- Avatar tweak position

### Fixed

- Panic when avatar is empty.

## [0.9.6]

### Added

- Tui: show user's avatar.

## [0.9.5]

### Fixed

- Keymap: Use correct field name `keymap`.

## [0.9.4]

### Added

- Config: add `dir_with_frontend_id` field, when filled true use frontend id create dir.

## [0.9.3]

### Added

- lcode: when submit not pass test case can add last test case.

### Chore

- Rust: Switch to Stable channel.
- Clippy: tweak lints.

### Perf

- Avoid some heap allocation by `format_push_string` lint

## [0.9.2]

### Perf

- Avoid unnecessary render and filter.

## [0.9.1] - 2024-05-18

### Fixed

- Tui: submit panel can't vertical scroll.

## [0.9.0] - 2024-05-14

### Added

- Use `toggle` trigger submit and test code.
- Add pop button.

### Changed

- Remove `submit_code` and `test_code` keymap, use replace by `toggle`.
- Not cycle topic tags.
- ratatui downgrade to 0.26.1 for fix Chinese broken.
- lcode-config: move global.rs upper dir.

### Fixed

- Correct apply user keymaps.
- First open will panic.

## [0.8.1] - 2024-04-28

### Fixed

- Cli: `lcode fzy detail` and `lcode generate` help

## [0.8.0] - 2024-04-28

### Added

- Shell completion run `lcode --generate <zsh|bash|...>`

### Changed

- `reqwest` feature from gzip to brotli.
- `lcode-config`: change dir name form "leetcode-cn-en-cli" to "lcode".
- Not auto generate config files, run `lcode gencon` generate default config.

## [0.7.18] - 2024-04-22

### Added

- Edit log file.

### Refactor

- leetcode-api: query.

## [0.7.17] - 2024-04-19

### Fixed

- When browser is empty panic.

### Added

- Check cookies expiry by decrypt-cookies v0.5.3.

## [0.7.16] - 2024-04-18

### Added

- Add helix magic.
- `leetcode-api` handle 429 status code frequent.

### Style

- Change question content.

## [0.7.15] - 2024-04-09

### Fixed

- Show submit info.

## [0.7.14] - 2024-04-01

### Fixed

- Update decrypt-cookies 0.5.

## [0.7.13] - 2024-03-29

### Fixed

- decrypt-cookies 0.4.1

## [0.7.12] - 2024-03-25

### Added

- Auto get cookies add safari support.

## [0.7.11] - 2024-03-24

### Changed

- Remove built-in markdown rendering and rely on mdcat instead.

## [0.7.10] - 2024-03-23

### Added

- When editor is vim or nvim, will open question and code.
- Auto get cookies add more browser support.
