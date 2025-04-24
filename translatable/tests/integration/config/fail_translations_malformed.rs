// the macro isn't filled because the expected
// failure is on configuration.

use translatable::{translation, Language};

fn fail_translations_malformed() {
    translation!(Language::ES, vec![]);
}
