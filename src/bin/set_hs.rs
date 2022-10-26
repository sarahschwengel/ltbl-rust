use std::result::Result::Ok;
use std::u8;

use clap::Parser;

use hueclient::Bridge;
use hueclient::Result;
use ltbl::light;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    // /// Input filepath
    // #[arg(short, long)]
    // input: String,

    // /// Output filepath
    // #[arg(short, long)]
    // output: String,
    /// Valid light id's
    #[arg(short, long)]
    ids: Vec<usize>,
    // /// red channel
    // #[arg(short)]
    // red: u8,
    //
    // /// green channel
    // #[arg(short)]
    // green: u8,
    //
    // /// blue channel
    // #[arg(short)]
    // blue: u8,
    // /// x channel
    // #[arg(short)]
    // x: f32,
    //
    // /// y channel
    // #[arg(short)]
    // y: f32,
    /// Hue channel
    #[arg(short, long)]
    hue: u16,

    /// Saturation channel
    #[arg(short, long)]
    saturation: u8,
}

pub fn main() -> Result<()> {
    let args = Args::parse();

    // // You only need to do this once
    // light::initialize_bridge()?;

    // do some light stuff
    const USERNAME: &str = "CLzg-pVKFJaHduKKfOs65TsS6IL7wxtxi9OaB78O";
    let bridge = Bridge::discover_required().with_user(USERNAME);

    light::set_lights_hs(&bridge, &args.ids, (args.hue, args.saturation))?;

    Ok(())
}
