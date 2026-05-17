use std::fmt::Write;

use crate::rules::Layer;
use crate::source::PersonaSource;

/// Deterministic markdown projection of a `PersonaSource`.
///
/// Section order (fixed):
///   1. voice
///   2. L1 rules
///   3. L2 cascade-anchor (the `l2` anchor block, if present)
///   4. L3 rules
///   5. skills
///   6. default questions
///
/// Empty sections are omitted. Intentionally minimal -- M1 will expand this
/// into per-target (claude/codex/gemini/generic) renderers.
pub fn render_to_markdown(src: &PersonaSource) -> String {
    let mut out = String::new();

    // 1. voice
    if !src.persona.voice.is_empty() {
        let _ = writeln!(out, "## Voice\n\n{}\n", src.persona.voice);
    }

    // 2. L1 rules
    let l1: Vec<_> = src
        .rules
        .rules
        .iter()
        .filter(|r| matches!(r.layer, Layer::L1))
        .collect();
    if !l1.is_empty() {
        let _ = writeln!(out, "## L1 -- Invariants\n");
        for r in l1 {
            let _ = writeln!(out, "- **{}**: {}", r.id, r.text);
        }
        out.push('\n');
    }

    // 3. L2 cascade anchor (from persona.anchor["l2"])
    if let Some(anchor) = src.persona.anchor.get("l2") {
        let _ = writeln!(out, "## L2 -- Cascade Anchor\n\n{}\n", anchor.text);
    }

    // 4. L3 rules
    let l3: Vec<_> = src
        .rules
        .rules
        .iter()
        .filter(|r| matches!(r.layer, Layer::L3))
        .collect();
    if !l3.is_empty() {
        let _ = writeln!(out, "## L3 -- Preferences\n");
        for r in l3 {
            let _ = writeln!(out, "- **{}**: {}", r.id, r.text);
        }
        out.push('\n');
    }

    // 5. skills
    if !src.skills.skills.is_empty() {
        let _ = writeln!(out, "## Skills\n");
        for s in &src.skills.skills {
            let _ = writeln!(out, "- **{}** -- {}", s.id, s.invoke_when);
        }
        out.push('\n');
    }

    // 6. default questions
    if !src.persona.default_questions.is_empty() {
        let _ = writeln!(out, "## Default Questions\n");
        for q in &src.persona.default_questions {
            let _ = writeln!(out, "- {}", q.question);
        }
        out.push('\n');
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::persona::{Anchor, DefaultQuestion, Persona};
    use crate::rules::{Rule, RuleSet};
    use crate::skills::{Skill, SkillSet};
    use std::collections::BTreeMap;

    #[test]
    fn render_is_deterministic_and_ordered() {
        let mut anchor = BTreeMap::new();
        anchor.insert(
            "l2".to_string(),
            Anchor {
                text: "anchor body".to_string(),
            },
        );

        let src = PersonaSource {
            persona: Persona {
                schema_version: 1,
                name: "demo".to_string(),
                voice: "calm and precise".to_string(),
                anchor,
                default_questions: vec![DefaultQuestion {
                    question: "why?".to_string(),
                }],
            },
            rules: RuleSet {
                rules: vec![
                    Rule {
                        id: "l1a".to_string(),
                        layer: Layer::L1,
                        text: "do not panic".to_string(),
                    },
                    Rule {
                        id: "l3a".to_string(),
                        layer: Layer::L3,
                        text: "prefer brevity".to_string(),
                    },
                ],
            },
            skills: SkillSet {
                skills: vec![Skill {
                    id: "review".to_string(),
                    invoke_when: "on pull requests".to_string(),
                }],
            },
        };

        let first = render_to_markdown(&src);
        let second = render_to_markdown(&src);
        assert_eq!(first, second, "render must be deterministic");

        let voice_pos = first.find("## Voice").unwrap();
        let l1_pos = first.find("## L1").unwrap();
        let l2_pos = first.find("## L2").unwrap();
        let l3_pos = first.find("## L3").unwrap();
        let skills_pos = first.find("## Skills").unwrap();
        let qs_pos = first.find("## Default Questions").unwrap();
        assert!(voice_pos < l1_pos);
        assert!(l1_pos < l2_pos);
        assert!(l2_pos < l3_pos);
        assert!(l3_pos < skills_pos);
        assert!(skills_pos < qs_pos);
    }

    #[test]
    fn render_omits_empty_sections() {
        let src = PersonaSource::new(Persona::new("empty"));
        let out = render_to_markdown(&src);
        assert!(!out.contains("## Voice"));
        assert!(!out.contains("## L1"));
        assert!(!out.contains("## Skills"));
    }
}
