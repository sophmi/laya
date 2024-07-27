mod info;
mod image;

use std::num::{NonZero};

pub struct ImageRequest {
    ident: String,
    region: Region,
    size: Size,
    rotation: Rotation,
    quality: Quality,
    format: String,
}

pub type Dimension = u32;

enum Region {
    Full,
    Square,
    Absolute { x: Dimension, y: Dimension, width: NonZero<Dimension>, height: NonZero<Dimension> },
    Percentage { x: Dimension, y: Dimension, width: NonZero<Dimension>, height: NonZero<Dimension> },
}

struct Size {
    maximize: bool,
    scale: Option<Scale>,
    upscale: bool,
    mirror: bool,
}

enum Scale {
    Percentage(u32),
    Fixed { width: Option<NonZero<Dimension>>, height: Option<NonZero<Dimension>> },
}

struct Rotation {
    mirror: bool,
    degrees: u16,
}

enum Quality {
    Color,
    Gray,
    Bitonal,
    Default,
}