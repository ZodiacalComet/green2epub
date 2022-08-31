# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!--
## [Unreleased]

### Added
### Changed
### Deprecated
### Removed
### Fixed
### Security
-->

## [Unreleased]

### Changed
- Avoid creating an EPUB without any text files to convert.
- Check if `COLOR` argument on `--green-color` and `--spoiler-color`
  are a valid RGB hex color prior to using them in the CSS stylesheet
  and give an explicative error message in the case that they are not.
- Descriptions for `--green-color` and `--spoiler-color` in help and
  in completions now mention that they expect an RGB color in
  hexadecimal notation.

### Fix
- Binary for `i686-linux` build was not being stripped of object file
  symbols, resulting in a way bigger binary when compared to the rest
  of the linux builds.

## [v0.1.0] - 2022-01-28

### Added
- Application generates EPUBs from text files in greentext format.
- Subjects and a cover can be added to them. Even the colors from the
  green highlight and spoiler can be changed, if you are so inclined.
- Has shell completions for bash, elvish, fish, powershell and zsh.
- README explains what this application does, how it can be used with
  an elaborate example and gives installation instructions.

[Unreleased]: https://github.com/ZodiacalComet/green2epub/compare/v0.1.0...HEAD
[v0.1.0]: https://github.com/ZodiacalComet/green2epub/releases/tag/v0.1.0