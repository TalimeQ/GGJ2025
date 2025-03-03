use std::collections::HashMap;
use bevy::prelude::*;
use crate::game_data::GameData;
use crate::generator::MainCamera;
use crate::input::{MagicItem, MouseData};

#[derive(Component)]
pub struct GoldTrackerSprite
{
    order: usize
}

#[derive(Component)]
pub struct ActiveUpgradeTracker
{
    order: u32
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

    //god gives his toughest battles to his strongest soldiers
    // i'm not one of them please make it stop

    let image1: Handle<Image> = asset_server.load("sprites/bubblebomb.png");
    let image2: Handle<Image> = asset_server.load("sprites/bomb.png");
    let image3: Handle<Image> = asset_server.load("sprites/nuke.png");
    let image4: Handle<Image> = asset_server.load("sprites/noso.png");

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
                        Transform::from_xyz(offset_item_x,offset_item_y,2.), ActiveUpgradeTracker{order : i} ));

        if i == 0
        {
            commands.spawn((Sprite::from_image(image1.clone()),
                Transform::from_xyz(offset_item_x,offset_item_y,5.)));
        }
        else if i == 1
        {
            commands.spawn((Sprite::from_image(image2.clone()),
                            Transform::from_xyz(offset_item_x,offset_item_y,5.)));
        }
        else if i == 3
        {
            commands.spawn((Sprite::from_image(image3.clone()),
                            Transform::from_xyz(offset_item_x,offset_item_y,5.)));
        }
        else if i == 2
        {
            commands.spawn((Sprite::from_image(image4.clone()),
                            Transform::from_xyz(offset_item_x,offset_item_y,5.)));
        }

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
                            Transform::from_xyz(initial_offset_x,initial_offset_y,1.),GoldTrackerSprite{ order: i}));

        }
        else
        {
            commands.spawn((Sprite {
                image: image.clone(),
                texture_atlas : Some(TextureAtlas
                {layout : sprite_atlas.0.clone(), index : in_index}), .. default() },
                            Transform::from_xyz(initial_offset_x,initial_offset_y,1.)
            ));
        }
    }
}

pub fn update_gold_tracker(
    mut query : Query<(&mut Sprite, &GoldTrackerSprite), With<GoldTrackerSprite>>,
    game_data: Res<GameData>)
{
    let zero_index = 32;
    let mut cache_cash = game_data.player_currency;
    let mut iter = 100_000;


    for i in [11, 10, 9, 8,7, 6]
    {
        let mut sprite = query.iter_mut()
            .find_map(|a| if a.1.order == i { Some(a.0) } else { None });

        if let Some(s) = &mut sprite
        {
            if let Some(atlas) = &mut s.texture_atlas
            {
                atlas.index = zero_index + ((cache_cash %10) as usize);
                cache_cash /= 10;
            }
        }
    }

    // for mut sprite in query.iter_mut()
    // {
    //     if let Some(atlas) = &mut sprite.0.texture_atlas
    //     {
    //         atlas.index = zero_index + (cache_cash / iter ) as usize;
    //         cache_cash = cache_cash % iter
    //     }
    //
    //     iter /= 10;
    // }
}
pub fn update_active_ability(
    mut query : Query<(&mut Sprite, &ActiveUpgradeTracker), With<ActiveUpgradeTracker>>,
    game_data: Res<GameData>,
    mouse_data: Res<MouseData>)
{
    for mut sprite in query.iter_mut()
    {
        if let Some(atlas) = &mut sprite.0.texture_atlas
        {
            atlas.index = 0;
        }
    }

    let active_order = match mouse_data.equipped_magic_item
    {
        MagicItem::PiuPiuPiu(_) => 0,
        MagicItem::Piuuum(_, _) => 1,
        MagicItem::KaBum(_, _) => 3,
        _ => 2
    };

    let mut active = query.iter_mut()
        .find_map(|a| if a.1.order == active_order { Some(a.0) } else { None });

    if let Some(a) = &mut active
    {
        if let Some(atlas) = &mut a.texture_atlas
        {
            atlas.index = 1;
        }
    }
}