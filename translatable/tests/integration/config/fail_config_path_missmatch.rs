// the macro isn't filled because the expected
// failure is on configuration.

use translatable::{translation, Language};

fn fail_config_path_missmatch() {
    translation!(Language::ES, vec![]);
}
