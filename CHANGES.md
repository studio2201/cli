# Changelog — studio2201 CLI

All notable changes to the studio2201 unified CLI are documented here.
Format: [Keep a Changelog](https://keepachangelog.com/) 1.1.0.
This project adheres to [Semantic Versioning](https://semver.org/).

## [0.1.4] — 2026-09-19

### Fixed
- Fixed `run_proven` in `runner.rs` to detect release artifacts and attest with ML-DSA-65 or skip gracefully when none found.

## [0.1.1] — 2026-09-19

### Fixed
- Fixed `run_boneyard` scanner handling in `runner.rs`: gracefully skip with `SKIPPED` (exit code 0, passed) when neither `hall.json` nor `catalog.json` exists in the target path.
- Maintained strict <= 256 LOC bound on `runner.rs` (249 LOC).

## [0.1.0] — 2026-09-19

### Added
- Initial release of unified CLI and toolchain manager for studio2201.
- Implemented toolchain verbs (`install`, `remove`, `upgrade`, `list`, `check`, `init`).
- Implemented multi-tool audit suite runner with GitHub Actions step summary scorecard.
