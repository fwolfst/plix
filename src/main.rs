use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};

const PLIX_SIZE :f32 = 3.0;
const GOAL_SIZE :f32 = 30.0;

const SCREEN_WIDTH  :i32 = 480;
const SCREEN_HEIGHT :i32 = 480;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    Playing,
    Won,
}

#[derive(Component, Debug)]
struct Plix {}

#[derive(Component, Debug)]
struct Goal {}

fn spawn_cockpit(mut commands:Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(TextBundle::from_section(
            "Plix - a lonely white square tries to meet the red area",
           TextStyle {
                color: Color::WHITE,
                ..default()
            }
            ));
}

fn spawn_plix(mut commands: Commands) {
    // White square
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(PLIX_SIZE, PLIX_SIZE)),
                ..default()
            },
            ..default()
        },
        Plix {}
        ));
}

fn spawn_goal(mut commands: Commands) {
    // Red rectangle/sprite at bottom of screen
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(GOAL_SIZE, 3.0)),
                color: Color::RED,
                ..default()
            },
            transform: Transform::from_translation(
                           Vec3::new(
                               0.0,
                               //SCREEN_WIDTH as f32 / 2.0 - GOAL_SIZE as f32 / 2.0,
                               - SCREEN_HEIGHT as f32 / 2.0,
                               0.0
                               )
                           ),
            ..default()
        },
        Goal {}
        ));
}

fn check_win(mut next_state: ResMut<NextState<GameState>>,
             the_plix: Query<&Transform, &Plix>,
             the_goal: Query<&Transform, &Goal>) {
    // Is White square in goal?
    let goal_x = the_goal.single().translation.x;
    let goal_y = the_goal.single().translation.y;
    let plix_pos = the_plix.single().translation.xy();
    if plix_pos.x < goal_x + 15.0 && plix_pos.x > goal_x - 15.0 
        && plix_pos.y < goal_y + 1.0 && plix_pos.y > goal_y - 1.0 {
        info!("Hit goal");
        next_state.set(GameState::Won)
    }
}

fn win(mut commands: Commands, sprites: Query<Entity, &Sprite>) {
    // despawn, but spanw text.
    for entity in sprites.iter() {
        commands.entity(entity).despawn();
    }
    commands.spawn(
        TextBundle::from_section(
            "You have WON",
            TextStyle {
                color: Color::RED,
                font_size: 22.0,
                ..default()
            }
        ).with_style( Style {
            left: Val::Px(30.0),
            top: Val::Px(130.0),
            ..default()
        })
    );
}
// Transform { translation { 30.9, 50.0, 0.0 }}
fn move_plix(mut the_plix: Query<&mut Transform, &Plix>,
             keyboard: Res<Input<KeyCode>>) {
    let speed = 2.0;
    for mut plix in &mut the_plix {
        if keyboard.pressed(KeyCode::W)  || keyboard.pressed(KeyCode::Up) {
            plix.translation.y += speed;
        }
        if keyboard.pressed(KeyCode::S) || keyboard.pressed(KeyCode::Down) {
            plix.translation.y -= speed;
        }
        if keyboard.pressed(KeyCode::A) || keyboard.pressed(KeyCode::Left) {
            plix.translation.x -= speed;
        }
        if keyboard.pressed(KeyCode::D) || keyboard.pressed(KeyCode::Right) {
            plix.translation.x += speed;
        }
    }
}

fn main() {
    App::new()
        .add_plugins(
            (
                DefaultPlugins
                     .set(WindowPlugin {
                         primary_window: Some(Window {
                             title: "Plix".into(),
                             resolution: (SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32).into(),
                             // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                             prevent_default_event_handling: false,
                             resizable: false,
                             .. default()
                         }),
                         .. default()
                     }),
                 LogDiagnosticsPlugin::default(),
                 FrameTimeDiagnosticsPlugin,
            ))
        .add_state::<GameState>() // will be renamed to ini_state in next bevy
        .add_systems(Startup, (spawn_cockpit, spawn_plix, spawn_goal))
        .add_systems(Update,
                     (check_win.run_if(in_state(GameState::Playing)),
                     move_plix.run_if(in_state(GameState::Playing)),
                     bevy::window::close_on_esc
                     )
                     )
        .add_systems(OnEnter(GameState::Won), win)
        .run();
}
