use std::{fmt, str::FromStr};

ipld_schema_macros::import_schema!("../specs/schemas/schema-schema.ipldsch");

impl Schema {
    pub fn schema_schema() -> Self {
        std::include_str!("../../specs/schemas/schema-schema.ipldsch")
            .parse()
            .unwrap()
    }
}

impl FromStr for Schema {
    type Err = ();

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        todo!("implement DSL parser for generated types")
    }
}

impl fmt::Display for Schema {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use insta::assert_debug_snapshot;
    use pretty_assertions::assert_eq;
    use test_strategy::proptest;

    #[cfg(feature = "fast-test")]
    const CASES: u32 = 10;
    #[cfg(not(feature = "fast-test"))]
    const CASES: u32 = 1000;

    #[cfg(feature = "fast-test")]
    const MAX_SHRINK_ITERS: u32 = 100;
    #[cfg(not(feature = "fast-test"))]
    const MAX_SHRINK_ITERS: u32 = 10000;

    fn schema_schema_json() -> String {
        std::fs::read_to_string("../specs/schemas/schema-schema.ipldsch.json").unwrap()
    }

    fn schema_roundtrips_through_json(schema: &Schema) {
        assert_eq!(
            *schema,
            serde_json::from_str(&serde_json::to_string(schema).unwrap()).unwrap()
        );
    }

    fn schema_roundtrips_through_dsl(schema: &Schema) {
        let rendered = schema.to_string();

        for (n, line) in rendered.lines().enumerate() {
            eprintln!("  {:>4}  │ {}", n + 1, line);
        }

        assert_eq!(*schema, rendered.parse::<Schema>().unwrap());
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

    #[test]
    #[ignore = "need to implement/port DSL parser first"]
    fn schema_schema_roundtrips_through_parsing_and_display() {
        schema_roundtrips_through_dsl(&Schema::schema_schema());
    }

    #[proptest(cases = CASES, max_shrink_iters = MAX_SHRINK_ITERS)]
    #[ignore = "causes stack overflow; need to tune proptest strategies"]
    fn roundtrips_through_dsl_form(schema: Schema) {
        schema_roundtrips_through_dsl(&schema);
    }

    #[proptest(cases = CASES, max_shrink_iters = MAX_SHRINK_ITERS)]
    #[ignore = "causes stack overflow; need to tune proptest strategies"]
    fn roundtrips_through_json_form(schema: Schema) {
        schema_roundtrips_through_json(&schema);
    }
}
