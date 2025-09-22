//! Simple ATEM USB tally light with [`tomu_usb_simple_client`][].

use clap::Parser;
use necromancer::{
    protocol::structs::{TallyFlags, VideoSource},
    AtemController, Error as NecroError, StateUpdate,
};
use std::net::SocketAddrV4;
use thiserror::Error;
use tomu_usb_simple_client::{Colour, Error as TomuError, TomuUsbSimple};
use tracing::{error, info, level_filters::LevelFilter};
use tracing_subscriber::EnvFilter;

#[derive(Debug, Error)]
enum Error {
    #[error("Necromancer error: {0}")]
    Necromancer(#[from] NecroError),

    #[error("Tomu error: {0}")]
    Tomu(#[from] TomuError),
}

type Result<T = ()> = std::result::Result<T, Error>;

/// ATEM tally light with a Tomu running usb_simple.
///
/// **Warning:** this talks the _full_ ATEM protocol, which is very chatty, especially with
/// multiple client connections. It'd be better to have a single client re-broadcast tally events
/// from the switcher to your tally devices.
#[derive(Debug, Parser)]
#[clap(verbatim_doc_comment)]
struct CliParser {
    /// IP address of the ATEM switcher.
    #[clap(short, long)]
    pub ip: String,

    /// Input source to tally on.
    #[clap(short, long)]
    pub source: VideoSource,

    /// Zero-indexed media encoder to watch.
    #[clap(short, long, default_value = "0")]
    pub me: u8,

    /// Automatically reconnect on connection loss.
    #[clap(short, long)]
    pub reconnect: bool,
}

async fn update_tomu(tomu: &mut TomuUsbSimple, tally: TallyFlags) -> Result {
    let colour = match (tally.preview(), tally.program()) {
        (true, true) => Colour::Both,
        (true, false) => Colour::Green,
        (false, true) => Colour::Red,
        (false, false) => Colour::Off,
    };

    info!("Colour: {colour:?}");
    tomu.led(colour).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .compact()
        .init();

    let opts = CliParser::parse();
    let mut tomu = TomuUsbSimple::open().await?;
    let atem = AtemController::connect_udp(
        SocketAddrV4::new(opts.ip.parse().unwrap(), 9910),
        opts.reconnect,
    )
    .await?;
    let state = atem.get_state().await;
    info!(
        "Connected ATEM switch: {:?}, FW v{}",
        state.product_name, state.version
    );

    // Set initial state
    if let Some(&tally) = state.tally_by_source.get(&opts.source) {
        update_tomu(&mut tomu, tally).await?;
    }

    loop {
        let Ok((state, update)) = atem.state_update_events().recv().await else {
            panic!("oops");
        };

        if !update.contains(StateUpdate::TALLY_BY_SOURCE) {
            continue;
        }

        let Some(&tally) = state.tally_by_source.get(&opts.source) else {
            // Unknown source state
            continue;
        };

        update_tomu(&mut tomu, tally).await?;
    }
}
