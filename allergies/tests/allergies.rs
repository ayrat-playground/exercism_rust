extern crate allergies;

use allergies::*;

fn compare_allergy_vectors(expected: &Vec<Allergen>, actual: &Vec<Allergen>) {
    for element in expected {
        if !actual.contains(element) {
            panic!("Allergen missing\n  {:?} should be in {:?}",
                   element,
                   actual);
        }
    }

    if actual.len() != expected.len() {
        panic!("Allergy vectors are of different lengths\n  expected {:?}\n  got {:?}",
               expected,
               actual);
    }
}

#[test]
fn is_not_allergic_to_anything() {
    let allergies = Allergies::new(0);
    assert_eq!(false, allergies.is_allergic_to(&Allergen::Peanuts));
    assert_eq!(false, allergies.is_allergic_to(&Allergen::Cats));
    assert_eq!(false, allergies.is_allergic_to(&Allergen::Strawberries));
}

#[test]
fn is_allergic_to_eggs() {
    assert_eq!(true, Allergies::new(1).is_allergic_to(&Allergen::Eggs));
}

#[test]
fn is_allergic_to_egg_shellfish_and_strawberries() {
    let allergies = Allergies::new(5);
    assert_eq!(true, allergies.is_allergic_to(&Allergen::Eggs));
    assert_eq!(true, allergies.is_allergic_to(&Allergen::Shellfish));
    assert_eq!(false, allergies.is_allergic_to(&Allergen::Strawberries));
}

#[test]
fn no_allergies_at_all() {
    let expected: Vec<Allergen> = Vec::new();
    let allergies = Allergies::new(0).allergies();

    compare_allergy_vectors(&expected, &allergies);
}

#[test]
fn allergic_to_just_eggs() {
    let expected = vec![Allergen::Eggs];
    let allergies = Allergies::new(1).allergies();

    compare_allergy_vectors(&expected, &allergies);
}

#[test]
fn allergic_to_just_peanuts() {
    let expected = vec![Allergen::Peanuts];
    let allergies = Allergies::new(2).allergies();

    compare_allergy_vectors(&expected, &allergies);
}

#[test]
fn allergic_to_just_strawberries() {
    let expected = vec![Allergen::Strawberries];
    let allergies = Allergies::new(8).allergies();

    compare_allergy_vectors(&expected, &allergies);
}

#[test]
fn allergic_to_eggs_and_peanuts() {
    let expected = vec![Allergen::Eggs, Allergen::Peanuts];
    let allergies = Allergies::new(3).allergies();

    compare_allergy_vectors(&expected, &allergies);
}

#[test]
fn allergic_to_eggs_and_shellfish() {
    let expected = vec![Allergen::Eggs, Allergen::Shellfish];
    let allergies = Allergies::new(5).allergies();

    compare_allergy_vectors(&expected, &allergies);
}

#[test]
fn allergic_to_many_things() {
    let expected = vec![Allergen::Strawberries,
                        Allergen::Tomatoes,
                        Allergen::Chocolate,
                        Allergen::Pollen,
                        Allergen::Cats];
    let allergies = Allergies::new(248).allergies();

    compare_allergy_vectors(&expected, &allergies);
}

#[test]
#[ignore]
fn allergic_to_everything() {
    let expected = vec![Allergen::Eggs,
                        Allergen::Peanuts,
                        Allergen::Shellfish,
                        Allergen::Strawberries,
                        Allergen::Tomatoes,
                        Allergen::Chocolate,
                        Allergen::Pollen,
                        Allergen::Cats];
    let allergies = Allergies::new(255).allergies();

    compare_allergy_vectors(&expected, &allergies);
}

#[test]
#[ignore]
fn scores_over_255_do_not_trigger_false_positives() {
    let expected = vec![Allergen::Eggs,
                        Allergen::Shellfish,
                        Allergen::Strawberries,
                        Allergen::Tomatoes,
                        Allergen::Chocolate,
                        Allergen::Pollen,
                        Allergen::Cats];
    let allergies = Allergies::new(509).allergies();

    compare_allergy_vectors(&expected, &allergies);
}
