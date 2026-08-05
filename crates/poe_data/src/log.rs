//! Console helpers for visually distinct per-piece logs.
//!
//! ANSI colors only, no dependencies.

use std::fmt::Display;

/// Colored label for a data piece, printed as `[Maps]`.
#[derive(Clone, Copy)]
pub enum ColoredLabel {
    ActAreas,
    Maps,
    MapBosses,
    Cards,
}

impl ColoredLabel {
    fn ansi(&self) -> &'static str {
        match self {
            Self::ActAreas => "\x1b[36m",
            Self::Maps => "\x1b[33m",
            Self::MapBosses => "\x1b[35m",
            Self::Cards => "\x1b[32m",
        }
    }

    fn name(&self) -> &'static str {
        match self {
            Self::ActAreas => "Act Areas",
            Self::Maps => "Maps",
            Self::MapBosses => "Map Bosses",
            Self::Cards => "Cards",
        }
    }
}

impl Display for ColoredLabel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}[{}]\x1b[0m", self.ansi(), self.name())
    }
}
