mod game;
mod settings;

use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use game::*;
use settings::{Settings, SettingsFile};

#[derive(Component)]
struct ScreenEntity;
#[derive(Component)]
struct SettingsText;
#[derive(Component)]
struct FpsText;
#[derive(Component)]
struct ScoreText;
#[derive(Resource, Default)]
struct SaveStatus(String);

fn main() {
    let file = SettingsFile::discover();
    let settings = file
        .0
        .as_ref()
        .map(|path| {
            Settings::load(path).unwrap_or_else(|error| {
                eprintln!("Could not load settings; using defaults: {error}");
                Settings::default()
            })
        })
        .unwrap_or_default();
    App::new()
        .insert_resource(settings)
        .insert_resource(file)
        .init_resource::<SaveStatus>()
        .insert_resource(ClearColor(Color::srgb(0.035, 0.055, 0.09)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Drift".into(),
                resolution: (800, 600).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .init_state::<Screen>()
        .add_plugins(ScorePlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (navigate, update_fps))
        .add_systems(Update, movement.run_if(in_state(Screen::Playing)))
        .add_systems(
            Update,
            update_score
                .after(collect_items)
                .run_if(in_state(Screen::Playing)),
        )
        .add_systems(
            Update,
            (toggle_settings, update_settings_text)
                .chain()
                .run_if(in_state(Screen::Settings)),
        )
        .add_systems(OnEnter(Screen::Menu), menu)
        .add_systems(OnEnter(Screen::Playing), playing)
        .add_systems(OnEnter(Screen::Settings), settings_screen)
        .add_systems(OnExit(Screen::Menu), clear_screen)
        .add_systems(OnExit(Screen::Playing), clear_screen)
        .add_systems(OnExit(Screen::Settings), clear_screen)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((
        FpsText,
        Text::new(""),
        TextFont {
            font_size: FontSize::Px(16.0),
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            right: px(20),
            top: px(16),
            ..default()
        },
    ));
}

fn label(commands: &mut Commands, content: &str, size: f32, top: f32) -> Entity {
    commands
        .spawn((
            ScreenEntity,
            Text::new(content),
            TextFont {
                font_size: FontSize::Px(size),
                ..default()
            },
            TextColor(Color::srgb(0.8, 0.9, 0.95)),
            Node {
                position_type: PositionType::Absolute,
                left: px(50),
                top: px(top),
                ..default()
            },
        ))
        .id()
}

fn menu(mut commands: Commands) {
    label(&mut commands, "DRIFT", 64.0, 130.0);
    label(&mut commands, "A quiet obstacle garden", 24.0, 220.0);
    label(
        &mut commands,
        "Press Enter to start\nS  Settings\nClose the window to quit",
        22.0,
        320.0,
    );
}

fn settings_screen(mut commands: Commands) {
    label(&mut commands, "Settings", 44.0, 130.0);
    let text = label(&mut commands, "", 24.0, 240.0);
    commands.entity(text).insert(SettingsText);
}

fn toggle_settings(
    keys: Res<ButtonInput<KeyCode>>,
    mut settings: ResMut<Settings>,
    file: Res<SettingsFile>,
    mut status: ResMut<SaveStatus>,
) {
    if !keys.just_pressed(KeyCode::KeyF) {
        return;
    }
    settings.show_fps = !settings.show_fps;
    status.0 = match &file.0 {
        Some(path) => match settings.save(path) {
            Ok(()) => "Saved on this device.".into(),
            Err(error) => format!("Could not save: {error}"),
        },
        None => "No device data directory is available; setting lasts this session.".into(),
    };
}

fn update_settings_text(
    settings: Res<Settings>,
    status: Res<SaveStatus>,
    mut text: Query<&mut Text, With<SettingsText>>,
) {
    for mut text in &mut text {
        **text = format!(
            "F  Show FPS: {}\n\nEsc  Back to menu\n\n{}",
            if settings.show_fps { "On" } else { "Off" },
            status.0
        );
    }
}

fn update_fps(
    settings: Res<Settings>,
    diagnostics: Res<DiagnosticsStore>,
    mut text: Query<&mut Text, With<FpsText>>,
) {
    for mut text in &mut text {
        **text = if settings.show_fps {
            diagnostics
                .get(&FrameTimeDiagnosticsPlugin::FPS)
                .and_then(|fps| fps.smoothed())
                .map(|fps| format!("FPS {fps:.0}"))
                .unwrap_or_else(|| "FPS …".into())
        } else {
            String::new()
        };
    }
}

fn update_score(score: Res<Score>, mut text: Query<&mut Text, With<ScoreText>>) {
    for mut text in &mut text {
        **text = format!("Score: {}", score.0);
    }
}

fn playing(mut commands: Commands) {
    label(
        &mut commands,
        "WASD / Arrow keys  Move     Esc  Menu",
        18.0,
        14.0,
    );
    let score_text = label(&mut commands, "Score: 0", 22.0, 0.0);
    commands.entity(score_text).insert((
        ScoreText,
        Node {
            position_type: PositionType::Absolute,
            left: px(50),
            bottom: px(14),
            ..default()
        },
    ));
    commands.spawn((
        ScreenEntity,
        Sprite::from_color(Color::srgb(0.07, 0.12, 0.18), HALF_ARENA * 2.0),
        Transform::from_xyz(0.0, 0.0, -1.0),
    ));
    commands.spawn((
        ScreenEntity,
        Player,
        Sprite::from_color(Color::srgb(0.3, 0.95, 0.8), Vec2::splat(PLAYER_SIZE)),
        Transform::from_translation(PLAYER_SPAWN.extend(1.0)),
    ));
    for position in ITEM_POSITIONS {
        commands.spawn((
            ScreenEntity,
            Item,
            Sprite::from_color(Color::srgb(1.0, 0.85, 0.2), Vec2::splat(ITEM_SIZE)),
            Transform::from_translation(position.extend(0.0)),
        ));
    }
    for obstacle in OBSTACLES {
        commands.spawn((
            ScreenEntity,
            obstacle,
            Sprite::from_color(Color::srgb(0.85, 0.55, 0.35), obstacle.size),
            Transform::from_translation(obstacle.center.extend(0.0)),
        ));
    }
}

fn clear_screen(mut commands: Commands, entities: Query<Entity, With<ScreenEntity>>) {
    for entity in &entities {
        commands.entity(entity).despawn();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fps_toggle_saves_the_preference() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("settings.json");
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .init_resource::<ButtonInput<KeyCode>>()
            .init_resource::<Settings>()
            .init_resource::<SaveStatus>()
            .insert_resource(SettingsFile(Some(path.clone())))
            .add_systems(Update, toggle_settings);
        app.update();
        assert!(!path.exists());
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyF);
        app.update();
        assert!(Settings::load(&path).unwrap().show_fps);
        assert_eq!(
            app.world().resource::<SaveStatus>().0,
            "Saved on this device."
        );
    }
}
