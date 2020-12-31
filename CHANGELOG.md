# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.1] - 2020-12-31
### Fixed
- (Hopefully) got `cargo install ipld-schema` to compile successfully.

## [0.3.0] - 2020-12-31
### Added
- CLI program called `ipld-schema` which can generate and validate schema and the data they describe.
- Public `Opt` type that drives all CLI functionality and enables library users to achieve the same things.
- Schema and data generation are deterministically driven by a 32-byte seed.

### Changed
- Implementations of Arbitrary for schema types are included regardless of specified cargo features.

## [0.2.0] - 2020-12-30
### Added
- Partial DSL renderer that can correctly display the schema-schema.
- Initial generators and property-based tests for roundtripping schemas through their DSL and JSON forms.
- Extensions to parser to support several schema forms which aren't demonstrated in the schema-schema.
- Convenient cargo aliases to improve development and testing workflow.

## Fixed
- Innaccuracies in snapshot of the schema-schema parsed from DSL form and then rendered in JSON form.

## [0.1.1] - 2020-12-28
### Added
- Schema types based on submodule-pinned copy of IPDL's [schema-schema](./specs/schemas/schema-schema.ipldsch).
- Partial DSL parser that can read the schema-schema.
- Serde deserializers to read IPDL's [reified JSON form of the schema-schema](./specs/schemas/schema-schema.ipldsch.json) and a test verifying correspondence to parsed DSL form.

[Unreleased]: https://github.com/mx00s/ipld-schema/compare/0.3.1...HEAD
[0.3.0]: https://github.com/mx00s/ipld-schema/compare/0.3.0...0.3.1
[0.3.0]: https://github.com/mx00s/ipld-schema/compare/0.2.0...0.3.0
[0.2.0]: https://github.com/mx00s/ipld-schema/compare/0.1.1...0.2.0
[0.1.1]: https://github.com/mx00s/ipld-schema/compare/b47846afc50ff594ed144197de35c81142b595bd...0.1.1
