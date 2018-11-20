use crate::actor::{Gender, Human};
use std::collections::HashMap;

/// 衣装
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Costume {
    Dress,
    Tailcoart,
}

impl Costume {
    pub fn can_be_wore_by(&self, human: &Human) -> bool {
        match self {
            &Costume::Dress => human.gender == Gender::Woman,
            &Costume::Tailcoart => human.gender == Gender::Man,
        }
    }
}

#[derive(Default, Debug)]
pub struct DressRoom {
    dresses: HashMap<Human, Costume>,
}

impl DressRoom {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn store(&mut self, h: &Human) {
        let cos = Costume::Dress;
        self.dresses.insert(h.clone(), cos);
    }

    pub fn get_dress(&mut self, h: &mut Human) {
        if let Some(dress) = self.dresses.get(h) {
            if dress.can_be_wore_by(&h) {
                h.set_costume(dress.clone());
            }
        }
    }
}
