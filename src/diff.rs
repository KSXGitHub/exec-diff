use super::Color;

/// The diff executor.
///
/// This is a wrapper around the GNU `diff` command.
#[derive(Debug, Clone, Copy)]
pub struct Diff<Left, Right> {
    /// Value on the left.
    pub left: Left,
    /// Value on the right.
    pub right: Right,
    /// Whether to include ANSI color in the diff.
    pub color: Color,
    /// Whether to use the unified diff format.
    pub unified: bool,
}

impl<Left, Right> Diff<Left, Right> {
    /// Initialize a diff executor.
    pub const fn new(left: Left, right: Right) -> Self {
        Diff {
            left,
            right,
            color: Color::Always,
            unified: true,
        }
    }

    /// Set the left value.
    pub fn left<NewLeft>(self, left: NewLeft) -> Diff<NewLeft, Right> {
        let Diff {
            left: _,
            right,
            color,
            unified,
        } = self;
        Diff {
            left,
            right,
            color,
            unified,
        }
    }

    /// Set the right value.
    pub fn right<NewRight>(self, right: NewRight) -> Diff<Left, NewRight> {
        let Diff {
            left,
            right: _,
            color,
            unified,
        } = self;
        Diff {
            left,
            right,
            color,
            unified,
        }
    }

    /// Set color mode. Default is [`Color::Always`].
    pub const fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Set diff format. Default is `true`.
    pub const fn unified(mut self, unified: bool) -> Self {
        self.unified = unified;
        self
    }
}
