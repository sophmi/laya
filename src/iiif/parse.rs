use std::str::FromStr;

use crate::iiif::{Format, Quality, Rotation};

impl FromStr for Rotation {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mirror = s.starts_with('!');
        let rotation = if mirror { &s[1..] } else { s };

        // Accept a rotation of 0 as examples (e.g. s 9) show it should be, despite the spec also
        // containing the following contradiction:
        // Image API 3.0, s 4: the rotation parameter MUST be positive [as a float or integer]
        // Image API 3.0, s 4.3: The [rotation] ... may be any floating point number from 0 to 360

        rotation
            .parse::<f32>()
            .map_err(|_| ParseError::RotationAngleUnparsable(rotation.into()))
            .and_then(|degrees| match degrees {
                0f32..=360f32 => Ok(Rotation { mirror, degrees }),
                _ => Err(ParseError::RotationOutOfBounds(rotation.into())),
            })
    }
}

impl FromStr for Quality {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "color" => Ok(Quality::Color),
            "gray" => Ok(Quality::Gray),
            "bitonal" => Ok(Quality::Bitonal),
            "default" => Ok(Quality::Default),
            _ => Err(ParseError::UnrecognisedQuality(s.into())),
        }
    }
}

impl FromStr for Format {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "jpg" => Ok(Format::Jpg),
            "tif" => Ok(Format::Tif),
            "png" => Ok(Format::Png),
            "gif" => Ok(Format::Gif),
            "jp2" => Ok(Format::Jp2),
            "pdf" => Ok(Format::Pdf),
            "webp" => Ok(Format::Webp),
            _ => Err(ParseError::UnrecognisedFormat(s.into())),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ParseError {
    /// If the degrees to rotate by could not be parsed as a float.
    RotationAngleUnparsable(String),

    /// If the degrees to rotate by is out of bounds.
    RotationOutOfBounds(String),

    /// If the requested quality is unrecognised.
    UnrecognisedQuality(String),

    /// If the requested format is unrecognised.
    UnrecognisedFormat(String),
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn rotation() {
        let result = "180".parse::<Rotation>();
        assert_eq!(result, Ok(Rotation { mirror: false, degrees: 180f32 }));

        let result = "180.42".parse::<Rotation>();
        assert_eq!(result, Ok(Rotation { mirror: false, degrees: 180.42f32 }));
    }

    #[test]
    fn mirrored_rotation() {
        let result = "!180".parse::<Rotation>();
        assert_eq!(result, Ok(Rotation { mirror: true, degrees: 180f32 }));
    }

    #[test]
    fn rotation_out_of_bounds_err() {
        let result = "-1".parse::<Rotation>();
        assert_eq!(result, Err(ParseError::RotationOutOfBounds("-1".into())));
    }

    #[test]
    fn rotation_not_float_err() {
        let result = "TRaFoaMP20230922".parse::<Rotation>();
        assert_eq!(result, Err(ParseError::RotationAngleUnparsable("TRaFoaMP20230922".into())));
    }

    #[test]
    fn quality() {
        let result = "color".parse::<Quality>();
        assert_eq!(result, Ok(Quality::Color));
    }

    #[test]
    fn unrecognised_quality_err() {
        let result = "TRaFoaMP20230922".parse::<Quality>();
        assert_eq!(result, Err(ParseError::UnrecognisedQuality("TRaFoaMP20230922".into())));
    }

    #[test]
    fn unrecognised_format_err() {
        let result = "TRaFoaMP20230922".parse::<Format>();
        assert_eq!(result, Err(ParseError::UnrecognisedFormat("TRaFoaMP20230922".into())));
    }
}
