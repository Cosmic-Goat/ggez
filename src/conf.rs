//! The `conf` module contains functions for loading and saving game
//! configurations.
//!
//! A [`Conf`](struct.Conf.html) struct is used to create a config file
//! which specifies hardware setup stuff, mostly video display settings.
//!
//! By default a ggez game will search its resource paths for a `/conf.toml`
//! file and load values from it when the [`Context`](../struct.Context.html) is created.  This file
//! must be complete (ie you cannot just fill in some fields and have the
//! rest be default) and provides a nice way to specify settings that
//! can be tweaked such as window resolution, multisampling options, etc.
//! If no file is found, it will create a `Conf` object from the settings
//! passed to the [`ContextBuilder`](../struct.ContextBuilder.html).

use std::convert::TryFrom;
use std::io;

use crate::error::{GameError, GameResult};

/// Possible fullscreen modes.
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum FullscreenType {
    /// Windowed mode.
    Windowed,
    /// True fullscreen, which used to be preferred 'cause it can have
    /// small performance benefits over windowed fullscreen.
    ///
    /// Also it allows us to set different resolutions.
    True,
    /// Windowed fullscreen, generally preferred over real fullscreen
    /// these days 'cause it plays nicer with multiple monitors.
    Desktop,
}

/// A builder structure containing window settings
/// that can be set at runtime and changed with [`graphics::set_mode()`](../graphics/fn.set_mode.html).
///
/// Defaults:
///
/// ```rust
/// # use ggez::conf::*;
/// # fn main() { assert_eq!(
/// WindowMode {
///     width: 800.0,
///     height: 600.0,
///     maximized: false,
///     fullscreen_type: FullscreenType::Windowed,
///     borderless: false,
///     min_width: 0.0,
///     max_width: 0.0,
///     min_height: 0.0,
///     max_height: 0.0,
///     resizable: false,
///     visible: true,
///     resize_on_scale_factor_change: false,
/// }
/// # , WindowMode::default());}
/// ```
#[derive(Debug, Copy, Clone, SmartDefault, Serialize, Deserialize, PartialEq)]
pub struct WindowMode {
    /// Window width in physical pixels
    #[default = 800.0]
    pub width: f32,
    /// Window height in physical pixels
    #[default = 600.0]
    pub height: f32,
    /// Whether or not to maximize the window
    #[default = false]
    pub maximized: bool,
    /// Fullscreen type
    #[default(FullscreenType::Windowed)]
    pub fullscreen_type: FullscreenType,
    /// Whether or not to show window decorations
    #[default = false]
    pub borderless: bool,
    /// Whether or not the window should be transparent
    #[default = false]
    pub transparent: bool,
    /// Minimum width for resizable windows; 0 means no limit
    #[default = 0.0]
    pub min_width: f32,
    /// Minimum height for resizable windows; 0 means no limit
    #[default = 0.0]
    pub min_height: f32,
    /// Maximum width for resizable windows; 0 means no limit
    #[default = 0.0]
    pub max_width: f32,
    /// Maximum height for resizable windows; 0 means no limit
    #[default = 0.0]
    pub max_height: f32,
    /// Whether or not the window is resizable
    #[default = false]
    pub resizable: bool,
    /// Whether this window should displayed (true) or hidden (false)
    #[default = true]
    pub visible: bool,
    /// Whether this window should change its size in physical pixels
    /// when its hidpi factor changes, i.e. when [`WindowEvent::ScaleFactorChanged`](https://docs.rs/winit/0.25.0/winit/event/enum.WindowEvent.html#variant.ScaleFactorChanged)
    /// is fired.
    ///
    /// You usually want this to be false, since the window suddenly changing size may break your game.
    /// Setting this to true may be desirable if you plan for it and want your window to behave like
    /// windows of other programs when being dragged from one screen to another, for example.
    ///
    /// For more context on this take a look at [this conversation](https://github.com/ggez/ggez/pull/949#issuecomment-854731226).
    #[default = false]
    pub resize_on_scale_factor_change: bool,
}

impl WindowMode {
    /// Set default window size, or screen resolution in true fullscreen mode.
    pub fn dimensions(mut self, width: f32, height: f32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    /// Set whether the window should be maximized.
    pub fn maximized(mut self, maximized: bool) -> Self {
        self.maximized = maximized;
        self
    }

    /// Set the fullscreen type.
    pub fn fullscreen_type(mut self, fullscreen_type: FullscreenType) -> Self {
        self.fullscreen_type = fullscreen_type;
        self
    }

    /// Set whether a window should be borderless in windowed mode.
    pub fn borderless(mut self, borderless: bool) -> Self {
        self.borderless = borderless;
        self
    }


    /// Set whether a window should be transparent.
    pub fn transparent(mut self, transparent: bool) -> Self {
        self.transparent = transparent;
        self
    }

    /// Set minimum window dimensions for windowed mode.
    pub fn min_dimensions(mut self, width: f32, height: f32) -> Self {
        self.min_width = width;
        self.min_height = height;
        self
    }

    /// Set maximum window dimensions for windowed mode.
    pub fn max_dimensions(mut self, width: f32, height: f32) -> Self {
        self.max_width = width;
        self.max_height = height;
        self
    }

    /// Set resizable.
    pub fn resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        self
    }

    /// Set visibility
    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    /// Set whether to resize when the hidpi factor changes
    pub fn resize_on_scale_factor_change(mut self, resize_on_scale_factor_change: bool) -> Self {
        self.resize_on_scale_factor_change = resize_on_scale_factor_change;
        self
    }
}

/// A builder structure containing window settings
/// that must be set at init time and cannot be changed afterwards.
///
/// Defaults:
///
/// ```rust
/// # use ggez::conf::*;
/// # fn main() { assert_eq!(
/// WindowSetup {
///     title: "An easy, good game".to_owned(),
///     samples: NumSamples::One,
///     vsync: true,
///     icon: "".to_owned(),
///     srgb: true,
/// }
/// # , WindowSetup::default()); }
/// ```
#[derive(Debug, Clone, SmartDefault, Serialize, Deserialize, PartialEq)]
pub struct WindowSetup {
    /// The window title.
    #[default(String::from("An easy, good game"))]
    pub title: String,
    /// Number of samples to use for multisample anti-aliasing.
    #[default(NumSamples::One)]
    pub samples: NumSamples,
    /// Whether or not to enable vsync.
    #[default = true]
    pub vsync: bool,
    /// A file path to the window's icon.
    /// It takes a path rooted in the `resources` directory (see the [`filesystem`](../filesystem/index.html)
    /// module for details), and an empty string results in a blank/default icon.
    #[default(String::new())]
    pub icon: String,
    /// Whether or not to enable sRGB (gamma corrected color)
    /// handling on the display.
    #[default = true]
    pub srgb: bool,
}

impl WindowSetup {
    /// Set window title.
    pub fn title(mut self, title: &str) -> Self {
        self.title = title.to_owned();
        self
    }

    /// Set number of samples to use for multisample anti-aliasing.
    pub fn samples(mut self, samples: NumSamples) -> Self {
        self.samples = samples;
        self
    }

    /// Set whether vsync is enabled.
    pub fn vsync(mut self, vsync: bool) -> Self {
        self.vsync = vsync;
        self
    }

    /// Set the window's icon.
    pub fn icon(mut self, icon: &str) -> Self {
        self.icon = icon.to_owned();
        self
    }

    /// Set sRGB color mode.
    pub fn srgb(mut self, active: bool) -> Self {
        self.srgb = active;
        self
    }
}

/// Possible backends.
/// Currently, only OpenGL and OpenGL ES Core specs are supported,
/// but this lets you specify which to use as well as the version numbers.
///
/// Defaults:
///
/// ```rust
/// # use ggez::conf::*;
/// # fn main() { assert_eq!(
/// Backend::OpenGL {
///     major: 3,
///     minor: 2,
/// }
/// # , Backend::default()); }
/// ```
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, SmartDefault)]
#[serde(tag = "type")]
pub enum Backend {
    /// Defaults to OpenGL 3.2, which is supported by basically
    /// every machine since 2009 or so (apart from the ones that don't).
    #[default]
    #[allow(clippy::upper_case_acronyms)]
    OpenGL {
        /// OpenGL major version
        #[default = 3]
        major: u8,
        /// OpenGL minor version
        #[default = 2]
        minor: u8,
    },
    /// OpenGL ES, defaults to 3.0.  Used for phones and other mobile
    /// devices.  Using something older
    /// than 3.0 starts to running into sticky limitations, particularly
    /// with instanced drawing (used for `SpriteBatch`), but might be
    /// possible.
    #[allow(clippy::upper_case_acronyms)]
    OpenGLES {
        /// OpenGL ES major version
        #[default = 3]
        major: u8,
        /// OpenGL ES minor version
        #[default = 0]
        minor: u8,
    },
}

impl Backend {
    /// Set requested OpenGL/OpenGL ES version.
    pub fn version(self, new_major: u8, new_minor: u8) -> Self {
        match self {
            Backend::OpenGL { .. } => Backend::OpenGL {
                major: new_major,
                minor: new_minor,
            },
            Backend::OpenGLES { .. } => Backend::OpenGLES {
                major: new_major,
                minor: new_minor,
            },
        }
    }

    /// Use OpenGL
    pub fn gl(self) -> Self {
        match self {
            Backend::OpenGLES { major, minor } => Backend::OpenGL { major, minor },
            gl => gl,
        }
    }

    /// Use OpenGL ES
    pub fn gles(self) -> Self {
        match self {
            Backend::OpenGL { major, minor } => Backend::OpenGLES { major, minor },
            es => es,
        }
    }
}

/// The possible number of samples for multisample anti-aliasing.
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum NumSamples {
    /// One sample
    One = 1,
    /// Two samples
    Two = 2,
    /// Four samples
    Four = 4,
    /// Eight samples
    Eight = 8,
    /// Sixteen samples
    Sixteen = 16,
}

impl TryFrom<u8> for NumSamples {
    type Error = GameError;
    fn try_from(i: u8) -> Result<Self, Self::Error> {
        match i {
            1 => Ok(NumSamples::One),
            2 => Ok(NumSamples::Two),
            4 => Ok(NumSamples::Four),
            8 => Ok(NumSamples::Eight),
            16 => Ok(NumSamples::Sixteen),
            _ => Err(GameError::ConfigError(String::from(
                "Invalid number of samples",
            ))),
        }
    }
}

impl From<NumSamples> for u8 {
    fn from(ns: NumSamples) -> u8 {
        ns as u8
    }
}

/// Defines which submodules to enable in ggez.
/// If one tries to use a submodule that is not enabled,
/// it will panic.  Currently, not all subsystems can be
/// disabled.
///
/// Defaults:
///
/// ```rust
/// # use ggez::conf::*;
/// # fn main() { assert_eq!(
/// ModuleConf {
///     gamepad: true,
///     audio: true,
/// }
/// # , ModuleConf::default()); }
/// ```
#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, SmartDefault)]
pub struct ModuleConf {
    /// The gamepad input module.
    #[default = true]
    pub gamepad: bool,

    /// The audio module.
    #[default = true]
    pub audio: bool,
}

impl ModuleConf {
    /// Sets whether or not to enable the gamepad input module.
    pub fn gamepad(mut self, gamepad: bool) -> Self {
        self.gamepad = gamepad;
        self
    }

    /// Sets whether or not to enable the audio module.
    pub fn audio(mut self, audio: bool) -> Self {
        self.audio = audio;
        self
    }
}

/// A structure containing configuration data
/// for the game engine.
///
/// Defaults:
///
/// ```rust
/// # use ggez::conf::*;
/// # fn main() { assert_eq!(
/// Conf {
///     window_mode: WindowMode::default(),
///     window_setup: WindowSetup::default(),
///     backend: Backend::default(),
///     modules: ModuleConf::default(),
/// }
/// # , Conf::default()); }
/// ```
#[derive(Serialize, Deserialize, Debug, PartialEq, SmartDefault, Clone)]
pub struct Conf {
    /// Window setting information that can be set at runtime
    pub window_mode: WindowMode,
    /// Window setting information that must be set at init-time
    pub window_setup: WindowSetup,
    /// Graphics backend configuration
    pub backend: Backend,
    /// Which modules to enable.
    pub modules: ModuleConf,
}

impl Conf {
    /// Same as `Conf::default()`
    pub fn new() -> Self {
        Self::default()
    }

    /// Load a TOML file from the given `Read` and attempts to parse
    /// a `Conf` from it.
    pub fn from_toml_file<R: io::Read>(file: &mut R) -> GameResult<Conf> {
        let mut s = String::new();
        let _ = file.read_to_string(&mut s)?;
        let decoded = toml::from_str(&s)?;
        Ok(decoded)
    }

    /// Saves the `Conf` to the given `Write` object,
    /// formatted as TOML.
    pub fn to_toml_file<W: io::Write>(&self, file: &mut W) -> GameResult {
        let s = toml::to_vec(self)?;
        file.write_all(&s)?;
        Ok(())
    }

    /// Sets the window mode
    pub fn window_mode(mut self, window_mode: WindowMode) -> Self {
        self.window_mode = window_mode;
        self
    }

    /// Sets the backend
    pub fn backend(mut self, backend: Backend) -> Self {
        self.backend = backend;
        self
    }

    /// Sets the backend
    pub fn modules(mut self, modules: ModuleConf) -> Self {
        self.modules = modules;
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::conf;

    /// Tries to encode and decode a `Conf` object
    /// and makes sure it gets the same result it had.
    #[test]
    fn headless_encode_round_trip() {
        let c1 = conf::Conf::new();
        let mut writer = Vec::new();
        let _c = c1.to_toml_file(&mut writer).unwrap();
        let mut reader = writer.as_slice();
        let c2 = conf::Conf::from_toml_file(&mut reader).unwrap();
        assert_eq!(c1, c2);
    }
}
