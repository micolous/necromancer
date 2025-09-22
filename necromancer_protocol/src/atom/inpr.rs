use crate::{
    atom::{colour::video_source_to_generator_id, str_from_utf8_null},
    structs::{ExternalPortType, PortType, VideoSource},
    Result,
};
use binrw::binrw;
use std::fmt::Debug;

/// `InPr`: Input properties
#[binrw]
#[brw(big)]
#[derive(PartialEq, Eq, Clone)]
pub struct InputProperties {
    pub video_source: VideoSource,
    pub long_name: [u8; 20],
    pub short_name: [u8; 4],
    #[br(map = |v: u8| v != 0)]
    #[bw(map = |v: &bool| Into::<u8>::into(*v))]
    pub input_names_are_default: bool,
    // pub available_external_port_types: u8, ?? may be wrong, always 0, never used
    // 0x24; ATEM API treats this as a mix of u8 and u16
    #[brw(align_before = 2)]
    pub available_external_port_types: ExternalPortType,
    // 0x26
    pub external_port_type: ExternalPortType,
    // 0x28 (this is 2 bytes, but we only retain the high byte)
    #[brw(pad_after = 1)]
    pub port_type: PortType,
    // 0x2a
    pub availability3: u16,
}

impl Debug for InputProperties {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InputProperties")
            .field("video_source", &self.video_source)
            .field(
                "long_name",
                &self
                    .get_long_name()
                    .map(str::to_string)
                    .unwrap_or_else(|_| self.long_name.escape_ascii().to_string()),
            )
            .field(
                "short_name",
                &self
                    .get_short_name()
                    .map(str::to_string)
                    .unwrap_or_else(|_| self.short_name.escape_ascii().to_string()),
            )
            .field("input_names_are_default", &self.input_names_are_default)
            .field(
                "available_external_port_types",
                &self.available_external_port_types,
            )
            .field("external_port_type", &self.external_port_type)
            .field("port_type", &self.port_type)
            .field("availability3", &self.availability3)
            .finish()
    }
}

impl InputProperties {
    #[inline]
    pub fn get_long_name(&self) -> Result<&str> {
        str_from_utf8_null(&self.long_name)
    }

    #[inline]
    pub fn get_short_name(&self) -> Result<&str> {
        str_from_utf8_null(&self.short_name)
    }

    /// Gets the colour generator ID for this video source.
    ///
    /// Returns [`None`] if this is not a colour generator.
    pub fn colour_generator_id(&self) -> Option<u8> {
        if self.port_type != PortType::ColourGenerator {
            return None;
        }
        video_source_to_generator_id(self.video_source).ok()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::atom::{Atom, Payload};
    use binrw::BinRead;
    use std::io::Cursor;

    #[test]
    fn input_properties() -> Result<()> {
        let cmd = hex::decode("002c0000496e5072000143616d657261203100000000000000000000000043414d3101000002000200001101")?;
        let inpr = Atom::read(&mut Cursor::new(&cmd))?;

        let Payload::InputProperties(input_properties) = inpr.payload else {
            panic!("wrong command type");
        };

        // println!("input properties: {:x?}", input_properties);
        assert_eq!(VideoSource::Input1, input_properties.video_source);
        assert_eq!("Camera 1", input_properties.get_long_name()?);
        assert_eq!("CAM1", input_properties.get_short_name()?);
        assert!(input_properties.input_names_are_default);
        assert_eq!(
            ExternalPortType::new().with_hdmi(true),
            input_properties.available_external_port_types
        );
        assert_eq!(
            ExternalPortType::new().with_hdmi(true),
            input_properties.external_port_type
        );
        assert_eq!(PortType::External, input_properties.port_type);
        assert_eq!(None, input_properties.colour_generator_id());

        Ok(())
    }

    #[test]
    fn colour_bars() -> Result<()> {
        let pl = hex::decode(
            "03e8436f6c6f722042617273000000000000000000004241525301000100010002001001",
        )?;
        let input_properties = InputProperties::read(&mut Cursor::new(&pl))?;
        // println!("{input_properties:x?}");
        assert_eq!(VideoSource::ColourBars, input_properties.video_source);
        assert_eq!("Color Bars", input_properties.get_long_name()?);
        assert_eq!("BARS", input_properties.get_short_name()?);
        assert!(input_properties.input_names_are_default);
        assert_eq!(
            ExternalPortType::new().with_internal(true),
            input_properties.available_external_port_types
        );
        assert_eq!(
            ExternalPortType::new().with_internal(true),
            input_properties.external_port_type
        );
        assert_eq!(PortType::ColourBars, input_properties.port_type);
        assert_eq!(None, input_properties.colour_generator_id());

        Ok(())
    }

    #[test]
    fn edited_input() -> Result<()> {
        let pl = hex::decode(
            "00014c6170746f702048444d490000000000000000004c41500000000002000200001101",
        )?;
        let input_properties = InputProperties::read(&mut Cursor::new(&pl))?;
        // println!("input properties: {:x?}", input_properties);
        assert_eq!(VideoSource::Input1, input_properties.video_source);
        assert_eq!("Laptop HDMI", input_properties.get_long_name()?);
        assert_eq!("LAP", input_properties.get_short_name()?);
        assert!(!input_properties.input_names_are_default);
        assert_eq!(
            ExternalPortType::new().with_hdmi(true),
            input_properties.available_external_port_types
        );
        assert_eq!(
            ExternalPortType::new().with_hdmi(true),
            input_properties.external_port_type
        );
        assert_eq!(PortType::External, input_properties.port_type);
        assert_eq!(None, input_properties.colour_generator_id());
        Ok(())
    }

    #[test]
    fn direct() -> Result<()> {
        let pl = hex::decode(
            "2af94c6170746f702048444d492044697265637400004449520000000002000207000100",
        )?;
        let input_properties = InputProperties::read(&mut Cursor::new(&pl))?;
        // println!("input properties: {:x?}", input_properties);
        assert_eq!(VideoSource::Input1Direct, input_properties.video_source);
        assert_eq!("Laptop HDMI Direct", input_properties.get_long_name()?);
        assert_eq!("DIR", input_properties.get_short_name()?);
        assert!(!input_properties.input_names_are_default);
        assert_eq!(
            ExternalPortType::new().with_hdmi(true),
            input_properties.available_external_port_types
        );
        assert_eq!(
            ExternalPortType::new().with_hdmi(true),
            input_properties.external_port_type
        );
        assert_eq!(PortType::ExternalDirect, input_properties.port_type);
        assert_eq!(None, input_properties.colour_generator_id());
        Ok(())
    }

    #[test]
    fn colour_generator() -> Result<()> {
        let expected = InputProperties {
            video_source: VideoSource::Colour1,
            long_name: *b"Color 1\0\0\0\0\0\0\0\0\0\0\0\0\0",
            short_name: *b"COL1",
            input_names_are_default: true,
            available_external_port_types: ExternalPortType::new().with_internal(true),
            external_port_type: ExternalPortType::new().with_internal(true),
            port_type: PortType::ColourGenerator,
            availability3: 1,
        };
        let cmd = hex::decode("002c0000496e507207d1436f6c6f72203100000000000000000000000000434f4c3101000100010003000001")?;
        let inpr = Atom::read(&mut Cursor::new(&cmd))?;

        let Payload::InputProperties(inpr) = inpr.payload else {
            panic!("wrong command type");
        };

        assert_eq!(expected, inpr);
        assert_eq!(Some(0), inpr.colour_generator_id());
        Ok(())
    }
}
