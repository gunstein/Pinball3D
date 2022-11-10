use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::Floor;
pub struct LauncherPlugin;

impl Plugin for LauncherPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system_to_stage(StartupStage::PostStartup, spawn_launcher)
            .add_system(launcher_movement);
    }
}

#[derive(Component)]
struct Launcher{
    start_pos : Vec3,
 }

fn spawn_launcher(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query_floors: Query<Entity, With<Floor>>
) {
    let mut floor = None;
    for entity in query_floors.iter(){
        floor = Some(entity);
    }

    let launcher_pos = Vec3::new(0.34, -0.95, 0.03);
    let launcher_mesh_handle:Handle<Mesh> = meshes.add(Mesh::from(shape::Box::new(0.02*2.0,0.02*2.0, 0.02*2.0)));
    let material_launcher = materials.add(Color::YELLOW.into());
 
    let launcher = commands.spawn()
    .insert_bundle(PbrBundle {
        mesh: launcher_mesh_handle.clone(),
        material: material_launcher.clone(),
        ..default()
    })
    .insert(RigidBody::KinematicPositionBased)
    .insert(Collider::cuboid(0.02, 0.02, 0.02))
    .insert_bundle(TransformBundle::from(Transform::from_xyz(launcher_pos.x, launcher_pos.y, launcher_pos.z)))
    .insert(Launcher{start_pos: launcher_pos})
    .id();

    commands.entity(floor.unwrap()).add_child(launcher);
}

fn launcher_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut launchers: Query<(&mut Launcher, &mut Transform), With<Launcher>>,
) {
    for (launcher, mut launcher_transform) in launchers.iter_mut() {
        let mut next_ypos = launcher_transform.translation.y;
        
        if keyboard_input.pressed(KeyCode::Space)
        {
            next_ypos = next_ypos + 0.03;
        }
        else
        {
            next_ypos = next_ypos - 0.02;
        }   
        let clamped_ypos = next_ypos.clamp(launcher.start_pos.y, launcher.start_pos.y +  0.05);
        launcher_transform.translation.y = clamped_ypos;    
    }
}
