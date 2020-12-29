# ipld-schema

[![Crates.io](https://img.shields.io/crates/v/ipld-schema.svg)](https://crates.io/crates/ipld-schema)
[![Docs.rs](https://docs.rs/ipld-schema/badge.svg)](https://docs.rs/ipld-schema)
[![CI](https://github.com/mx00s/ipld-schema/workflows/Continuous%20Integration/badge.svg)](https://github.com/mx00s/ipld-schema/actions)
[![Coverage Status](https://coveralls.io/repos/github/mx00s/ipld-schema/badge.svg?branch=main)](https://coveralls.io/github/mx00s/ipld-schema?branch=main)

## Features

- [ ] DSL
  - [x] Parse the [schema-schema](./specs/schemas/schema-schema.ipldsch)
- [ ] Reified Form
  - [x] Convert parsed schema-schema to its [reified JSON form](./specs/schemas/schema-schema.ipldsch.json)
  - [x] Convert reified form of the schema-schema back to its DSL representation (sans comments)
  - [ ] Generate arbitrary IPLD schemas in reified form from the parsed schema-schema
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
