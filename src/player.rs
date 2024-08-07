use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self , app: &mut App) {
        app
        .add_systems(Update, (control, debug_player))
        .add_systems(Startup, spawn_player);
    }
}


// Spawn Shape Player
fn spawn_player(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>

) {
    // cube
	commands.spawn((PbrBundle {
		mesh: meshes.add(Cuboid::new(1.0,1.0,1.0)), // This is Box
		material: materials.add(Color::srgb_u8(124, 144, 255)), // Color
		transform : Transform::from_xyz(0.0, 0.5, 0.0), //Transform
		..default()
	},
    Player::new()
    ));
}


#[derive(Component)]
pub struct Player {
    speed:f32,      // Speed Player
    toggle_c: bool, // Controling Player 
}

impl Player {
    pub fn new() -> Self {
        Player {
            speed: 2.0,
            toggle_c: false,
        }
    }
}

fn control(
    mut player_q: Query<(&mut Transform, &mut Player)>, 
    mut cam_q: Query<&mut Transform, (With<Camera3d>, Without<Player>)>,
    time:Res<Time>, 
    input:ResMut<ButtonInput<KeyCode>>) {

    if !player_q.single().1.toggle_c { return; }

    let speed = player_q.single().1.speed;

    let mut transform = player_q.single_mut().0;
    
    let camera = match cam_q.get_single() {
        Ok(c) => c,
        Err(why) => panic!("Error to get Camera in player : {}",why)
    };

    let mut direction = Vec3::ZERO;


    if input.pressed(KeyCode::KeyW) {
        direction += camera.forward().as_vec3();
    }
    if input.pressed(KeyCode::KeyS) {
        direction += camera.back().as_vec3();
    }
    if input.pressed(KeyCode::KeyA) {
        direction += camera.left().as_vec3();
    }
    if input.pressed(KeyCode::KeyD) {
        direction += camera.right().as_vec3();
    }

    direction.y = 0.0;

    let movement = direction.normalize_or_zero() * time.delta_seconds() * 2.0;
    transform.translation += movement;
    
    
}

fn debug_player(
    mut player: Query<(&mut Transform,&mut Player)>, 
    input:ResMut<ButtonInput<KeyCode>>) {
    
    if input.just_released(KeyCode::KeyF) {
        
        player.single_mut().1.toggle_c = !player.single().1.toggle_c; // Reverse yourself

        println!("PLAYER MOVEABLE : {}", player.single().1.toggle_c);
    }
}