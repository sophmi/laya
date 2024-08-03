use std::str::FromStr;

use crate::iiif::{Format, Quality, Region, Rotation};

const PERCENT_PREFIX: &str = "pct:";
const REGION_SELECTOR_COUNT: usize = 4;

impl FromStr for Region {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "full" => Ok(Region::Full),
            "square" => Ok(Region::Square),
            _ => {
                if let Some(input) = s.strip_prefix(PERCENT_PREFIX) {
                    let [x, y, width, height] = parse_region_selectors(input)?;
                    Ok(Region::Percentage { x, y, width, height })
                } else {
                    let [x, y, width, height] = parse_region_selectors(s)?;
                    Ok(Region::Absolute { x, y, width, height })
                }
            }
        }
    }
}

fn parse_region_selectors<T: SpatialSelector>(
    input: &str,
) -> Result<[T; REGION_SELECTOR_COUNT], ParseError> {
    let mut selectors = [T::default(); REGION_SELECTOR_COUNT];
    let mut split = input.split(",");

    for (index, item) in selectors.iter_mut().enumerate() {
        let s = split
            .next()
            .ok_or(ParseError::RegionSelectorCount(input.into()))?;

        *item = s
            .parse::<T>()
            .map_err(|_| ParseError::region_unparsable(s, index))
            .and_then(|value| T::validate(value, index))?;
    }

    match split.next() {
        Some(_) => Err(ParseError::RegionSelectorCount(input.into())),
        None => Ok(selectors),
    }
}

trait SpatialSelector: Copy + Default + FromStr + ToString {
    fn in_bounds(value: Self, index: usize) -> bool;

    fn validate(value: Self, index: usize) -> Result<Self, ParseError> {
        if Self::in_bounds(value, index) {
            Ok(value)
        } else {
            Err(ParseError::RegionSelectorOutOfBounds {
                input: value.to_string(),
                selector: SpatialSelectorIndex::from_ordinal(index),
            })
        }
    }
}

impl SpatialSelector for f32 {
    fn in_bounds(value: Self, _: usize) -> bool {
        // Image API 3.0, s 4.1: region parameters in percentages ... must be positive
        value > 0.0
    }
}

impl SpatialSelector for u32 {
    fn in_bounds(value: Self, index: usize) -> bool {
        // Image API 3.0, s 4.1: If the requested region’s height or width is zero, [fail].
        index <= SpatialSelectorIndex::Y as usize || value > 0
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum SpatialSelectorIndex {
    X = 0,
    Y,
    Width,
    Height,
}

impl SpatialSelectorIndex {
    fn from_ordinal(value: usize) -> Self {
        match value {
            0 => SpatialSelectorIndex::X,
            1 => SpatialSelectorIndex::Y,
            2 => SpatialSelectorIndex::Width,
            3 => SpatialSelectorIndex::Height,
            _ => panic!("Region bounds index must be [0, 3]."),
        }
    }
}

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
    /// If one of the spatial selectors (x, y, width, height) is out of bounds.
    RegionSelectorOutOfBounds { input: String, selector: SpatialSelectorIndex },

    /// If one of the spatial selectors (x, y, width, height) could not be parsed.
    RegionSelectorUnparsable { input: String, selector: SpatialSelectorIndex },

    /// If a spatial selector was missing, or too many were provided.
    RegionSelectorCount(String),

    /// If the degrees to rotate by could not be parsed as a float.
    RotationAngleUnparsable(String),

    /// If the degrees to rotate by is out of bounds.
    RotationOutOfBounds(String),

    /// If the requested quality is unrecognised.
    UnrecognisedQuality(String),

    /// If the requested format is unrecognised.
    UnrecognisedFormat(String),
}

impl ParseError {
    fn region_unparsable<S: Into<String>>(input: S, sel_index: usize) -> ParseError {
        ParseError::RegionSelectorUnparsable {
            input: input.into(),
            selector: SpatialSelectorIndex::from_ordinal(sel_index),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn full_region() {
        let result = "full".parse::<Region>();
        assert_eq!(result, Ok(Region::Full));
    }

    #[test]
    fn square_region() {
        let result = "square".parse::<Region>();
        assert_eq!(result, Ok(Region::Square));
    }

    #[test]
    fn percent_region() {
        let result = "pct:5,5,90,90".parse::<Region>();
        assert_eq!(result, Ok(Region::Percentage { x: 5.0, y: 5.0, width: 90.0, height: 90.0 }));

        let result = "pct:5.1,5.2,90.3,90.4".parse::<Region>();
        assert_eq!(result, Ok(Region::Percentage { x: 5.1, y: 5.2, width: 90.3, height: 90.4 }));
    }

    #[test]
    fn absolute_region() {
        let result = "5,5,50,50".parse::<Region>();
        assert_eq!(result, Ok(Region::Absolute { x: 5, y: 5, width: 50, height: 50 }));
    }

    #[test]
    fn region_selector_count_err() {
        let input = "5,5,1";
        assert_eq!(input.parse::<Region>(), Err(ParseError::RegionSelectorCount(input.into())));

        let input = "5,5,50,50,";
        assert_eq!(input.parse::<Region>(), Err(ParseError::RegionSelectorCount(input.into())));

        let input = "5,5,50,50,0";
        assert_eq!(input.parse::<Region>(), Err(ParseError::RegionSelectorCount(input.into())));
    }

    #[test]
    fn region_selector_unparsable_err() {
        let input = "abcdefg";
        assert_eq!(
            input.parse::<Region>(),
            Err(ParseError::RegionSelectorUnparsable {
                input: input.into(),
                selector: SpatialSelectorIndex::X,
            })
        );

        let result = "5,5,c,5".parse::<Region>();
        assert_eq!(
            result,
            Err(ParseError::RegionSelectorUnparsable {
                input: "c".into(),
                selector: SpatialSelectorIndex::Width,
            })
        );

        let result = "5,5,5,".parse::<Region>();
        assert_eq!(
            result,
            Err(ParseError::RegionSelectorUnparsable {
                input: "".into(),
                selector: SpatialSelectorIndex::Height,
            })
        );
    }

    #[test]
    fn region_selector_oob_err() {
        let result = "5,5,0,90".parse::<Region>();
        assert_eq!(
            result,
            Err(ParseError::RegionSelectorOutOfBounds {
                input: "0".into(),
                selector: SpatialSelectorIndex::Width,
            })
        );

        let result = "5,5,90,0".parse::<Region>();
        assert_eq!(
            result,
            Err(ParseError::RegionSelectorOutOfBounds {
                input: "0".into(),
                selector: SpatialSelectorIndex::Height,
            })
        );
    }

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
