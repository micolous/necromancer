#[macro_use]
extern crate tracing;

use clap::{ArgGroup, Parser, ValueHint};
use necromancer_protocol::{
    ay10::{ay10be_to_yuva422p10be, yuva422p10be_to_ay10be},
    rle::{RleCompressor, RleDecompressor},
    Error, IntReader,
};
use std::{
    fs::{File, OpenOptions},
    io::{BufReader, BufWriter, Read, Write},
    path::PathBuf,
};
use tracing_subscriber::{filter::LevelFilter, EnvFilter};

type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

const MAX_FILE_SIZE: usize = 7860 * 4680 * 10;

/// ATEM frame (image) RLE compression/decompression tool.
///
/// Frames are always encoded as `ay10` (bit-packed big-endian 10-bit 4:2:2:4
/// YUVA). You can convert to/from `yuva422p10be` format (usable by FFmpeg) with
/// --planar, but this (currently) uses significantly more memory.
///
/// To convert a 1920x1080 frame in BT.709 colour space to a 8bpc RGBA (lossy)
/// PNG with ffmpeg:
///
/// ```sh
/// rle --decompress --planar /tmp/still1.rle -o /tmp/still1.yuva422p10be
/// ffmpeg -f rawvideo -pixel_format yuva422p10be -colorspace bt709 \
///     -video_size 1920x1080 -i /tmp/still1.yuva422p10be \
///     -pix_fmt rgba /tmp/still1.'%03d'.png
/// ```
///
/// And encoded again, assuming the image is already the correct size, and using
/// the BT.709 colour space:
///
/// ```sh
/// ffmpeg -i /tmp/still2.png -f rawvideo -pix_fmt yuva422p10be \
///     -colorspace bt709 /tmp/still2.yuva422p10be
/// rle --compress --planar /tmp/still2.yuva422p10be -o /tmp/still2.rle
/// ```
#[derive(Debug, Parser)]
#[clap(verbatim_doc_comment)]
#[clap(group(
    ArgGroup::new("mode")
        .required(true)
        .args(&["compress", "decompress"])
))]
struct CliParser {
    /// Compresses a frame into a format suitable for the ATEM switcher
    #[clap(short, long)]
    compress: bool,

    /// Decompresses a frame from ATEM's switcher format
    #[clap(short, long)]
    decompress: bool,

    /// Input filename
    #[clap(value_hint = ValueHint::FilePath)]
    input: PathBuf,

    /// Output filename, required for --compress or --decompress
    #[clap(short, value_hint = ValueHint::FilePath)]
    output: Option<PathBuf>,

    /// Convert to/from planar format (ffmpeg's `yuva422p10be`), instead of
    /// (native) bit-packed YUVA (`ay10`).
    ///
    /// Converting to/from planar format uses *significantly* more memory.
    #[clap(long)]
    planar: bool,
}

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .compact()
        .init();

    let opts = CliParser::parse();
    if !opts.input.exists() || !opts.input.is_file() {
        panic!(
            "input file {} does not exist, or is not a file!",
            opts.input.display()
        );
    }

    let mut i = BufReader::new(File::open(opts.input)?);

    if opts.decompress || opts.compress {
        let output = opts
            .output
            .expect("output path is required for compression or decompression");
        let output_path = output.to_string_lossy().to_string();
        if output.exists() {
            panic!("output file {output_path} already exists, refusing to overwrite!");
        }

        let mut o = BufWriter::new(
            OpenOptions::new()
                .create_new(true)
                .write(true)
                .open(output)?,
        );

        if opts.decompress {
            let reader = RleDecompressor::new(IntReader::<_, u64>::new(i));
            if opts.planar {
                info!("converting to planar yuva422p10be...");
                let i: Vec<u64> = reader.collect();
                let i = ay10be_to_yuva422p10be(i.iter().copied());
                let l = i.len();
                info!("writing {l} bytes to {output_path}...");
                o.write(&i)?;
            } else {
                info!("writing bit-packed YUVA 4:2:2:4...");
                let mut l = 0;
                for w in reader {
                    o.write(&w.to_be_bytes())?;
                    l += 8;
                }
                info!("wrote {l} bytes to {output_path}.");
            }
        } else if opts.compress {
            if opts.planar {
                let length = i.get_ref().metadata()?.len() as usize;
                if length > MAX_FILE_SIZE {
                    error!("file is too large: {length} > {MAX_FILE_SIZE}");
                    return Err(Box::new(Error::InvalidLength));
                }

                info!("converting to bit-packed yuva422p10be...");
                let mut buf = vec![0; length];
                i.read_exact(&mut buf)?;
                let mut buf = yuva422p10be_to_ay10be(&buf)?;

                // RLE
                info!("writing {} bytes to {output_path}...", buf.len() * 8);
                let rle = RleCompressor::new(buf.drain(..));
                for w in rle {
                    o.write(&w.to_be_bytes())?;
                }
            } else {
                let rle = RleCompressor::new(IntReader::<_, u64>::new(i));
                let mut l = 0;
                for w in rle {
                    o.write(&w.to_be_bytes())?;
                    l += 8;
                }
                info!("wrote {l} bytes to {output_path}.");
            }
        } else {
            unreachable!();
        }
        o.flush()?;
    } else {
        unreachable!();
    }

    info!("all done!");
    Ok(())
}
