use confy;

use crate::models::AppConfig;

pub fn load_config() -> Result<AppConfig, confy::ConfyError> {
    confy::load("OVES", "config.properties")
}
