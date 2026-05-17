use frameshift_source::PersonaSource;

use crate::composed::{ComposedPersona, Layer, Provenance, ProvenancedRule, ProvenancedSkill};
use crate::error::ComposeError;

/// The order in which composition layers are stacked.
///
/// `BaseFirst` means: base (extends) at the bottom, mixins in declared order
/// above, root persona at the top. Later layers override earlier ones on
/// ID collision.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MergeOrder {
    BaseFirst,
}

/// Merge a list of persona source layers into a single `ComposedPersona`.
///
/// The first element of `layers` is treated as the root persona (its
/// `Persona` block wins for top-level metadata). Each subsequent layer is
/// composed in over the previous, with last-write-wins on `id` collision
/// for both rules and skills.
///
/// NOTE for WS-5: layer identity (Base vs Mixin vs Root) is not carried in
/// through `&[&PersonaSource]` -- we tag every contributed item as
/// `Layer::Root` for now. Layer-aware provenance lands when `Composer`
/// orchestrates the call in M1. Same goes for `Conflict` reporting: that's
/// future work (see `conflict::detect_conflicts`).
pub fn merge_sources(
    order: MergeOrder,
    layers: &[&PersonaSource],
) -> Result<ComposedPersona, ComposeError> {
    match order {
        MergeOrder::BaseFirst => merge_base_first(layers),
    }
}

fn merge_base_first(layers: &[&PersonaSource]) -> Result<ComposedPersona, ComposeError> {
    let root = layers.first().ok_or(ComposeError::Unresolved {
        spec: "<empty>".to_string(),
        reason: "merge_sources called with no layers".to_string(),
    })?;

    let mut rules: Vec<ProvenancedRule> = Vec::new();
    let mut skills: Vec<ProvenancedSkill> = Vec::new();

    for layer in layers.iter() {
        let provenance = Provenance { layer: Layer::Root };

        for rule in layer.rules.rules.iter() {
            let id = rule.id.clone();
            if let Some(existing) = rules.iter_mut().find(|p| p.rule.id == id) {
                *existing = ProvenancedRule {
                    rule: rule.clone(),
                    provenance: provenance.clone(),
                };
            } else {
                rules.push(ProvenancedRule {
                    rule: rule.clone(),
                    provenance: provenance.clone(),
                });
            }
        }

        for skill in layer.skills.skills.iter() {
            let id = skill.id.clone();
            if let Some(existing) = skills.iter_mut().find(|p| p.skill.id == id) {
                *existing = ProvenancedSkill {
                    skill: skill.clone(),
                    provenance: provenance.clone(),
                };
            } else {
                skills.push(ProvenancedSkill {
                    skill: skill.clone(),
                    provenance: provenance.clone(),
                });
            }
        }
    }

    Ok(ComposedPersona {
        persona: root.persona.clone(),
        rules,
        skills,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use frameshift_source::{Persona, PersonaSource};

    fn fixture(name: &str) -> PersonaSource {
        PersonaSource::new(Persona::new(name))
    }

    #[test]
    fn merge_two_empty_sources_yields_empty_composed() {
        let a = fixture("base");
        let b = fixture("root");
        let composed = merge_sources(MergeOrder::BaseFirst, &[&a, &b])
            .expect("empty merge should succeed");
        assert!(composed.rules.is_empty());
        assert!(composed.skills.is_empty());
        assert_eq!(composed.persona.name, "base");
    }
}
