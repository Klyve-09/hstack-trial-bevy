use bevy::prelude::*;

#[derive(States, Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Screen {
    #[default]
    Menu,
    Playing,
    Settings,
}

#[derive(Component)]
pub struct Player;

#[derive(Component, Clone, Copy)]
pub struct Obstacle {
    pub center: Vec2,
    pub size: Vec2,
}

pub const HALF_ARENA: Vec2 = Vec2::new(350.0, 230.0);
pub const PLAYER_SIZE: f32 = 26.0;
const SPEED: f32 = 220.0;

pub fn navigate(
    keys: Res<ButtonInput<KeyCode>>,
    state: Res<State<Screen>>,
    mut next: ResMut<NextState<Screen>>,
) {
    match state.get() {
        Screen::Menu if keys.just_pressed(KeyCode::Enter) => next.set(Screen::Playing),
        Screen::Menu if keys.just_pressed(KeyCode::KeyS) => next.set(Screen::Settings),
        Screen::Playing | Screen::Settings if keys.just_pressed(KeyCode::Escape) => {
            next.set(Screen::Menu)
        }
        _ => {}
    }
}

pub fn movement(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut players: Query<&mut Transform, With<Player>>,
    obstacles: Query<&Obstacle>,
) {
    let mut direction = Vec2::ZERO;
    for (codes, axis) in [
        ([KeyCode::KeyA, KeyCode::ArrowLeft], Vec2::NEG_X),
        ([KeyCode::KeyD, KeyCode::ArrowRight], Vec2::X),
        ([KeyCode::KeyW, KeyCode::ArrowUp], Vec2::Y),
        ([KeyCode::KeyS, KeyCode::ArrowDown], Vec2::NEG_Y),
    ] {
        if keys.any_pressed(codes) {
            direction += axis;
        }
    }
    // Small substeps keep a slow frame from jumping through an obstacle.
    let displacement = direction.normalize_or_zero() * SPEED * time.delta_secs().min(0.25);
    let steps = (displacement.length() / (PLAYER_SIZE / 2.0))
        .ceil()
        .max(1.0) as u32;
    let step = displacement / steps as f32;
    let limit = HALF_ARENA - Vec2::splat(PLAYER_SIZE / 2.0);
    for mut transform in &mut players {
        let mut position = transform.translation.truncate();
        for _ in 0..steps {
            for delta in [Vec2::new(step.x, 0.0), Vec2::new(0.0, step.y)] {
                let candidate = (position + delta).clamp(-limit, limit);
                let blocked = obstacles.iter().any(|obstacle| {
                    let distance = (candidate - obstacle.center).abs();
                    let extent = (obstacle.size + Vec2::splat(PLAYER_SIZE)) / 2.0;
                    distance.x < extent.x && distance.y < extent.y
                });
                if !blocked {
                    position = candidate;
                }
            }
        }
        transform.translation = position.extend(1.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::{state::app::StatesPlugin, time::TimeUpdateStrategy};
    use std::time::Duration;

    fn app() -> App {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, StatesPlugin))
            .init_state::<Screen>()
            .init_resource::<ButtonInput<KeyCode>>()
            .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f32(
                0.1,
            )))
            .add_systems(Update, navigate)
            .add_systems(Update, movement.run_if(in_state(Screen::Playing)));
        app.update();
        app
    }

    fn press(app: &mut App, key: KeyCode) {
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(key);
        app.update();
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .reset_all();
        app.update();
    }

    #[test]
    fn menu_settings_and_game_transitions() {
        let mut app = app();
        press(&mut app, KeyCode::KeyS);
        assert_eq!(
            *app.world().resource::<State<Screen>>().get(),
            Screen::Settings
        );
        press(&mut app, KeyCode::Escape);
        assert_eq!(*app.world().resource::<State<Screen>>().get(), Screen::Menu);
        press(&mut app, KeyCode::Enter);
        assert_eq!(
            *app.world().resource::<State<Screen>>().get(),
            Screen::Playing
        );
        press(&mut app, KeyCode::Escape);
        assert_eq!(*app.world().resource::<State<Screen>>().get(), Screen::Menu);
    }

    #[test]
    fn movement_uses_delta_and_only_runs_while_playing() {
        let mut app = app();
        let player = app.world_mut().spawn((Player, Transform::default())).id();
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyD);
        app.update();
        assert_eq!(
            app.world().get::<Transform>(player).unwrap().translation.x,
            0.0
        );
        app.world_mut()
            .resource_mut::<NextState<Screen>>()
            .set(Screen::Playing);
        app.update();
        assert!((app.world().get::<Transform>(player).unwrap().translation.x - 22.0).abs() < 0.01);
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyW);
        let before = app.world().get::<Transform>(player).unwrap().translation;
        app.update();
        let after = app.world().get::<Transform>(player).unwrap().translation;
        assert!((after.distance(before) - 22.0).abs() < 0.01);
    }

    #[test]
    fn obstacles_and_arena_stop_the_player() {
        let mut app = app();
        let player = app.world_mut().spawn((Player, Transform::default())).id();
        app.world_mut().spawn(Obstacle {
            center: Vec2::new(50.0, 0.0),
            size: Vec2::splat(30.0),
        });
        app.world_mut()
            .resource_mut::<NextState<Screen>>()
            .set(Screen::Playing);
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::ArrowRight);
        for _ in 0..20 {
            app.update();
        }
        assert!(app.world().get::<Transform>(player).unwrap().translation.x <= 22.0);
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .reset_all();
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::ArrowLeft);
        for _ in 0..40 {
            app.update();
        }
        let x = app.world().get::<Transform>(player).unwrap().translation.x;
        assert!((x + HALF_ARENA.x - PLAYER_SIZE / 2.0).abs() < 0.01);
    }
}
