# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).


## [Unreleased](https://github.com/pack-it/binspect/compare/0.0.1...HEAD)

### Added
- Symlink traversal is now shown to the user.
- The `sign` command, which signs a MachO binary.

### Fixed
- When MachO binaries containing multiple targets are changed, all targets are preserved.
- MachO binaries are now signed after changing when running on macOS.


## [v0.0.1](https://github.com/pack-it/binspect/releases/tag/0.0.1) - 2026-07-23

First release of Binspect, consisting of simple inspect and change commands for `ELF`, `MachO` and `PE` binaries.
