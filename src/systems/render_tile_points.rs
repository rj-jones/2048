use bevy::prelude::*;

use crate::data::{points::Points, tile_text::TileText};

pub fn render_tile_points(
    mut texts: Query<&mut Text, With<TileText>>, // query for all text components labeled with TileText
    tiles: Query<(&Points, &Children)>, // query for entities with Points & Children (will give us all tiles)
) {
    for (points, children) in tiles.iter() {
        // the first child is going to be the text component
        if let Some(entity) = children.first() {
            // We can use the texts query like a database, passing in the id to get a mutable entity.
            let mut text = texts.get_mut(*entity).expect("expected Text to exist");
            let text_section = text.sections.first_mut().expect("expected first section");
            text_section.value = points.value.to_string();
        }
    }
}
