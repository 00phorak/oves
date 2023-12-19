use std::path::Path;

use confy;

use crate::models::AppConfig;

pub fn load_config() -> Result<AppConfig, confy::ConfyError> {
    confy::load_path(Path::new("config.toml"))
}
