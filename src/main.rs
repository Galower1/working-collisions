use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};
use rand::Rng;

const TILE_DIMENSIONS: f32 = 64.;
const GRAVITY: f32 = 3000.;
const JUMP_SPEED: f32 = 1000.;
const MOVEMENT_SPEED: f32 = 200.;
const SIZE: Vec2 = Vec2::new(TILE_DIMENSIONS, TILE_DIMENSIONS);

#[derive(Component, Copy, Clone, Debug)]
struct Tile {
    composite: f32,
}

impl Default for Tile {
    fn default() -> Self {
        Self {
            composite: TILE_DIMENSIONS,
        }
    }
}

#[derive(Component)]
struct Player;

#[derive(Component, Default)]
struct Speed {
    speed_x: f32,
    speed_y: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (physics, collision, keyboard_input))
        .run()
}

fn setup(mut commands: Commands, query: Query<&Window>) {
    commands.spawn(Camera2dBundle::default());

    let window = query.single();
    let mut rng = rand::thread_rng();

    let floor = -(window.height() / 2.);

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25 * rng.gen::<f32>(), 0.75),
                custom_size: Some(SIZE),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(
                TILE_DIMENSIONS * 3.,
                floor + TILE_DIMENSIONS * 3.,
                0.,
            )),
            ..default()
        },
        Tile::default(),
    ));

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25 * rng.gen::<f32>(), 0.75),
                custom_size: Some(SIZE),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(
                TILE_DIMENSIONS * 2.,
                floor + TILE_DIMENSIONS,
                0.,
            )),
            ..default()
        },
        Tile::default(),
    ));

    // SPAWN TILES
    for i in -200..200 {
        let pos_x = i as f32 * TILE_DIMENSIONS;

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.25, 0.25 * rng.gen::<f32>(), 0.75),
                    custom_size: Some(SIZE),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(pos_x, floor, 0.)),
                ..default()
            },
            Tile::default(),
        ));
    }

    // SPAWN PLAYER
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.9, 0.75),
                custom_size: Some(SIZE),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            ..default()
        },
        Player,
        Speed::default(),
    ));
}

fn physics(mut query: Query<(&mut Transform, &mut Speed), With<Player>>, time: Res<Time>) {
    let (mut player_transform, mut player_speed) = query.single_mut();

    player_transform.translation.y += player_speed.speed_y * time.delta_seconds();

    player_speed.speed_y -= GRAVITY * time.delta_seconds();
}

fn collision(
    mut set: ParamSet<(
        Query<(&mut Transform, &mut Tile)>,
        Query<(&mut Transform, &mut Speed), With<Player>>,
    )>,
) {
    let mut binding = set.p0();
    let mut tiles: Vec<(Mut<'_, Transform>, Mut<'_, Tile>)> = binding.iter_mut().collect();

    let mut composite_tiles: Vec<(Vec3, Tile)> = Vec::new();

    let mut connected_counter = 1;
    for i in 1..tiles.len() {
        tiles[i - 1].0.translation.z *= -(i as f32);
        if (tiles[i].0.translation.x - tiles[i - 1].0.translation.x).abs() <= TILE_DIMENSIONS
            && tiles[i].0.translation.y == tiles[i - 1].0.translation.y
        {
            tiles[i - connected_counter].1.composite += TILE_DIMENSIONS;
            connected_counter += 1;
        } else {
            connected_counter = 1;
        }
        composite_tiles.push((tiles[i - 1].0.translation, *tiles[i - 1].1));
    }

    let mut binding = set.p1();
    let (mut player_transform, mut player_speed) = binding.single_mut();

    let player_position = player_transform.translation;

    for (tile_position, tile) in composite_tiles {
        if let Some(collision) = collide(
            player_position,
            SIZE,
            tile_position,
            Vec2::new(tile.composite, TILE_DIMENSIONS),
        ) {
            match collision {
                Collision::Top => {
                    player_speed.speed_y = 0.;
                    player_transform.translation.y = tile_position.y + TILE_DIMENSIONS;
                }
                Collision::Bottom => {
                    player_speed.speed_y = -GRAVITY;
                    player_transform.translation.y = tile_position.y - TILE_DIMENSIONS;
                }
                Collision::Left => {
                    player_speed.speed_x = 0.;
                    player_transform.translation.x = tile_position.x - TILE_DIMENSIONS;
                }
                Collision::Right => {
                    player_speed.speed_x = 0.;
                    player_transform.translation.x = tile_position.x + TILE_DIMENSIONS;
                }
                Collision::Inside => {
                    player_speed.speed_y = 0.;
                    player_transform.translation.y = tile_position.y + TILE_DIMENSIONS;
                }
            }
        }
    }
}

fn keyboard_input(
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&mut Speed, &mut Transform), With<Player>>,
    time: Res<Time>,
) {
    let (mut player_speed, mut player_transform) = query.single_mut();

    if keys.pressed(KeyCode::Space) {
        player_speed.speed_y = JUMP_SPEED;
    }

    if keys.pressed(KeyCode::D) {
        player_transform.translation.x += MOVEMENT_SPEED * time.delta_seconds();
    }

    if keys.pressed(KeyCode::A) {
        player_transform.translation.x -= MOVEMENT_SPEED * time.delta_seconds();
    }
}
