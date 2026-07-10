use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Display {
    None,
    Inline,
    Block,
    Table,
    TableRow,
    TableCell,
    TableCaption,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhiteSpace {
    Normal,
    Pre,
    PreLine,
    PreWrap,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TextTransform {
    None,
    Uppercase,
    Lowercase,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Style {
    pub display: Display,
    pub visibility_visible: bool,
    pub white_space: WhiteSpace,
    pub text_transform: TextTransform,
}

impl Style {
    pub fn root() -> Self {
        Self {
            display: Display::Block,
            visibility_visible: true,
            white_space: WhiteSpace::Normal,
            text_transform: TextTransform::None,
        }
    }
}

#[derive(Debug, Error)]
pub enum ExtractionError {
    #[error("unable to parse input HTML")]
    Parse,
}
