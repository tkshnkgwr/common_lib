# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [0.1.0] - 2026-07-03

### Added
- Initial implementation of the utility library.
- Single instance checking functionality using Windows Named Mutex (`check_single_instance`).
- RAII-based single instance checking for desktop applications (`desktop::acquire_single_instance`).
- Text line difference computation (`compute_diff`) using LCS algorithm.
- Word occurrences count utility (`count_occurrences`).
- Added documentation for specifications (`docs/SPEC.md`), system diagrams (`docs/DIAGRAM.md`), and performance footprints (`docs/FOOTPRINTS.md`).
- Added Japanese and English READMEs.
- Custom instructions for AI agents (`.agents/AGENTS.md`).
