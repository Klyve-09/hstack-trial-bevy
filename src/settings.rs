use bevy::prelude::*;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::{
    fs, io,
    path::{Path, PathBuf},
};

#[derive(Resource, Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(deny_unknown_fields)]
pub struct Settings {
    pub version: u32,
    pub show_fps: bool,
    pub show_movement_hints: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            version: 1,
            show_fps: false,
            show_movement_hints: true,
        }
    }
}

impl Settings {
    pub fn load(path: &Path) -> io::Result<Self> {
        let text = match fs::read_to_string(path) {
            Ok(text) => text,
            Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(Self::default()),
            Err(error) => return Err(error),
        };
        let settings: Self = serde_json::from_str(&text)?;
        if settings.version != 1 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Unsupported settings version",
            ));
        }
        Ok(settings)
    }

    pub fn save(&self, path: &Path) -> io::Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let temporary = path.with_extension("json.tmp");
        fs::write(&temporary, serde_json::to_vec_pretty(self)?)?;
        fs::rename(temporary, path)
    }
}

#[derive(Resource)]
pub struct SettingsFile(pub Option<PathBuf>);

impl SettingsFile {
    pub fn discover() -> Self {
        Self(
            ProjectDirs::from("", "Klyve", "Drift")
                .map(|dirs| dirs.data_dir().join("settings.json")),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn settings_round_trip_and_replace() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("nested/settings.json");
        assert_eq!(Settings::load(&path).unwrap(), Settings::default());
        let mut settings = Settings {
            show_fps: true,
            ..default()
        };
        settings.save(&path).unwrap();
        assert_eq!(Settings::load(&path).unwrap(), settings);
        settings.show_fps = false;
        settings.save(&path).unwrap();
        assert_eq!(Settings::load(&path).unwrap(), settings);
        assert!(!path.with_extension("json.tmp").exists());
    }

    #[test]
    fn malformed_and_future_settings_are_rejected() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("settings.json");
        for contents in ["broken", r#"{"version":2,"show_fps":true}"#] {
            fs::write(&path, contents).unwrap();
            assert!(Settings::load(&path).is_err());
        }
    }
}
