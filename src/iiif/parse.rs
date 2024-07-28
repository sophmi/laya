use std::str::FromStr;
use crate::iiif::Rotation;

impl FromStr for Rotation {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mirror = s.starts_with('!');
        let rotation = if mirror { &s[1..] } else { s };

        // Accept a rotation of 0 as examples (e.g. s 9) show it should be, despite the spec also
        // containing the following contradiction:
        // Image API 3.0, s 4: the rotation parameter MUST be positive [as a float or integer]
        // Image API 3.0, s 4.3: The [rotation] ... may be any floating point number from 0 to 360

        rotation.parse::<f32>()
            .map_err(|_| ParseError::RotationAngleUnparsable(rotation.into()))
            .and_then(|degrees| {
                match degrees {
                    0f32..=360f32 => Ok(Rotation { mirror, degrees }),
                    _ => Err(ParseError::RotationOutOfBounds(rotation.into()))
                }
            })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ParseError {
    /// If the degrees to rotate by could not be parsed as a float.
    RotationAngleUnparsable(String),

    /// If the degrees to rotate by is out of bounds.
    RotationOutOfBounds(String),
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
}
