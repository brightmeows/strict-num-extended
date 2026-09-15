# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
## [0.5.1](https://github.com/brightmeows/strict-num-extended/compare/strict-num-extended-macros-v0.5.0...strict-num-extended-macros-v0.5.1) - 2026-09-15

### Features
- **(lint)** sync clippy config ([#97](https://github.com/MiyakoMeow/strict-num-extended/pull/97))

## [0.5.0](https://github.com/MiyakoMeow/strict-num-extended/compare/strict-num-extended-macros-v0.4.3...strict-num-extended-macros-v0.5.0) - 2026-01-12

### Bug Fixes
- no hardcode macro functions ([#85](https://github.com/MiyakoMeow/strict-num-extended/pull/85))


### Features
- PiRounded ([#84](https://github.com/MiyakoMeow/strict-num-extended/pull/84))
- [**breaking**] rename types ([#80](https://github.com/MiyakoMeow/strict-num-extended/pull/80))


### Other
- type alias ([#83](https://github.com/MiyakoMeow/strict-num-extended/pull/83))
- constants ([#82](https://github.com/MiyakoMeow/strict-num-extended/pull/82))

## [0.4.3](https://github.com/MiyakoMeow/strict-num-extended/compare/strict-num-extended-macros-v0.4.2...strict-num-extended-macros-v0.4.3) - 2026-01-11

### Chore
- split modules ([#78](https://github.com/MiyakoMeow/strict-num-extended/pull/78))


### Features
- raw float arithmetic ([#76](https://github.com/MiyakoMeow/strict-num-extended/pull/76))

## [0.4.2](https://github.com/MiyakoMeow/strict-num-extended/compare/strict-num-extended-macros-v0.4.1...strict-num-extended-macros-v0.4.2) - 2026-01-10

### Features
- impl From<FloatError> for ParseFloatError ([#75](https://github.com/MiyakoMeow/strict-num-extended/pull/75))


### Other
- try reopen doc test ([#73](https://github.com/MiyakoMeow/strict-num-extended/pull/73))

## [0.4.1](https://github.com/MiyakoMeow/strict-num-extended/compare/strict-num-extended-macros-v0.4.0...strict-num-extended-macros-v0.4.1) - 2026-01-08

### Features
- impl FromStr ([#69](https://github.com/MiyakoMeow/strict-num-extended/pull/69))
- enhance no_std ([#66](https://github.com/MiyakoMeow/strict-num-extended/pull/66))
- impl no_std ([#62](https://github.com/MiyakoMeow/strict-num-extended/pull/62))


### Other
- FiniteFloat trait ([#68](https://github.com/MiyakoMeow/strict-num-extended/pull/68))

## [0.4.0](https://github.com/MiyakoMeow/strict-num-extended/compare/strict-num-extended-macros-v0.3.1...strict-num-extended-macros-v0.4.0) - 2026-01-08

### Bug Fixes
- **(rustdoc)** remove or hide inner type ([#60](https://github.com/MiyakoMeow/strict-num-extended/pull/60))


### Features
- auto-gen doc & doc-test ([#61](https://github.com/MiyakoMeow/strict-num-extended/pull/61))
- [**breaking**] reimpl f32/f64 conversion ([#56](https://github.com/MiyakoMeow/strict-num-extended/pull/56))
- split types ([#53](https://github.com/MiyakoMeow/strict-num-extended/pull/53))


### Other
- dead code ([#58](https://github.com/MiyakoMeow/strict-num-extended/pull/58))
- .value() ([#55](https://github.com/MiyakoMeow/strict-num-extended/pull/55))

## [0.3.1](https://github.com/MiyakoMeow/strict-num-extended/compare/strict-num-extended-macros-v0.3.0...strict-num-extended-macros-v0.3.1) - 2026-01-07

### Features
- impl serde feature ([#50](https://github.com/MiyakoMeow/strict-num-extended/pull/50))

## [0.3.0](https://github.com/MiyakoMeow/strict-num-extended/compare/strict-num-extended-macros-v0.2.0...strict-num-extended-macros-v0.3.0) - 2026-01-07

### Bug Fixes
- rearrange duplicate logic ([#48](https://github.com/MiyakoMeow/strict-num-extended/pull/48))
- rm extra doc tests ([#47](https://github.com/MiyakoMeow/strict-num-extended/pull/47))
- doc warning ([#43](https://github.com/MiyakoMeow/strict-num-extended/pull/43))
- enhance unsafe checking ([#41](https://github.com/MiyakoMeow/strict-num-extended/pull/41))
- **(test)** type judge ([#34](https://github.com/MiyakoMeow/strict-num-extended/pull/34))
- clippy ([#30](https://github.com/MiyakoMeow/strict-num-extended/pull/30))
- simplify range def ([#27](https://github.com/MiyakoMeow/strict-num-extended/pull/27))


### Features
- impl sin/cos/tan & move DivideZero into NaN ([#49](https://github.com/MiyakoMeow/strict-num-extended/pull/49))
- impl From/TryFrom convertion ([#39](https://github.com/MiyakoMeow/strict-num-extended/pull/39))
- option arithmetic ([#38](https://github.com/MiyakoMeow/strict-num-extended/pull/38))
- result arithmetic ([#37](https://github.com/MiyakoMeow/strict-num-extended/pull/37))
- [**breaking**] modulize & more impl & rm trait & impl result ([#36](https://github.com/MiyakoMeow/strict-num-extended/pull/36))
- [**breaking**] impl type-safe calculate ([#32](https://github.com/MiyakoMeow/strict-num-extended/pull/32))


### Other
- unary ops (abs/sig) ([#46](https://github.com/MiyakoMeow/strict-num-extended/pull/46))
- Neg implement ([#29](https://github.com/MiyakoMeow/strict-num-extended/pull/29))


### Refactoring
- re-impl float type ([#35](https://github.com/MiyakoMeow/strict-num-extended/pull/35))

## [0.2.0](https://github.com/MiyakoMeow/strict-num-extended/compare/strict-num-extended-macros-v0.1.1...strict-num-extended-macros-v0.2.0) - 2026-01-05

### Bug Fixes
- supports only finite type ([#22](https://github.com/MiyakoMeow/strict-num-extended/pull/22))


### Chore
- translate ([#24](https://github.com/MiyakoMeow/strict-num-extended/pull/24))


### Other
- module README ([#26](https://github.com/MiyakoMeow/strict-num-extended/pull/26))

## [0.1.1](https://github.com/MiyakoMeow/strict-num-extended/compare/strict-num-extended-macros-v0.1.0...strict-num-extended-macros-v0.1.1) - 2026-01-04

### Other
- NegativeNormalized ([#18](https://github.com/MiyakoMeow/strict-num-extended/pull/18))

## [0.1.0](https://github.com/MiyakoMeow/strict-num-extended/releases/tag/strict-num-extended-macros-v0.1.0) - 2026-01-03

### Bug Fixes
- **(ci)** format & release-please manifest ([#2](https://github.com/MiyakoMeow/strict-num-extended/pull/2))
- clippy warnings


### Chore
- **(release-please)** simplify release please config ([#7](https://github.com/MiyakoMeow/strict-num-extended/pull/7))
- full workspace config ([#5](https://github.com/MiyakoMeow/strict-num-extended/pull/5))
- split modules
- translate


### Features
- crates.io publish config ([#15](https://github.com/MiyakoMeow/strict-num-extended/pull/15))
- **(ci)** init


### Other
- release please ([#12](https://github.com/MiyakoMeow/strict-num-extended/pull/12))
- first version

