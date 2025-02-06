# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [3.1] - 2025-02-06
### :sparkles: New Features
- [`290920f`](https://github.com/kcx1/qplug/commit/290920f50ef55d46cfca6952755155e1bc42a46f) - Adds ability to specify a path to copy compilted .qplug file into *(commit by [@kcx1](https://github.com/kcx1))*
- [`e96a583`](https://github.com/kcx1/qplug/commit/e96a583c5ac9e2ffeeefde9d9994113c755ca4bb) - Create a build path; allow copy path to be optional *(commit by [@kcx1](https://github.com/kcx1))*
- [`63d4e86`](https://github.com/kcx1/qplug/commit/63d4e86ae92938aa363f6fb19236cfb854f3ca58) - Create a build only flag *(commit by [@kcx1](https://github.com/kcx1))*
- [`e3fe204`](https://github.com/kcx1/qplug/commit/e3fe204969b2b9093fc976a4f0e604e514c79a2f) - Removes the copy command in favor of an option under the build command *(commit by [@kcx1](https://github.com/kcx1))*
- [`a455dba`](https://github.com/kcx1/qplug/commit/a455dbae29243745a1c3bfde1dfdbb3e2cc2c37e) - Remove Build Path as option *(commit by [@kcx1](https://github.com/kcx1))*

### :bug: Bug Fixes
- [`ee5c885`](https://github.com/kcx1/qplug/commit/ee5c8859319063c74d8c32f20d3f63055939f100) - Update packages to prevent vuln *(commit by [@kcx1](https://github.com/kcx1))*
- [`9e520d0`](https://github.com/kcx1/qplug/commit/9e520d0e5b2c2c4317bc33ab326ab23d40cb7a42) - Fixes IDNA vuln *(commit by [@kcx1](https://github.com/kcx1))*

### :wrench: Chores
- [`b0d33dd`](https://github.com/kcx1/qplug/commit/b0d33ddec4b9c83c74a012bfc51554537ee2ac26) - Fix typo *(commit by [@kcx1](https://github.com/kcx1))*
- [`b0a3262`](https://github.com/kcx1/qplug/commit/b0a3262f05dfa1639c87962ccadc2dd0d0e37da2) - Fix Typo *(commit by [@kcx1](https://github.com/kcx1))*
- [`b097a60`](https://github.com/kcx1/qplug/commit/b097a60c6ba1063cd9bc367db2a7998efb56c426) - Update phrasing *(commit by [@kcx1](https://github.com/kcx1))*
- [`4a35d4a`](https://github.com/kcx1/qplug/commit/4a35d4acf1a913dba4fb42b8a35c653ffe76c33b) - Clean up *(commit by [@kcx1](https://github.com/kcx1))*
- [`886cd2e`](https://github.com/kcx1/qplug/commit/886cd2e334ee1307fdb70eaa9ec049df359c2ee3) - Cleanup TODOs *(commit by [@kcx1](https://github.com/kcx1))*
- [`3b4a862`](https://github.com/kcx1/qplug/commit/3b4a86254afabb9ffd2ec6155e7b7251cd06c70a) - Update packages and TODO *(commit by [@kcx1](https://github.com/kcx1))*
- [`878ca27`](https://github.com/kcx1/qplug/commit/878ca27d7276cc56530540f6bd85cbbc2f52c40e) - Cleanup and error files *(commit by [@kcx1](https://github.com/kcx1))*
- [`c172651`](https://github.com/kcx1/qplug/commit/c17265182431a21d4bfb2ec00661905f16e6c226) - Update the pkg version *(commit by [@kcx1](https://github.com/kcx1))*


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
[3.1]: https://github.com/kcx1/qplug/compare/0.3.0...3.1
