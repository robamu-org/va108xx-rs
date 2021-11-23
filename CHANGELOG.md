Change Log
=======

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [unreleased]

## [v0.1.3]

### Added

- Added two missing bit fields for I2CA STATUS register: I2CIDLE and IDLE

### Fixed

- Made I2CA STATUS register read-only

## [v0.1.2]

### Fixed

- Generated with patched version of `svd2rust`: See 
  https://github.com/rust-embedded/svd2rust/pull/549 for more details.
  Some bitmasks were missing from register reader definitions.

## [v0.1.1]

- Relicensed under dual Apache-2.0 / MIT license

### Changed

- SVD file handling improved and new fields added for the peripheral
  clock enable register

### Added

- Helper script to automate all steps for PAC generation
- Added badges for README

## [v0.1.0]

### Added

- First version of the PAC which builds. Uses a patched version
  of `svd2rust`: https://github.com/rust-embedded/svd2rust
