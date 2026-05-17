use crate::bundle::TestBundle;
use crate::case::{ExpectedBehavior, ScorerKind, TestCase};

/// A 0.0..=1.0 score; 0 = total failure, 1 = perfect.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Score(pub f32);

impl Score {
    pub const ZERO: Score = Score(0.0);
    pub const PERFECT: Score = Score(1.0);
}

/// Score a single test case against a response.
///
/// M3: only the `Substring` scorer is implemented. Others are M4.
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
        ScorerKind::Regex => todo!("M4 impl: regex scorer"),
        ScorerKind::ExactJson => todo!("M4 impl: exact-json scorer"),
        ScorerKind::Caller => todo!("M4 impl: caller-provided scorer"),
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
