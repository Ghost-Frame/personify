use crate::bundle::TestBundle;
use crate::case::{ExpectedBehavior, ScorerKind, TestCase};

/// A 0.0..=1.0 score; 0 = total failure, 1 = perfect.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Score(pub f32);

impl Score {
    /// The lowest possible score: the response completely failed the test.
    pub const ZERO: Score = Score(0.0);
    /// The highest possible score: the response perfectly satisfied the test.
    pub const PERFECT: Score = Score(1.0);
}

/// Score a single test case against a response string.
///
/// Returns [`Score::PERFECT`] when the response satisfies the expected behavior,
/// and [`Score::ZERO`] for any mismatch, parse failure, or unsupported pairing.
/// The `Caller` variant always returns [`Score::ZERO`] here -- use
/// [`crate::caller::score_bundle_with_caller`] to delegate those cases to a
/// [`crate::caller::CallerScorer`] implementation.
pub fn score_test(test: &TestCase, response: &str) -> Score {
    match test.scorer {
        ScorerKind::Substring => match &test.expected {
            ExpectedBehavior::Contains { value } => {
                if response.contains(value.as_str()) {
                    Score::PERFECT
                } else {
                    Score::ZERO
                }
            }
            _ => Score::ZERO,
        },
        ScorerKind::Regex => match &test.expected {
            ExpectedBehavior::Matches { pattern } => {
                match regex::Regex::new(pattern) {
                    Ok(re) => {
                        if re.is_match(response) {
                            Score::PERFECT
                        } else {
                            Score::ZERO
                        }
                    }
                    Err(_) => {
                        tracing::warn!(
                            pattern = %pattern,
                            "invalid regex in test case {}",
                            test.id
                        );
                        Score::ZERO
                    }
                }
            }
            _ => Score::ZERO,
        },
        ScorerKind::ExactJson => match &test.expected {
            ExpectedBehavior::JsonShape { shape } => {
                match serde_json::from_str::<serde_json::Value>(response) {
                    Ok(parsed) => {
                        if parsed == *shape {
                            Score::PERFECT
                        } else {
                            Score::ZERO
                        }
                    }
                    Err(_) => Score::ZERO,
                }
            }
            _ => Score::ZERO,
        },
        ScorerKind::Caller => {
            tracing::warn!(
                test_id = %test.id,
                "score_test called with Caller scorer; returning ZERO -- use score_bundle_with_caller instead"
            );
            Score::ZERO
        }
    }
}

/// Average per-test score across the bundle. Empty bundles score 0.
pub fn bundle_score(_bundle: &TestBundle, results: &[(TestCase, String)]) -> Score {
    if results.is_empty() {
        return Score::ZERO;
    }
    let total: f32 = results
        .iter()
        .map(|(case, response)| score_test(case, response).0)
        .sum();
    Score(total / results.len() as f32)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bundle::TestBundle;
    use crate::caller::{score_bundle_with_caller, CallerScorer};
    use crate::case::{ExpectedBehavior, ScorerKind, TestCase};

    /// Build a minimal TestCase with the given id, expected behavior, and scorer.
    fn make_case(id: &str, expected: ExpectedBehavior, scorer: ScorerKind) -> TestCase {
        TestCase {
            id: id.to_string(),
            prompt: "prompt".to_string(),
            expected,
            scorer,
        }
    }

    /// Build a minimal TestBundle for use in bundle-level tests.
    fn make_bundle() -> TestBundle {
        TestBundle {
            name: "test".to_string(),
            version: "0.1.0".to_string(),
            tests: vec![],
        }
    }

    // -- Regex scorer tests --

    #[test]
    /// Regex that appears as a substring of the response scores PERFECT.
    fn regex_scorer_matches_substring() {
        let case = make_case(
            "t1",
            ExpectedBehavior::Matches { pattern: "hello".to_string() },
            ScorerKind::Regex,
        );
        assert_eq!(score_test(&case, "say hello"), Score::PERFECT);
    }

    #[test]
    /// Regex that does not match any part of the response scores ZERO.
    fn regex_scorer_no_match() {
        let case = make_case(
            "t2",
            ExpectedBehavior::Matches { pattern: "goodbye".to_string() },
            ScorerKind::Regex,
        );
        assert_eq!(score_test(&case, "hello"), Score::ZERO);
    }

    #[test]
    /// Anchored regex `^hello` does not match "say hello" (anchor blocks it).
    fn regex_scorer_anchored() {
        let case = make_case(
            "t3",
            ExpectedBehavior::Matches { pattern: "^hello".to_string() },
            ScorerKind::Regex,
        );
        assert_eq!(score_test(&case, "say hello"), Score::ZERO);
    }

    #[test]
    /// An invalid regex pattern scores ZERO without panicking.
    fn regex_scorer_invalid_pattern() {
        let case = make_case(
            "t4",
            ExpectedBehavior::Matches { pattern: "[unclosed".to_string() },
            ScorerKind::Regex,
        );
        assert_eq!(score_test(&case, "anything"), Score::ZERO);
    }

    // -- ExactJson scorer tests --

    #[test]
    /// Response JSON that is byte-for-byte equal to the shape scores PERFECT.
    fn exact_json_equal() {
        let shape: serde_json::Value = serde_json::json!({"x": 1});
        let case = make_case(
            "j1",
            ExpectedBehavior::JsonShape { shape },
            ScorerKind::ExactJson,
        );
        assert_eq!(score_test(&case, r#"{"x":1}"#), Score::PERFECT);
    }

    #[test]
    /// Response JSON that differs from the shape scores ZERO.
    fn exact_json_not_equal() {
        let shape: serde_json::Value = serde_json::json!({"x": 1});
        let case = make_case(
            "j2",
            ExpectedBehavior::JsonShape { shape },
            ScorerKind::ExactJson,
        );
        assert_eq!(score_test(&case, r#"{"x":2}"#), Score::ZERO);
    }

    #[test]
    /// A response that is not valid JSON scores ZERO.
    fn exact_json_invalid_response() {
        let shape: serde_json::Value = serde_json::json!({"x": 1});
        let case = make_case(
            "j3",
            ExpectedBehavior::JsonShape { shape },
            ScorerKind::ExactJson,
        );
        assert_eq!(score_test(&case, "not json"), Score::ZERO);
    }

    #[test]
    /// A JSON number response does not match a JSON object shape.
    fn exact_json_type_mismatch() {
        let shape: serde_json::Value = serde_json::json!({"x": 1});
        let case = make_case(
            "j4",
            ExpectedBehavior::JsonShape { shape },
            ScorerKind::ExactJson,
        );
        assert_eq!(score_test(&case, "1"), Score::ZERO);
    }

    // -- Caller scorer tests --

    #[test]
    /// score_test with ScorerKind::Caller always returns ZERO.
    fn caller_returns_zero_in_score_test() {
        let case = make_case(
            "c1",
            ExpectedBehavior::Custom { id: "my-judge".to_string() },
            ScorerKind::Caller,
        );
        assert_eq!(score_test(&case, "any response"), Score::ZERO);
    }

    /// A mock CallerScorer that always returns PERFECT for use in tests.
    struct AlwaysPerfect;

    impl CallerScorer for AlwaysPerfect {
        /// Always returns PERFECT regardless of the test case or response.
        fn score(&self, _test: &TestCase, _response: &str) -> Score {
            Score::PERFECT
        }
    }

    #[test]
    /// score_bundle_with_caller delegates Caller cases to the CallerScorer.
    fn bundle_with_caller_trait() {
        let bundle = make_bundle();
        let case = make_case(
            "c2",
            ExpectedBehavior::Custom { id: "judge".to_string() },
            ScorerKind::Caller,
        );
        let results = vec![(case, "response".to_string())];
        let scorer = AlwaysPerfect;
        let score = score_bundle_with_caller(&bundle, &results, &scorer);
        assert_eq!(score, Score::PERFECT);
    }
}
