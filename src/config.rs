use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

#[derive(Clone)]
pub struct Config {
    pub options: Option<HashMap<String, String>>,
    pub native_file: Option<PathBuf>,
    pub cross_file: Option<PathBuf>,
}

impl Config {
    pub fn new() -> Config {
        Config {
            options: None,
            native_file: None,
            cross_file: None,
        }
    }

    pub fn options<'a>(self, options: HashMap<&'a str, &'a str>) -> Self {
        let options = options
            .into_iter()
            .map(|(key, value)| (String::from(key), String::from(value)))
            .collect::<HashMap<String, String>>();

        let mut config = self;

        config.options = Some(options);
        config
    }

    pub fn native_file(self, native_file: impl AsRef<Path>) -> Self {
        let native_file: PathBuf = native_file.as_ref().into();
        let mut config = self;
        config.native_file = Some(native_file);
        config
    }

    pub fn cross_file(self, cross_file: impl AsRef<Path>) -> Self {
        let cross_file: PathBuf = cross_file.as_ref().into();
        let mut config = self;
        config.cross_file = Some(cross_file);
        config
    }
}
