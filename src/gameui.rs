use std::collections::HashMap;
use bevy::prelude::*;
use crate::game_data::GameData;
use crate::generator::MainCamera;


#[derive(Component)]
pub struct GoldTrackerSprite
{

}

#[derive(Component)]
pub struct ActiveUpgradeTracker
{

}

#[derive(Resource)]
pub struct ActiveSheet(Handle<TextureAtlasLayout>);
impl FromWorld for ActiveSheet {
    fn from_world(world: &mut World) -> Self {
        let texture_atlas = TextureAtlasLayout::from_grid(
            (32, 32).into(), // The size of each image
            2,               // The number of columns
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


#[derive(Resource)]
pub struct LetterSheet(Handle<TextureAtlasLayout>);
impl FromWorld for LetterSheet {
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
             sprite_atlas : Res<LetterSheet>,
                active_atlas : Res<ActiveSheet>)
{
    let mut initial_offset_x = -640.0;
    let initial_offset_y = 320.0;
    let image : Handle<Image> = asset_server.load("sprites/font_in_row_unselected_sheet.png");
    let image_actives : Handle<Image> = asset_server.load("sprites/SpriteInActive.png");

    commands.spawn((Camera2d::default(), MainCamera));

    commands.spawn(
        (Sprite::from_image(asset_server.load("sprites/GoldBoard.png")),
         Transform::from_xyz(initial_offset_x + 96.0 ,initial_offset_y - 48.0,1.)));

    let mut offset_item_x = initial_offset_x + 38.0;
    let offset_item_y = initial_offset_y - 48.0;

    for i in 0 .. 4
    {
        commands.spawn((Sprite {
            image: image_actives.clone(),
            texture_atlas : Some(TextureAtlas
            {layout : active_atlas .0.clone(), index : 0}), .. default() },
                        Transform::from_xyz(offset_item_x,offset_item_y,2.)));
        offset_item_x += 38.0;
    }

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
    mut query : Query<&mut Sprite, With<GoldTrackerSprite>>,
    game_data: Res<GameData>
)
{
    let zero_index = 32;
    let cache_cash = game_data.player_currency;
    let mut iter = 100000;


    for mut sprite in query.iter_mut()
    {
        if let Some(atlas) = &mut sprite.texture_atlas
        {
            atlas.index = zero_index + (cache_cash / iter ) as usize;
        }

        iter /= 10;
    }
}
pub fn update_active_ability(mut query : Query<&mut Sprite, With<GoldTrackerSprite>>)
{

}