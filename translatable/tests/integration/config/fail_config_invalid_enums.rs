// the macro isn't filled because the expected
// failure is on configuration.

use translatable::{translation, Language};

fn fail_config_invalid_enums() {
    translation!(Language::ES, vec![]);
}
