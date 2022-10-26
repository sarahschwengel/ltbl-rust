use std::error::Error;
use std::result::Result::Ok;

use clap::Parser;

use hueclient::Bridge;
use hueclient::CommandLight;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input filepath
    #[arg(short, long)]
    input: String,

    // /// Output filepath
    // #[arg(short, long)]
    // output: String,
    /// Valid light id's
    #[arg(short, long)]
    ids: Vec<usize>,
}

pub fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    // // You only need to do this once
    // light::initialize_bridge()?;

    // do some light stuff
    const USERNAME: &str = "CLzg-pVKFJaHduKKfOs65TsS6IL7wxtxi9OaB78O";
    let bridge = Bridge::discover_required().with_user(USERNAME);

    for light in &bridge.get_all_lights().unwrap() {
        if args.ids.contains(&light.id) {
            let cmd = CommandLight::default().on();
            bridge.set_light_state(light.id, &cmd).unwrap();
        }
    }

    Ok(())
}
