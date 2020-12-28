# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Schema types based on submodule-pinned copy of IPDL's [schema-schema](./specs/schemas/schema-schema.ipldsch).
- Partial DSL parser that can read the schema-schema.
- Serde deserializers to read IPDL's [reified JSON form of the schema-schema](./specs/schemas/schema-schema.ipldsch.json) and a test verifying correspondence to parsed DSL form.
