use std::error::Error;

use hueclient::Bridge;

pub fn change_color() {}

pub fn list_lights() {}

pub fn initialize_bridge() -> Result<(), Box<dyn Error>> {
    let bridge: Bridge = Bridge::discover_required().register_user("sarah@bedevere")?;
    println!("username: {}", bridge.username);

    Ok(())
}
