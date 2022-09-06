# Changelog

This project follows semantic versioning.

Possible log types:

- `[added]` for new features.
- `[changed]` for changes in existing functionality.
- `[deprecated]` for once-stable features removed in upcoming releases.
- `[removed]` for deprecated features removed in this release.
- `[fixed]` for any bug fixes.
- `[security]` to invite users to upgrade in case of vulnerabilities.

### v0.4.0 (2022-09-06)
- `[added]` a binary for interacting with the Canvas API.

### v0.3.0 (2022-01-14)
- `[changed]` forward errors to the outside of the crate.

### v0.2.1 (2020-09-04)
- `[added]` `blocking` feature. This disables the asynchronous calls.

### v0.2.0 (2020-09-01)
- `[changed]` `CanvasInformation` now accepts `&str` instead of `String`.
- `[added]` Documentation is added to the library.
