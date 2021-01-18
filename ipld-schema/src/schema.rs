use std::{fmt, str::FromStr};

ipld_schema_macros::import_schema!("../specs/schemas/schema-schema.ipldsch");

impl Schema {
    #[must_use]
    pub fn schema_schema() -> Self {
        std::include_str!("../../specs/schemas/schema-schema.ipldsch")
            .parse()
            .unwrap()
    }

    #[must_use]
    pub fn from_seed(seed: [u8; 32]) -> Self {
        let config = proptest::test_runner::Config::default();
        let rng = proptest::test_runner::TestRng::from_seed(
            proptest::test_runner::RngAlgorithm::ChaCha,
            &seed,
        );
        let mut runner = proptest::test_runner::TestRunner::new_with_rng(config, rng);

        Self::arbitrary().new_tree(&mut runner).unwrap().current()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use insta::assert_debug_snapshot;

    fn schema_schema_json() -> String {
        std::fs::read_to_string("../specs/schemas/schema-schema.ipldsch.json").unwrap()
    }

    #[test]
    #[ignore = "need to implement/port DSL parser first"]
    fn snapshot_of_parsed_schema_schema() {
        assert_debug_snapshot!(Schema::schema_schema());
    }

    #[test]
    #[ignore = "need to implement/port DSL parser first"]
    fn reified_form_of_schema_schema_matches_parsed_dsl_form() {
        assert_eq!(
            Schema::schema_schema(),
            serde_json::from_str(&schema_schema_json()).unwrap()
        );
    }
}
