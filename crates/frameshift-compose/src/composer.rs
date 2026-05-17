use frameshift_source::PersonaSource;

use crate::composed::ComposedPersona;
use crate::error::ComposeError;
use crate::resolver::SourceResolver;

/// Orchestrates composition of a root persona with its `extends` base and
/// `mixins`, using a `SourceResolver` to fetch each layer by spec.
///
/// The composer is generic over the resolver so test code can plug in an
/// in-memory implementation and production can use `LocalResolver` (or, in
/// M1+, a cache-backed resolver).
pub struct Composer<R: SourceResolver> {
    resolver: R,
}

impl<R: SourceResolver> Composer<R> {
    pub fn new(resolver: R) -> Self {
        Self { resolver }
    }

    pub fn resolver(&self) -> &R {
        &self.resolver
    }

    /// Compose `root` with its declared `extends` base and `mixins`.
    ///
    /// Layer order is: base (if any) -> mixins (in declared order) -> root.
    /// Each layer overrides previous on rule/skill id collision.
    pub fn compose(
        &self,
        _root: PersonaSource,
        _extends: Option<String>,
        _mixins: &[String],
    ) -> Result<ComposedPersona, ComposeError> {
        todo!("M1 impl");
    }
}
