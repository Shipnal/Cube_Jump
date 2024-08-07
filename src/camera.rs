use bevy::window::CursorGrabMode;
use bevy::{math::VectorSpace, prelude::*};
use bevy::input::mouse::MouseMotion;

pub struct CameraPlayerPlugin;

impl Plugin for CameraPlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, (control_cam,camera_head ,debug_cam))
        .add_systems(Startup, spwan_cam);
    }
}


fn spwan_cam(
    mut commands: Commands,
    mut windows: Query<&mut Window>,
) {
    commands.spawn((Camera3dBundle {
        transform:Transform::from_xyz(-2.5 , 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    }, CameraSystem::new()));

    windows.single_mut().cursor.visible = false;
    windows.single_mut().cursor.grab_mode = CursorGrabMode::Locked;
}


#[derive(Component)]
pub struct CameraSystem {
    toggle_c:bool, // toggle control
    pitch: f32, // To limit the pitch (vertical rotation) of the camera to 90 degrees and -90 degrees
    sensitivity: f32,
    speed:f32,
}

impl CameraSystem {
    pub fn new() -> Self {
        CameraSystem {
            toggle_c: true,
            pitch: 0.0,
            sensitivity: 0.5,
            speed: 2.5,
        }
    }
}

fn control_cam(
    mut cam_q: Query<(&mut Transform, &mut CameraSystem), With<Camera3d>>, 
    timer:Res<Time>,
    input_key: Res<ButtonInput<KeyCode>>) 
{

    if !cam_q.single().1.toggle_c {return;}

    let (mut camera , mut cam_sys) = cam_q.single_mut();
    
    let mut direction = Vec3::ZERO;

    if input_key.pressed(KeyCode::KeyW) {
        direction += camera.forward().as_vec3();
        
    }
    if input_key.pressed(KeyCode::KeyS) {
        direction += camera.back().as_vec3();
    }

    if input_key.pressed(KeyCode::KeyA) {
        direction += camera.left().as_vec3();
    }
    if input_key.pressed(KeyCode::KeyD) {
        direction += camera.right().as_vec3();
    }

    let movement =  direction.normalize_or_zero() * timer.delta_seconds() * cam_sys.speed;
    camera.translation += movement;


    if input_key.pressed(KeyCode::ShiftLeft) {
        camera.translation.y -= timer.delta_seconds() * cam_sys.speed;
    }
    if input_key.pressed(KeyCode::Space) {
        camera.translation.y += timer.delta_seconds() * cam_sys.speed;
    }

}

fn camera_head(
    mut cam_q: Query<(&mut Transform , &mut CameraSystem), With<Camera3d>>,
    time:Res<Time>,
    mut evr_motion: EventReader<MouseMotion>,
) {

    for ev in evr_motion.read() {

        let (mut camera , mut cam_sys) = cam_q.single_mut();

        let vertical = ev.delta.y * cam_sys.sensitivity * time.delta_seconds();
        let horizontal = ev.delta.x * cam_sys.sensitivity * time.delta_seconds();
        

        let yaw_rotation = Quat::from_rotation_y(-horizontal);
        camera.rotation = yaw_rotation * camera.rotation;
        
        cam_sys.pitch = (cam_sys.pitch - vertical * cam_sys.sensitivity)
        .clamp(-std::f32::consts::FRAC_PI_2, std::f32::consts::FRAC_PI_2);

        let pitch_rotation = Quat::from_rotation_x(cam_sys.pitch);
        let yaw = camera.rotation.to_euler(EulerRot::YXZ).0;
        camera.rotation = Quat::from_euler(EulerRot::YXZ, yaw, 0.0, 0.0) * pitch_rotation;
        
    }
}



fn debug_cam(
    input: Res<ButtonInput<KeyCode>>,
    mut cam_q: Query<(&mut Transform , &mut CameraSystem), With<Camera3d>>
) {
    if input.just_released(KeyCode::KeyF) {
        cam_q.single_mut().1.toggle_c = !cam_q.single().1.toggle_c ;
        println!("CAMERA MOVEABLE : {}", cam_q.single().1.toggle_c);
    }
}