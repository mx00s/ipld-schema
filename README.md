# ipld-schema

[![Crates.io](https://img.shields.io/crates/v/ipld-schema.svg)](https://crates.io/crates/ipld-schema)
[![Docs.rs](https://docs.rs/ipld-schema/badge.svg)](https://docs.rs/ipld-schema)
[![CI](https://github.com/mx00s/ipld-schema/workflows/Continuous%20Integration/badge.svg)](https://github.com/mx00s/ipld-schema/actions)
[![Coverage Status](https://coveralls.io/repos/github/mx00s/ipld-schema/badge.svg?branch=main)](https://coveralls.io/github/mx00s/ipld-schema?branch=main)

_Generate and validate [IPLD Schemas](https://specs.ipld.io/schemas/) and the data they describe_

`ipld-schema` can be used both as a command-line application and a Rust library.

## Status

The project is under active development and interfaces may change. Details are tracked in the [changelog](CHANGELOG.md) for each [semver](https://semver.org/) release.

For pre-1.0.0 releases any known breakage in the determinism of schema/data generation will incur a minor version bump. After the 1.0.0 release such breakage will incur major bumps. Version and seed information is included as comment headers in generated outputs.

[IPLD's specification](https://github.com/ipld/specs) is continually evolving. To cope with this, at least during this early stage of development, an revision is pinned using a gitsubmodule at [`./specs`](./specs). In cases where the latest revision of the upstream repository and the pinned version are inconsistent this repository will generally prefer what's specified in the pinned version. As the project matures the pinned specification version may be bumped.

The public Rust API may not expose enough to be useful; however, this will be addressed after more core features are implemented.

For now the minimum supported Rust version (MSRV) is 1.40.0.

## Installation

Versioned binary releases for Linux, macOS, and Windows are available [here](https://github.com/mx00s/ipld-schema/releases). The binary can also be installed using [`cargo`](https://github.com/rust-lang/cargo/), which can be conveniently installed and managed using [`rustup`](https://rustup.rs/).

```shell
$ cargo install ipld-schema
```

## Example Usage

### Generate a new schema

```shell
$ ipld-schema generate
##
## Deterministically generated with ipld-schema 0.3.2
##
##   - reproduction seed: 'gHvBv/QtyFqCo5SeeAaIS7vGtomE1fRIl0O2HXAPH2Y='
##

type Kvq9__ bytes
...
```

### Determinstically generate a schema from a seed

```shell
$ ipld-schema generate --seed gHvBv/QtyFqCo5SeeAaIS7vGtomE1fRIl0O2HXAPH2Y=
##
## Deterministically generated with ipld-schema 0.3.2
##
##   - reproduction seed: 'gHvBv/QtyFqCo5SeeAaIS7vGtomE1fRIl0O2HXAPH2Y='
##

type Kvq9__ bytes
...
```

### Validate a schema

No output means it's considered valid.

```shell
$ ipld-schema validate my-schema.ipldsch
$
```

Beware, the validator does not yet check everything. If you encounter schema it classifies incorrectly please file a bug with a minimal schema demonstrating the problem along with relevant context from the [pinned specification](./specs) to support your case.

### TODO: Generate data conforming to a specified schema

```shell
$ ipld-schema generate my-schema.ipldsch
thread 'main' panicked at 'not yet implemented: generate data using seed 'EHVBvPdE6tDWMdCGkHrsf6zZQqIHZbBLrKJSqtBgsG0=' and schema "my-schema.ipldsch"' ...
##
## Deterministically generated with ipld-schema 0.3.2
##
##   - reproduction seed: 'EHVBvPdE6tDWMdCGkHrsf6zZQqIHZbBLrKJSqtBgsG0='
##   - schema file: "my-schema.ipldsch"
##
```

### TODO: Validate data conforms to a schema

```shell
$ validate my-schema.ipldsch my-data.json
thread 'main' panicked at 'not yet implemented: validate data ("my-data.json") using schema ("my-schema.ipldsch")' ...
```

## Features

- [ ] DSL
  - [x] Parse the [schema-schema](./specs/schemas/schema-schema.ipldsch)
- [ ] Reified Form
  - [x] Convert parsed schema-schema to its [reified JSON form](./specs/schemas/schema-schema.ipldsch.json)
  - [x] Convert reified form of the schema-schema back to its DSL representation (sans comments)
  - [x] Generate arbitrary IPLD schemas in reified form from the parsed schema-schema
  - [ ] Validate IPLD schema in reified form against the schema-schema and additional constraints (e.g. "rules around valid characters for type names")
  - [ ] Generate Rust types from a valid IPLD schema
  - [ ] Test generated IPLD schemas against implementations in other languages
  - [ ] Parse IPLD values into Rust types generated from a compatible schema  
  - [ ] Generate arbitrary IPLD values from the reified form of some IPLD schema
  - [ ] Verify generated IPLD values roundtrip through parsing to Rust types which were generated from the IPLD schema

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

See [CONTRIBUTING.md](CONTRIBUTING.md).
