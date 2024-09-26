# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0] - 2024-09-26
### :sparkles: New Features
- [`e8c465d`](https://github.com/kcx1/qplug/commit/e8c465dd3349dedbdef2bcd0d56fc9095c4ba9e2) - Enable users to check version, is_qplug, and config
- [`74dd942`](https://github.com/kcx1/qplug/commit/74dd9428f2505091dfe5de09ee7628955af98154) - Implement config check function. Returns path of found config
- [`8ae032a`](https://github.com/kcx1/qplug/commit/8ae032a201cf6d4b355a2779fc9702e9dc66816b) - [#6](https://github.com/kcx1/qplug/pull/6) - Augment global config with local config
- [`f737d38`](https://github.com/kcx1/qplug/commit/f737d387f16de130a33d1c311faa6dfa54cb8351) - Add small api for user config

### :bug: Bug Fixes
- [`8789668`](https://github.com/kcx1/qplug/commit/8789668cdf543b00dca88943e4845912a6f6ea6c) - Update the release name
- [`9ba4331`](https://github.com/kcx1/qplug/commit/9ba43310ccc746bb69f9fa6100af424d14497b08) - Properly loads user config file into the lua environment
- [`7c8a3aa`](https://github.com/kcx1/qplug/commit/7c8a3aaf03d845c0c4bdb141fa121bf593ac05e3) - [#9](https://github.com/kcx1/qplug/pull/9) - build now uses user config

### :recycle: Refactors
- [`1039f05`](https://github.com/kcx1/qplug/commit/1039f05d9cc95cef0fd0d83d94a28e02ae4f7006) - Move user config and lua env into a UserEnv struct
- [`4f14c64`](https://github.com/kcx1/qplug/commit/4f14c64a8bf1d626dc55bae02efdbb1171586aee) - Cleanup function naming


## [0.2.2] - 2024-09-23
### :bug: Bug Fixes
- [`2f2b15a`](https://github.com/kcx1/qplug/commit/2f2b15a040bae1ffa23772be383a07f27eddcdb0) - GH Actions
- [`69aff61`](https://github.com/kcx1/qplug/commit/69aff61ed414f497aed6a6ef4b9d82e444a80fe1) - Changelog generation for Release

[0.2.2]: https://github.com/kcx1/qplug/compare/0.2.1...0.2.2
[0.3.0]: https://github.com/kcx1/qplug/compare/0.2.2...0.3.0
