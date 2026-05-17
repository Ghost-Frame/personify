use crate::score::Score;
use frameshift_pack::ConformanceBaseline;

/// Decision returned by [`RegressionGate::evaluate_upgrade`].
#[derive(Debug, Clone, PartialEq)]
pub enum GateDecision {
    /// Upgrade clears the baseline.
    Pass,
    /// New score is below the old baseline by `delta` (positive value).
    FailRegression { delta: f32 },
    /// Bundle hash changed, so the baseline cannot be compared directly.
    FailBundleChanged,
}

/// Stateless evaluator. The runtime constructs one per upgrade attempt.
pub struct RegressionGate;

impl RegressionGate {
    /// Compare a new run's score against the baseline shipped with the
    /// previous pack version.
    ///
    /// Rules:
    /// 1. If the bundle hash changed, fail with [`GateDecision::FailBundleChanged`].
    ///    Comparing scores across different bundles is meaningless.
    /// 2. Otherwise if `new_score < old_baseline.score`, fail regression.
    /// 3. Otherwise pass.
    pub fn evaluate_upgrade(
        old_baseline: &ConformanceBaseline,
        new_score: Score,
        new_bundle_hash: &str,
    ) -> GateDecision {
        if old_baseline.bundle_hash != new_bundle_hash {
            return GateDecision::FailBundleChanged;
        }
        if new_score.0 < old_baseline.score {
            return GateDecision::FailRegression {
                delta: old_baseline.score - new_score.0,
            };
        }
        GateDecision::Pass
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn baseline(score: f32, hash: &str) -> ConformanceBaseline {
        ConformanceBaseline {
            score,
            bundle_hash: hash.to_string(),
        }
    }

    #[test]
    fn gate_passes_when_score_meets_baseline() {
        let b = baseline(0.8, "abc");
        let decision = RegressionGate::evaluate_upgrade(&b, Score(0.85), "abc");
        assert_eq!(decision, GateDecision::Pass);

        let decision_eq = RegressionGate::evaluate_upgrade(&b, Score(0.8), "abc");
        assert_eq!(decision_eq, GateDecision::Pass);
    }

    #[test]
    fn gate_fails_on_regression() {
        let b = baseline(0.9, "abc");
        let decision = RegressionGate::evaluate_upgrade(&b, Score(0.7), "abc");
        match decision {
            GateDecision::FailRegression { delta } => {
                assert!((delta - 0.2).abs() < 1e-6, "delta was {delta}");
            }
            other => panic!("expected FailRegression, got {other:?}"),
        }
    }

    #[test]
    fn gate_fails_on_bundle_change() {
        let b = baseline(0.5, "abc");
        let decision = RegressionGate::evaluate_upgrade(&b, Score(1.0), "xyz");
        assert_eq!(decision, GateDecision::FailBundleChanged);
    }
}
