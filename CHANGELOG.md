<!-- markdownlint-disable MD024 -->
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.8.0] - 2024-04-28

### Added

- shell completion run `lcode --generate <zsh|bash|...>`

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
