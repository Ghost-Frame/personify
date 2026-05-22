//! Caller-provided scorer extension point for the conformance harness.
//!
//! When a [`crate::case::ScorerKind::Caller`] test case is encountered,
//! standard scoring logic cannot handle it -- the meaning is defined entirely
//! by the caller (e.g. an LLM-judge plugin or integration test harness).
//! This module provides the [`CallerScorer`] trait and
//! [`score_bundle_with_caller`], which threads a caller-provided implementation
//! through bundle scoring so Caller cases are handled correctly.

use crate::bundle::TestBundle;
use crate::case::{ScorerKind, TestCase};
use crate::score::{score_test, Score};

/// A caller-provided scorer for [`ScorerKind::Caller`] test cases.
///
/// Implement this trait to supply custom scoring logic (e.g. an LLM-judge,
/// a semantic similarity metric, or a structured output validator) for test
/// cases that cannot be evaluated by the built-in substring, regex, or
/// exact-JSON strategies.
pub trait CallerScorer: Send + Sync {
    /// Score a single [`TestCase`] against a response string.
    ///
    /// Called only when `test.scorer == ScorerKind::Caller`. The return value
    /// must be in `0.0..=1.0`; values outside that range are clamped by the
    /// bundle aggregation layer.
    fn score(&self, test: &TestCase, response: &str) -> Score;
}

/// Score a [`TestBundle`] using standard scorers for non-Caller cases and
/// a [`CallerScorer`] implementation for [`ScorerKind::Caller`] cases.
///
/// `results` is a slice of `(TestCase, response_string)` pairs produced by
/// a runner. Standard cases are delegated to [`score_test`]; Caller cases are
/// delegated to `caller`. The final [`Score`] is the per-test average, matching
/// the semantics of [`bundle_score`].
pub fn score_bundle_with_caller(
    bundle: &TestBundle,
    results: &[(TestCase, String)],
    caller: &dyn CallerScorer,
) -> Score {
    if results.is_empty() {
        return Score::ZERO;
    }
    let total: f32 = results
        .iter()
        .map(|(case, response)| {
            if case.scorer == ScorerKind::Caller {
                caller.score(case, response)
            } else {
                score_test(case, response)
            }
        })
        .map(|s| s.0)
        .sum();
    // Re-use the same averaging formula as bundle_score to stay consistent.
    let _ = bundle; // bundle is accepted for API symmetry with bundle_score
    Score(total / results.len() as f32)
}
