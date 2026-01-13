# Changelog: cattongue

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]


## [0.1.3] (2026-01-13)
### Added
* crate: `async-sleep-aki`
* `bicmid_id` column into `Cat` table
* session with `tower-sessions`

### Changed
* updated crate: browserinfocm(0.1.11)
* Finely tuned the `loading view` using `async_sleep()`
* migrated from `rusqlite` to `sqlx with sqlite`

### Fixed
* If an error occurs in `reqwest::get()` in `cat search`, `Loading` remains displayed.

## [0.1.2] (2026-01-04)
### Changed
* updated crate: browserinfocm(0.1.10)
* rename the repository name: `broinfo` to `webapp-broinfo`

## [0.1.1] (2025-12-29)
### Added
* `version`
* `browserinfocm`
* backend api signature
* `patched` by using `patch-crate`
* supports of `cargo deb`
* `base_dir` of database file
* page navigations on the favorites view

### Changed
* rename the repository name: `cat_tongue` to `webapp-cattongue`
* to use `home_dir()` function
* updated: dioxus(0.7.1)
* css: `cursor` of button to `pointer`
* deb config

## [0.1.0] (2025-10-22)
### Added
* first commit

[Unreleased]: https://github.com/aki-akaguma/cat_tongue/compare/v0.1.3..HEAD
[0.1.3]: https://github.com/aki-akaguma/cat_tongue/compare/v0.1.2..v0.1.3
[0.1.2]: https://github.com/aki-akaguma/cat_tongue/compare/v0.1.1..v0.1.2
[0.1.1]: https://github.com/aki-akaguma/cat_tongue/compare/v0.1.0..v0.1.1
[0.1.0]: https://github.com/aki-akaguma/cat_tongue/releases/tag/v0.1.0
