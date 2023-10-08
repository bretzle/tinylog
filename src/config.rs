use log::Level;
use log::LevelFilter;

use std::borrow::Cow;
use termcolor::Color;

#[derive(Debug, Clone, Copy)]
pub enum LevelPadding {
    Left,
    Right,
    Off,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub(crate) level: LevelFilter,
    pub(crate) level_padding: LevelPadding,
    pub(crate) location: LevelFilter,
    pub(crate) filter_ignore: Cow<'static, [Cow<'static, str>]>,
    pub(crate) level_color: [Option<Color>; 6],
    pub(crate) write_log_enable_colors: bool,
}


#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct ConfigBuilder(Config);

impl ConfigBuilder {
    pub fn set_max_level(&mut self, level: LevelFilter) -> &mut ConfigBuilder {
        self.0.level = level;
        self
    }

    pub fn set_location_level(&mut self, location: LevelFilter) -> &mut ConfigBuilder {
        self.0.location = location;
        self
    }

    pub fn set_level_padding(&mut self, padding: LevelPadding) -> &mut ConfigBuilder {
        self.0.level_padding = padding;
        self
    }

    pub fn set_level_color(&mut self, level: Level, color: Option<Color>) -> &mut ConfigBuilder {
        self.0.level_color[level as usize] = color;
        self
    }

    pub fn add_filter_ignore_str(&mut self, filter_ignore: &'static str) -> &mut ConfigBuilder {
        let mut list = Vec::from(&*self.0.filter_ignore);
        list.push(Cow::Borrowed(filter_ignore));
        self.0.filter_ignore = Cow::Owned(list);
        self
    }

    pub fn add_filter_ignore(&mut self, filter_ignore: String) -> &mut ConfigBuilder {
        let mut list = Vec::from(&*self.0.filter_ignore);
        list.push(Cow::Owned(filter_ignore));
        self.0.filter_ignore = Cow::Owned(list);
        self
    }

    pub fn clear_filter_ignore(&mut self) -> &mut ConfigBuilder {
        self.0.filter_ignore = Cow::Borrowed(&[]);
        self
    }

    pub fn build(&mut self) -> Config {
        self.0.clone()
    }
}

impl Default for Config {
    fn default() -> Config {
        Config {
            level: LevelFilter::Error,
            level_padding: LevelPadding::Off,
            location: LevelFilter::Trace,
            filter_ignore: Cow::Borrowed(&[]),
            write_log_enable_colors: false,

            level_color: [
                None,                // Default foreground
                Some(Color::Red),    // Error
                Some(Color::Yellow), // Warn
                Some(Color::Blue),   // Info
                Some(Color::Cyan),   // Debug
                Some(Color::White),  // Trace
            ],
        }
    }
}
