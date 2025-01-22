use self::Allergen::*;

pub struct Allergies {
    allergy_list: Vec<Allergen>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Allergen {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128,
}

impl Allergen {
    pub fn iterator() -> impl Iterator<Item = Allergen> {
        [
            Eggs,
            Peanuts,
            Shellfish,
            Strawberries,
            Tomatoes,
            Chocolate,
            Pollen,
            Cats,
        ]
        .iter()
        .copied()
    }
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies {
            allergy_list: Allergen::iterator()
                .filter(|&x| (x as u32 & score) != 0)
                .collect(),
        }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergy_list.iter().any(|x| x == allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.allergy_list.clone()
    }
}
