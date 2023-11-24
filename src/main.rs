use bevy::{prelude::*, window::WindowResolution, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;

const WINDOW_WIDTH: f32 = 1024.0;
const WINDOW_HEIGHT: f32 = 720.0;
const WINDOW_BOTTOM_Y: f32 = WINDOW_HEIGHT / -2.0;
const WINDOW_LEFT_X: f32 = WINDOW_WIDTH / -2.0;
const COLOR_BACKGROUND: Color = Color::rgb(0.29, 0.31, 0.41);
const COLOR_PLATFORM: Color = Color::rgb(0.13, 0.13, 0.23);
const COLOR_PLAYER: Color = Color::rgb(0.60, 0.55, 0.60);
const COLOR_FLOOR: Color = Color::rgb(0.45, 0.55, 0.66);
const FLOOR_THICKNESS: f32 = 10.0;
const PLAYER_VELOCITY_X: f32 = 400.0;
const PLAYER_VELOCITY_Y: f32 = 850.0;
const MAX_JUMP_HEIGHT: f32 = 230.0;

fn main() {
    App::new()
    .insert_resource(ClearColor(COLOR_BACKGROUND))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Platformer".into(),
                resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(200.0))
        .add_plugins(RapierDebugRenderPlugin::default())

        .add_systems(Startup, setup)
        .add_systems(Update, movement)
        // .add_systems(Update, jump)
        // .add_systems(Update, rise)
        // .add_systems(Update, fall)

        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    // Spawn the platforms
    for (x, scale) in [
        (-100.0, Vec3::new(75.0, 200.0, 1.0)),
        (100.0, Vec3::new(50.0, 350.0, 1.0)),
        (350.0, Vec3::new(150.0, 260.0, 1.0)),
    ] {
        commands.spawn(PlatformBundle::new(x, scale));
    }
    // Spawn the floor
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: COLOR_FLOOR,
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, WINDOW_BOTTOM_Y + (FLOOR_THICKNESS / 2.0), 0.0),
                scale: Vec3::new(WINDOW_WIDTH, FLOOR_THICKNESS, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(0.5, 0.5));
    // Spawn the camera
    commands.spawn(Camera2dBundle::default());
    // Spawn the player
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::default().into()).into(),
        material: materials.add(ColorMaterial::from(COLOR_PLAYER)),
        transform: Transform {
            translation: Vec3::new(WINDOW_LEFT_X + 100.0, WINDOW_BOTTOM_Y + 30.0, 0.0),
            scale: Vec3::new(30.0, 30.0, 1.0),
            ..Default::default()
        },
        ..default()
    })
    .insert(RigidBody::KinematicPositionBased)
    .insert(Collider::ball(0.5))
    .insert(KinematicCharacterController::default());
}

fn movement(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut KinematicCharacterController>,
) {
    let mut player = query.single_mut();
    let mut movement = 0.0;

    for pressed in input.get_just_pressed() {
        match *pressed {
            KeyCode::Right => movement += time.delta_seconds() * PLAYER_VELOCITY_X,
            KeyCode::Left => movement += time.delta_seconds() * PLAYER_VELOCITY_X * -1.0,
            KeyCode::Space => player.translation = 
            _ => ()
        }
    }

    match player.translation {
        Some(vec) => player.translation = Some(Vec2::new(movement, vec.y)), // update if it already exists
        None => player.translation = Some(Vec2::new(movement, 0.0)),
    }
}

// #[derive(Component)]
// struct Jump(f32);

// fn jump(
//     input: Res<Input<KeyCode>>,
//     mut commands: Commands,
//     query: Query<Entity, (With<KinematicCharacterController>, Without<Jump>)>,
// ) {
//     if query.is_empty() {
//         return;
//     }

//     let player = query.single();

//     if input.pressed(KeyCode::Up) {
//         commands.entity(player).insert(Jump(0.0));
//     }
// }

// fn rise(
//     mut commands: Commands,
//     time: Res<Time>,
//     mut query: Query<(Entity, &mut KinematicCharacterController, &mut Jump)>,
// ) {
//     if query.is_empty() {
//         return;
//     }

//     let (entity, mut player, mut jump) = query.single_mut();

//     let mut movement = time.delta().as_secs_f32() * PLAYER_VELOCITY_Y;

//     if movement + jump.0 >= MAX_JUMP_HEIGHT {
//         movement = MAX_JUMP_HEIGHT - jump.0;
//         commands.entity(entity).remove::<Jump>();
//     }

//     jump.0 += movement;

//     match player.translation {
//         Some(vec) => player.translation = Some(Vec2::new(vec.x, movement)),
//         None => player.translation = Some(Vec2::new(0.0, movement)),
//     }
// }

// fn fall(time: Res<Time>, mut query: Query<&mut KinematicCharacterController, Without<Jump>>) {
//     if query.is_empty() {
//         return;
//     }

//     let mut player = query.single_mut();

//     // I am using two-thirds of the Y-velocity since I want the character to fall slower than it rises
//     let movement = time.delta().as_secs_f32() * (PLAYER_VELOCITY_Y / 1.5) * -1.0;

//     match player.translation {
//         Some(vec) => player.translation = Some(Vec2::new(vec.x, movement)),
//         None => player.translation = Some(Vec2::new(0.0, movement)),
//     }
// }

#[derive(Bundle)]
struct PlatformBundle {
    sprite_bundle: SpriteBundle,
    body: RigidBody,
    collider: Collider,
}

impl PlatformBundle {
    fn new(x: f32, scale: Vec3) -> Self {
        Self {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: COLOR_PLATFORM,
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(x, WINDOW_BOTTOM_Y + (scale.y / 2.0), 0.0),
                    scale,
                    ..Default::default()
                },
                ..Default::default()
            },
            body: RigidBody::Fixed,
            collider: Collider::cuboid(0.5, 0.5),
        }
    }
}