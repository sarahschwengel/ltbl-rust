use hueclient::{Bridge, CommandLight, Result};

pub fn list_lights(bridge: &Bridge) {
    for light in bridge.get_all_lights().unwrap() {
        println!("id: {}, name: {}", light.id, light.light.name);
    }
}

pub fn initialize_bridge() -> Result<()> {
    let bridge: Bridge = Bridge::discover_required().register_user("sarah@bedevere")?;
    println!("username: {}", bridge.username);

    Ok(())
}

pub fn set_lights_xy(bridge: &Bridge, light_ids: &[usize], xy: (f32, f32)) -> Result<()> {
    let cmd = CommandLight::default().on().with_xy(xy.0, xy.1);
    set_lights(bridge, light_ids, &cmd)?;

    Ok(())
}

pub fn set_lights_hs(bridge: &Bridge, light_ids: &[usize], hs: (u16, u8)) -> Result<()> {
    let cmd = CommandLight::default().on().with_hue(hs.0).with_sat(hs.1);
    set_lights(bridge, light_ids, &cmd)?;

    Ok(())
}

fn set_lights(bridge: &Bridge, light_ids: &[usize], cmd: &CommandLight) -> Result<()> {
    for light in bridge.get_all_lights().unwrap() {
        if light_ids.contains(&light.id) {
            bridge.set_light_state(light.id, cmd)?;
        }
    }

    Ok(())
}
