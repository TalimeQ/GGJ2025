use bevy::asset::{Assets, Handle};
use bevy::color::{Color, Srgba};
use bevy::image::Image;
use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(Resource, AssetCollection)]
pub struct MapSource {
    #[asset(path = "maps/map.png")]
    map_tiles: Handle<Image>,
}

const ENEMY_CELL: &str = "ac3232"; //RED

pub fn prepare_map(mut commands: Commands, map_source: Res<MapSource>, images: Res<Assets<Image>>) {

    if let Some(map) = images.get(&map_source.map_tiles) {
        for i in 0..map.width() {
            for j in 0..map.height() {
                let color : Srgba = map.get_color_at(i, j).unwrap().into();
                if color == Srgba::hex(&ENEMY_CELL).unwrap()
                {
                    println!("i:{i} j:{j} IS ENEMY!", i = i, j = j );
                }
            }
        }
    }
}