//! Non-atom structures and enumerations.
//!
//! These generally correspond to `BEPStruct*` classes in `BMDSwitcherAPI`.
mod equaliser;
mod external_port_type;
mod port_type;
mod tally;
mod video_mode;
mod video_source;

pub use self::{
    equaliser::{
        EqualiserRange, EqualiserRangeLimit, EqualiserShape, SupportedEqualiserRanges,
        SupportedEqualiserShapes,
    },
    external_port_type::ExternalPortType,
    port_type::PortType,
    tally::TallyFlags,
    video_mode::VideoMode,
    video_source::VideoSource,
};
