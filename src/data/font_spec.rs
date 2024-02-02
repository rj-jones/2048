use bevy::prelude::*;

// A font is required to display any text, so we define that here
#[derive(Resource)]
pub struct FontSpec {
    // could be called anything
    pub family: Handle<Font>,
}

impl FromWorld for FontSpec {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource_mut::<AssetServer>().unwrap();
        FontSpec {
            family: asset_server.load("fonts/FiraCode-Bold.ttf"),
        }
    }
}
