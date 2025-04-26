#[allow(unused_imports)] // trybuild
use translatable::translation;

#[cfg(test)]
#[test]
pub fn pass_dynamic_expr() {
    let author = "Juan";
    let target = "Pepito";

    let translation = translation!("es", static auditory::actions::delete_user, author, target,);

    assert_eq!(translation, "Juan ha borrado al usuario Pepito.");
}

#[allow(dead_code)]
fn main() {} // trybuild
