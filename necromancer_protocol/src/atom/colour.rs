//! # Colour generator
use crate::{error::Error, structs::VideoSource, Result};
use binrw::{binrw, BinRead, BinWrite};
use modular_bitfield::{bitfield, specifiers::B5};

const MAX_HUE: u16 = 3600;
const MAX_SAT_LUM: u16 = 1000;

/// Converts a colour generator ID to a [VideoSource].
///
/// Returns [VideoSource::Unknown] for unknown or invalid values.
pub fn generator_id_to_video_source(id: u8) -> VideoSource {
    match id {
        0 => VideoSource::Colour1,
        1 => VideoSource::Colour2,
        2 => VideoSource::Colour3,
        3 => VideoSource::Colour4,
        4 => VideoSource::Colour5,
        5 => VideoSource::Colour6,
        6 => VideoSource::Colour7,
        7 => VideoSource::Colour8,

        _ => VideoSource::Unknown,
    }
}

/// Converts a [VideoSource] to a colour generator ID.
///
/// Returns [Error::ParameterOutOfRange] for [VideoSource]s which are not colour
/// generators.
pub fn video_source_to_generator_id(src: VideoSource) -> Result<u8> {
    match src {
        VideoSource::Colour1 => Ok(0),
        VideoSource::Colour2 => Ok(1),
        VideoSource::Colour3 => Ok(2),
        VideoSource::Colour4 => Ok(3),
        VideoSource::Colour5 => Ok(4),
        VideoSource::Colour6 => Ok(5),
        VideoSource::Colour7 => Ok(6),
        VideoSource::Colour8 => Ok(7),
        _ => Err(Error::ParameterOutOfRange),
    }
}

#[cfg(feature = "palette")]
fn from_palette_hsl(colour: palette::Hsl) -> Result<(u16, u16, u16)> {
    let hue = (colour.hue.into_positive_degrees() * 10.) as u16;
    let sat = (colour.saturation * 1000.) as u16;
    let lum = (colour.lightness * 1000.) as u16;

    if hue > MAX_HUE || sat > MAX_SAT_LUM || lum > MAX_SAT_LUM {
        return Err(Error::ParameterOutOfRange);
    }

    Ok((hue, sat, lum))
}

#[cfg(feature = "palette")]
fn to_palette_hsl(hue: u16, sat: u16, lum: u16) -> palette::Hsl {
    palette::Hsl::new_srgb(
        f32::from(hue) / 10.,
        f32::from(sat) / 1000.,
        f32::from(lum) / 1000.,
    )
}

/// `ColV`: Change colour generator parameters (`ColourSourceValue`)
///
/// ## Packet format
///
/// * `u8`: colour generator ID
/// * 1 byte padding
/// * `u16`: hue (0 .. 3600)
/// * `u16`: saturation (0 .. 1000)
/// * `u16`: luminance
#[binrw]
#[brw(big)]
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct ColourGeneratorParams {
    #[brw(pad_after = 1)]
    pub id: u8,
    pub hue: u16,
    pub saturation: u16,
    pub luminance: u16,
}

impl ColourGeneratorParams {
    /// Gets the [VideoSource] that this [ColourGeneratorParams] is for.
    #[inline]
    pub fn video_source(&self) -> VideoSource {
        generator_id_to_video_source(self.id)
    }

    #[cfg(feature = "palette")]
    /// Gets the colour indicated in this command.
    #[inline]
    pub fn colour(&self) -> palette::Hsl {
        to_palette_hsl(self.hue, self.saturation, self.luminance)
    }
}

#[bitfield(bits = 8)]
#[repr(u8)]
#[derive(Specifier, BinRead, BinWrite, Default, Clone, Copy, PartialEq, Eq)]
#[br(map = From::<u8>::from)]
#[bw(map = |&x| Into::<u8>::into(x))]
struct SetColourGeneratorMask {
    pub hue: bool,
    pub saturation: bool,
    pub luminance: bool,
    #[skip]
    __: B5,
}

/// `CClV`: Set colour generator parameters
///
/// ## Examples
///
#[cfg_attr(feature = "palette", doc = "```")]
#[cfg_attr(not(feature = "palette"), doc = "```ignore")]
/// # use necromancer_protocol::atom::SetColourGeneratorParams;
/// # fn main() -> necromancer_protocol::Result<()> {
/// let params1 = SetColourGeneratorParams {
///     id: 0,
///     hue: Some(1230),
///     saturation: Some(1000),
///     luminance: Some(500),
/// };
///
/// // Can also use palette::Hsl in a builder pattern with the `palette` feature:
/// let params2 = SetColourGeneratorParams::new(0)
///     .with_colour(palette::Hsl::new_srgb(123., 1., 0.5))?;
///
/// assert_eq!(params1, params2);
/// # Ok(())
/// # }
/// ```
///
/// ## Packet format
///
/// * `u8`: setting mask
/// * `u8`: colour generator ID
/// * `u16`: hue (0 ..= 3600)
/// * `u16`: saturation (0 ..= 1000)
/// * `u16`: luminance (0 ..= 1000)
#[binrw]
#[brw(big)]
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct SetColourGeneratorParams {
    #[br(temp)]
    #[bw(calc(
        SetColourGeneratorMask::new()
            .with_hue(self.hue.is_some())
            .with_saturation(self.saturation.is_some())
            .with_luminance(self.luminance.is_some())
    ))]
    mask: SetColourGeneratorMask,

    /// The colour generator ID for this command.
    pub id: u8,

    /// The hue to set.
    ///
    /// Value is in the range `0..=3600`', eg: `1904` = 190.4° (cyan).
    #[br(map(|v: u16| mask.hue().then_some(v)))]
    #[brw(assert(hue.is_none_or(|h| h <= MAX_HUE)))]
    #[bw(map(|v| v.unwrap_or_default()))]
    pub hue: Option<u16>,

    /// The saturation to set.
    ///
    /// Value is in the range `0..=1000`, eg: `123` = 12.3%.
    #[br(map(|v: u16| mask.saturation().then_some(v)))]
    #[brw(assert(saturation.is_none_or(|v| v <= MAX_SAT_LUM)))]
    #[bw(map(|v| v.unwrap_or_default()))]
    pub saturation: Option<u16>,

    /// The luminance to set.
    ///
    /// Value is in the range `0..=1000`, eg: `123` = 12.3%.
    #[br(map(|v: u16| mask.luminance().then_some(v)))]
    #[brw(assert(luminance.is_none_or(|v| v <= MAX_SAT_LUM)))]
    #[bw(map(|v| v.unwrap_or_default()))]
    pub luminance: Option<u16>,
}

impl SetColourGeneratorParams {
    /// Creates a new [SetColourGeneratorParams].
    pub fn new(id: u8) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }

    /// Creates a new [SetColourGeneratorParams] for a given [VideoSource] ID.
    ///
    /// Returns [Error::ParameterOutOfRange] if `src` is not a colour generator.
    pub fn new_from_video_source(src: VideoSource) -> Result<Self> {
        let id = video_source_to_generator_id(src)?;
        Ok(Self::new(id))
    }

    /// The colour generator ID which is being configured.
    #[inline]
    pub const fn id(&self) -> u8 {
        self.id
    }

    /// Gets the [VideoSource] that this [ColourGeneratorParams] is for.
    #[inline]
    pub fn video_source(&self) -> VideoSource {
        generator_id_to_video_source(self.id)
    }

    #[cfg(feature = "palette")]
    /// Gets the colour indicated in this command if all components are set.
    pub fn colour(&self) -> Option<palette::Hsl> {
        match (self.hue, self.saturation, self.luminance) {
            (Some(hue), Some(sat), Some(lum)) => Some(to_palette_hsl(hue, sat, lum)),
            (_, _, _) => None,
        }
    }

    #[cfg(feature = "palette")]
    /// Sets the colour using a [palette::Hsl], using a builder pattern.
    pub fn with_colour(mut self, colour: palette::Hsl) -> Result<Self> {
        let (h, s, l) = from_palette_hsl(colour)?;
        self.hue = Some(h);
        self.saturation = Some(s);
        self.luminance = Some(l);
        Ok(self)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::atom::{Atom, Payload};
    use binrw::{BinRead, BinWrite};
    use std::io::Cursor;

    #[test]
    fn colour_generator_params() -> Result<()> {
        let expected = ColourGeneratorParams {
            id: 0,
            hue: 1904,
            saturation: 622,
            luminance: 1000,
        };

        let cmd = hex::decode("00100000436f6c5600000770026e03e8")?;
        let colv = Atom::read(&mut Cursor::new(&cmd))?;

        let Payload::ColourGeneratorParams(colv) = colv.payload else {
            panic!("wrong command type");
        };
        assert_eq!(expected, colv);

        let o = Atom::new(expected);
        let mut out = Cursor::new(Vec::with_capacity(cmd.len()));
        o.write(&mut out)?;
        assert_eq!(cmd, out.into_inner());
        Ok(())
    }

    #[test]
    fn set_hue() -> Result<()> {
        let expected = SetColourGeneratorParams {
            id: 0,
            hue: Some(1904),
            ..Default::default()
        };

        let cmd = hex::decode("0010000043436c560100077000000000")?;
        let cclv = Atom::read(&mut Cursor::new(&cmd))?;

        let Payload::SetColourGeneratorParams(cclv) = cclv.payload else {
            panic!("wrong command type");
        };
        assert_eq!(expected, cclv);

        let o = Atom::new(expected);
        let mut out = Cursor::new(Vec::with_capacity(cmd.len()));
        o.write(&mut out)?;
        assert_eq!(cmd, out.into_inner());
        Ok(())
    }

    #[test]
    fn set_saturation() -> Result<()> {
        let expected = SetColourGeneratorParams {
            id: 0,
            saturation: Some(622),
            ..Default::default()
        };

        let cmd = hex::decode("0010000043436c5602000000026e0000")?;
        let cclv = Atom::read(&mut Cursor::new(&cmd))?;

        let Payload::SetColourGeneratorParams(cclv) = cclv.payload else {
            panic!("wrong command type");
        };
        assert_eq!(expected, cclv);

        let o = Atom::new(expected);
        let mut out = Cursor::new(Vec::with_capacity(cmd.len()));
        o.write(&mut out)?;
        assert_eq!(cmd, out.into_inner());
        Ok(())
    }

    #[test]
    fn set_luminance() -> Result<()> {
        let expected = SetColourGeneratorParams {
            id: 0,
            luminance: Some(1000),
            ..Default::default()
        };

        let cmd = hex::decode("0010000043436c5604000000000003e8")?;
        let cclv = Atom::read(&mut Cursor::new(&cmd))?;

        let Payload::SetColourGeneratorParams(cclv) = cclv.payload else {
            panic!("wrong command type");
        };
        assert_eq!(expected, cclv);

        let o = Atom::new(expected);
        let mut out = Cursor::new(Vec::with_capacity(cmd.len()));
        o.write(&mut out)?;
        assert_eq!(cmd, out.into_inner());
        Ok(())
    }

    /// Check that fields are set to `None` if the bitmask doesn't mention it,
    /// even when it contains uninitialised memory.
    #[test]
    fn bitmask() -> Result<()> {
        // Set hue
        let expected = SetColourGeneratorParams {
            id: 0,
            hue: Some(1904),
            ..Default::default()
        };
        let cmd = hex::decode("0010000043436c560100077000000000")?;
        let cclv = Atom::read(&mut Cursor::new(&cmd))?;

        let Payload::SetColourGeneratorParams(cclv) = cclv.payload else {
            panic!("wrong command type");
        };
        assert_eq!(expected, cclv);

        // Set saturation
        let expected = SetColourGeneratorParams {
            id: 0,
            saturation: Some(622),
            ..Default::default()
        };
        let cmd = hex::decode("0010000043436c5602007007026e0000")?;
        let cclv = Atom::read(&mut Cursor::new(&cmd))?;

        let Payload::SetColourGeneratorParams(cclv) = cclv.payload else {
            panic!("wrong command type");
        };
        assert_eq!(expected, cclv);

        // Set luminance
        let expected = SetColourGeneratorParams {
            id: 0,
            luminance: Some(1000),
            ..Default::default()
        };
        let cmd = hex::decode("0010740343436c5604000215600003e8")?;
        let cclv = Atom::read(&mut Cursor::new(&cmd))?;

        let Payload::SetColourGeneratorParams(cclv) = cclv.payload else {
            panic!("wrong command type");
        };
        assert_eq!(expected, cclv);

        Ok(())
    }

    #[cfg(feature = "palette")]
    #[test]
    fn palette() -> Result<()> {
        let colour = palette::Hsl::new_srgb(123.4, 0.567, 0.654);
        let colv = ColourGeneratorParams {
            id: 1,
            hue: 1234,
            saturation: 567,
            luminance: 654,
        };
        assert_eq!(VideoSource::Colour2, colv.video_source());
        assert_eq!(colour, colv.colour());

        let expected = SetColourGeneratorParams {
            id: 0,
            hue: Some(1234),
            saturation: Some(567),
            luminance: Some(654),
        };
        let cclv = SetColourGeneratorParams::new(0).with_colour(colour)?;

        assert_eq!(expected, cclv);
        assert_eq!(VideoSource::Colour1, cclv.video_source());
        assert_eq!(Some(colour), cclv.colour());

        Ok(())
    }
}
