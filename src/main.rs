use bevy::prelude::*;
use camera::CameraPlayerPlugin;
use player::PlayerPlugin;
use world::WorldPlugin;


mod player;
mod world;
mod camera;

fn main(){
  App::new()
    .insert_resource(ClearColor(Color::BLACK)) // used to clear the screen between frames
    .add_plugins((DefaultPlugins, PlayerPlugin , WorldPlugin, CameraPlayerPlugin))
    .run();
}
