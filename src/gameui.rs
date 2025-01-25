use std::collections::HashMap;
use bevy::prelude::*;
use crate::game_data::GameData;
use crate::generator::MainCamera;


#[derive(Component)]
pub struct GoldTrackerSprite
{

}

#[derive(Resource)]
pub struct UiWrapperSheet(Handle<TextureAtlasLayout>);
impl FromWorld for  UiWrapperSheet {
    fn from_world(world: &mut World) -> Self {
        let texture_atlas = TextureAtlasLayout::from_grid(
            (32, 32).into(), // The size of each image
            42,               // The number of columns
            1,               // The number of rows
            None,            // Padding
            None,            // Offset
        );

        let mut texture_atlases = world
            .get_resource_mut::<Assets<TextureAtlasLayout>>()
            .unwrap();
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        Self(texture_atlas_handle)
    }
}

pub fn spawn_ui(mut commands: Commands,
             asset_server: Res<AssetServer>,
             sprite_atlas : Res<UiWrapperSheet>)
{
    let image : Handle<Image> = asset_server.load("sprites/font_in_row_unselected_sheet.png");

    commands.spawn((Camera2d::default(), MainCamera));
    let mut initial_offset_x = -640.0;
    let initial_offset_y = 320.0;

    // God has forsaken us or atleast me
    let x = [ 6,14,11,3,30,32,32,32,32,33,32];


    for i in 0 .. 11
    {
        initial_offset_x += 32.0;
        let in_index = x[i];
        if( i > 5)
        {
            commands.spawn((Sprite {
                image: image.clone(),
                texture_atlas : Some(TextureAtlas
                {layout : sprite_atlas.0.clone(), index : in_index}), .. default() },
                            Transform::from_xyz(initial_offset_x,initial_offset_y,1.)));

        }
        else
        {
            commands.spawn((Sprite {
                image: image.clone(),
                texture_atlas : Some(TextureAtlas
                {layout : sprite_atlas.0.clone(), index : in_index}), .. default() },
                            Transform::from_xyz(initial_offset_x,initial_offset_y,1.),GoldTrackerSprite{}
            ));
        }
    }
}

pub fn update_gold_tracker(
    query : Query<&Sprite, With<GoldTrackerSprite>>,
    game_data: Res<GameData>
)
{

}