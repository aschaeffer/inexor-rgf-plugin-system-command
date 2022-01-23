# Inexor Reactive Graph Flow

| Project             | Module | Sub-Module     | Functionality                                                        | Tests                                                                                                                                                                  |
|---------------------|--------|----------------|----------------------------------------------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Reactive Graph Flow | Plugin | System Command | <img src="https://img.shields.io/badge/state-completed-brightgreen"> | [<img src="https://img.shields.io/codecov/c/github/aschaeffer/inexor-rgf-plugin-input-device">](https://app.codecov.io/gh/aschaeffer/inexor-rgf-plugin-system-command) |

### About Inexor

<a href="https://inexor.org/">
<img align="right" width="200" height="200" src="https://raw.githubusercontent.com/aschaeffer/inexor-rgf-plugin-system-command/main/docs/images/inexor_2.png">
</a>

* Inexor will be a new first-person shooter game which is based on a new octree-based game engine.
* Inexor focuses on classic gameplay as we've seen in Cube2 or the Quake series.
* Inexor will be written from ground up new in C++17 and Rust.
* You can contribute anything you want: code, content, ideas...
* Inexor and all its content is 100% open source!

### About Inexor Reactive Graph Flow

The Inexor Reactive Graph Flow (RGF) manages reactive flows based on a graph database. The main interface is GraphQL.

* Semantic: Graph database with entities and relationships as first class citizens
* Reactive: entities and relationships are/can be reactive: If the input has been altered the entity processes its new state
* Interoperable: Use GraphQL for queries and mutations
* Extendable: Built in type system: components, entity types and relation types
* Memory efficient: Rust
* Fast: Rust
* Secure: Rust

### About this plugin

This plugin allows executing system commands.

[<img src="https://img.shields.io/badge/Language-Rust-brightgreen">](https://www.rust-lang.org/)
[<img src="https://img.shields.io/badge/Platforms-Linux%20%26%20Windows-brightgreen">]()
[<img src="https://img.shields.io/github/workflow/status/aschaeffer/inexor-rgf-plugin-system-command/Rust">](https://github.com/aschaeffer/inexor-rgf-plugin-system-command/actions?query=workflow%3ARust)
[<img src="https://img.shields.io/github/last-commit/aschaeffer/inexor-rgf-plugin-system-command">]()
[<img src="https://img.shields.io/github/languages/code-size/aschaeffer/inexor-rgf-plugin-system-command">]()
[<img src="https://img.shields.io/codecov/c/github/aschaeffer/inexor-rgf-plugin-system-command">](https://app.codecov.io/gh/aschaeffer/inexor-rgf-plugin-system-command)

[<img src="https://img.shields.io/github/license/aschaeffer/inexor-rgf-plugin-system-command">](https://github.com/aschaeffer/inexor-rgf-plugin-system-command/blob/main/LICENSE)
[<img src="https://img.shields.io/discord/698219248954376256?logo=discord">](https://discord.com/invite/acUW8k7)

#### Platform Compatibility

| Platform | Compatibility |
|----------|---------------|
| Linux    | ✓             |
| MacOS    | ✓             |
| Windows  | ✓             |

#### Type System

<img src="https://raw.githubusercontent.com/aschaeffer/inexor-rgf-plugin-system-command/main/docs/images/type_system.png">

#### Entity Types

| Name          | Property    | Data Type | Socket Type |
|---------------|-------------|-----------|-------------|
| SystemCommand | name        | string    | none        |
|               | current_dir | string    | none        |
|               | command     | string    | none        |
|               | spawn       | array     | input       |
|               | stdin       | string    | input       |
|               | stdout      | string    | output      |
|               | stderr      | string    | output      |

#### Entity Behaviours

| Name          | Description                                                                                     |
|---------------|-------------------------------------------------------------------------------------------------|
| SystemCommand | Executes a command if the stream of the property `spawn` has been triggered with the arguments. |

### Thanks to

* https://github.com/xd009642/tarpaulin
* https://codecov.io/

### Sponsors

|                                                                                                                                                                                                                               |           |                                                                   |
|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-----------|-------------------------------------------------------------------|
| <a href="https://www.jetbrains.com/?from=github.com/inexorgame"><img align="right" width="100" height="100" src="https://raw.githubusercontent.com/aschaeffer/inexor-rgf-plugin-logical/main/docs/images/icon_CLion.svg"></a> | JetBrains | Special thanks to JetBrains for providing us with CLion licenses! |
