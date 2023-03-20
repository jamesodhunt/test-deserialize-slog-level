use serde::de::{Error, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;

// Since slog is an external crate and since rust only allows us to implement
// traits for local crates, we wrap the type we are about in a struct to allow
// us to do what we want ;)
#[derive(Debug)]
pub struct LevelContainer {
    pub level: slog::Level,
}

#[derive(Debug, thiserror::Error)]
pub enum LevelError {
    #[error("invalid slog level: {0}")]
    InvalidLevel(String),
}

impl TryFrom<&str> for LevelContainer {
    type Error = LevelError;

    fn try_from(s: &str) -> Result<Self, LevelError> {
        let level = match s.to_lowercase().as_str() {
            "critical" => slog::Level::Critical,
            "error" => slog::Level::Error,
            "warning" => slog::Level::Warning,
            "info" => slog::Level::Info,
            "debug" => slog::Level::Debug,
            "trace" => slog::Level::Trace,
            _ => return Err(LevelError::InvalidLevel(s.to_string())),
        };

        Ok(LevelContainer { level })
    }
}

impl TryFrom<String> for LevelContainer {
    type Error = LevelError;

    fn try_from(s: String) -> Result<Self, LevelError> {
        LevelContainer::try_from(s.as_str())
    }
}

impl<'de> Deserialize<'de> for LevelContainer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LevelContainerVisitor;

        impl<'de> Visitor<'de> for LevelContainerVisitor {
            type Value = LevelContainer;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("valid slog log level")
            }

            fn visit_str<E>(self, value: &str) -> Result<LevelContainer, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "critical" => Ok(LevelContainer {
                        level: slog::Level::Critical,
                    }),
                    "error" => Ok(LevelContainer {
                        level: slog::Level::Critical,
                    }),
                    "warning" => Ok(LevelContainer {
                        level: slog::Level::Warning,
                    }),
                    "info" => Ok(LevelContainer {
                        level: slog::Level::Info,
                    }),
                    "debug" => Ok(LevelContainer {
                        level: slog::Level::Debug,
                    }),
                    "trace" => Ok(LevelContainer {
                        level: slog::Level::Trace,
                    }),

                    _ => {
                        return Err(LevelError::InvalidLevel(value.to_string()))
                            .map_err(serde::de::Error::custom)
                    }
                }
            }
        }

        deserializer.deserialize_str(LevelContainerVisitor)
    }
}
