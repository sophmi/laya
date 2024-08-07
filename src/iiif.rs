mod image;
mod info;
pub(crate) mod parse;

use std::num::NonZero;

#[derive(Clone, Debug, PartialEq)]
pub struct ImageRequest {
    identifier: String,
    region: Region,
    size: Size,
    rotation: Rotation,
    quality: Quality,
    format: Format,
}

impl ImageRequest {
    pub fn new<S: Into<String>>(
        identifier: S,
        region: Region,
        size: Size,
        rotation: Rotation,
        quality: Quality,
        format: Format,
    ) -> ImageRequest {
        ImageRequest { identifier: identifier.into(), region, size, rotation, quality, format }
    }

    pub fn identifier(&self) -> &str {
        &self.identifier
    }

    pub fn region(&self) -> &Region {
        &self.region
    }

    pub fn size(&self) -> Size {
        self.size
    }

    pub fn rotation(&self) -> Rotation {
        self.rotation
    }

    pub fn quality(&self) -> Quality {
        self.quality
    }

    pub fn format(&self) -> Format {
        self.format
    }
}

pub type Dimension = u32;

#[derive(Clone, Debug, PartialEq)]
pub enum Region {
    Full,
    Square,
    Absolute { x: Dimension, y: Dimension, width: Dimension, height: Dimension },
    Percentage { x: f32, y: f32, width: f32, height: f32 },
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Size {
    scale: Scale,
    upscale: bool,
}

impl Size {
    pub fn new(scale: Scale) -> Size {
        Size { upscale: false, scale }
    }

    pub fn upscaled(scale: Scale) -> Size {
        Size { upscale: true, scale }
    }

    pub fn scale(&self) -> Scale {
        self.scale
    }

    pub fn upscale(&self) -> bool {
        self.upscale
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Scale {
    Max,
    Percentage(f32),
    Fixed { width: Option<NonZero<Dimension>>, height: Option<NonZero<Dimension>> },
    AspectPreserving { width: NonZero<Dimension>, height: NonZero<Dimension> },
}

impl Scale {
    pub fn fixed(width: Option<NonZero<Dimension>>, height: Option<NonZero<Dimension>>) -> Scale {
        Scale::Fixed { width, height }
    }

    pub fn fixed_height(height: Option<NonZero<Dimension>>) -> Scale {
        Scale::Fixed { width: None, height }
    }

    pub fn fixed_width(width: Option<NonZero<Dimension>>) -> Scale {
        Scale::Fixed { width, height: None }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rotation {
    degrees: f32,
    mirror: bool,
}

impl Rotation {
    pub fn new(degrees: f32) -> Rotation {
        Rotation { mirror: false, degrees }
    }

    pub fn mirrored(degrees: f32) -> Rotation {
        Rotation { mirror: true, degrees }
    }

    pub fn degrees(&self) -> f32 {
        self.degrees
    }

    pub fn mirror(&self) -> bool {
        self.mirror
    }
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum Quality {
    Color,
    Gray,
    Bitonal,
    Default,
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum Format {
    Jpg,
    Tif,
    Png,
    Gif,
    Jp2,
    Pdf,
    Webp,
}

impl Format {
    pub fn mime(&self) -> &'static str {
        match self {
            Format::Jpg => "image/jpeg",
            Format::Tif => "image/tiff",
            Format::Png => "image/png",
            Format::Gif => "image/gif",
            Format::Jp2 => "image/jp2",
            Format::Pdf => "application/pdf",
            Format::Webp => "image/webp",
        }
    }
}
