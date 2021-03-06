# Contribution guidelines

First off, thank you for considering contributing to ipld-schema.

If your contribution is not straightforward, please first discuss the change you
wish to make by creating a new issue before making the change.

## Reporting issues

Before reporting an issue on the
[issue tracker](https://github.com/mx00s/ipld-schema/issues),
please check that it has not already been reported by searching for some related
keywords.

## Pull requests

Try to do one pull request per change.

### Updating the changelog

Update the changes you have made in
[CHANGELOG](https://github.com/mx00s/ipld-schema/blob/master/CHANGELOG.md)
file under the **Unreleased** section.

Add the changes of your pull request to one of the following subsections,
depending on the types of changes defined by
[Keep a changelog](https://keepachangelog.com/en/1.0.0/):

- `Added` for new features.
- `Changed` for changes in existing functionality.
- `Deprecated` for soon-to-be removed features.
- `Removed` for now removed features.
- `Fixed` for any bug fixes.
- `Security` in case of vulnerabilities.

If the required subsection does not exist yet under **Unreleased**, create it!

## Developing

### Set up

This is no different than other Rust projects.

```shell
git clone https://github.com/mx00s/ipld-schema
cd ipld-schema
cargo build
```

Tests depend on the pinned gitsubmodule. Be sure to run the following before expecting `cargo test` to work.

```shell
git submodule update --init --recursive
```

### Useful Commands

Consult the convenient cargo aliases in [`config.toml`](./.cargo/config.toml). During development/testing `cargo w` can watch for regressions. Before pushing changes `cargo p` can catch many regressions that may otherwise arise during continuous integration (CI).
