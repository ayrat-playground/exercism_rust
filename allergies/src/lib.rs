use self::Allergen::*;
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

pub struct Allergies {
    score: u32,
    value: Vec<Allergen>,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        let mut score = score;
        let mut allergies = vec!();

        if score % 2 != 0 {
            allergies.push(Eggs);
            score = score - 1;
        }

        let mut div = (score as f32).log2() as u32;
        let mut scores = allergen_scores();

        println!("{}", div);
        println!("{}", score);
        while div != 0 {
            let cur = scores.iter().find(|el| el.0 <= div).unwrap().clone();
            allergies.push(cur.1);

            div = div - cur.0;
            let position = scores.iter().position(|x| *x == cur).unwrap();
            scores.remove(position);
        }

        Self { score: score, value: allergies }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.value.iter().any(|el| el == allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.value.to_owned()
    }
}

fn allergen_scores() -> Vec<(u32, Allergen)> {
    vec!(
        (7, Cats),
        (6, Pollen),
        (5, Chocolate),
        (4, Tomatoes),
        (3, Strawberries),
        (2, Shellfish),
        (1, Peanuts),
    )
}
