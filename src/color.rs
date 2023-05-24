/// Set color mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    /// Don't pass the `--color` option.
    /// Allow the `diff` command to decide for itself
    Auto,
    /// Pass `--color=always` option.
    /// Force the `diff` command to always show color.
    Always,
    /// Pass `--color=never` option.
    /// Prevent the `diff` command from showing color.
    Never,
}

impl Color {
    /// Convert [`Color`] to CLI flag.
    pub const fn as_flag(&self) -> Option<&'static str> {
        match self {
            Color::Auto => None,
            Color::Always => Some("--color=always"),
            Color::Never => Some("--color=never"),
        }
    }
}
