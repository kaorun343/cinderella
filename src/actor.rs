use crate::costume::Costume;
use crate::shoes::Shoes;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Gender {
    Woman,
    Man,
}

/// 登場人物
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Human {
    pub name: String,
    age: u32,
    pub gender: Gender,
    pub cos: Option<Costume>,
    pub shoes: Option<Shoes>,
}

impl Human {
    pub fn new(name: &str, age: u32, gender: Gender) -> Self {
        Human {
            name: name.to_owned(),
            age,
            gender,
            cos: None,
            shoes: None,
        }
    }

    pub fn say(&self, s: &str) {
        println!("{}: {}", self.name, s);
    }

    pub fn set_costume(&mut self, c: Costume) {
        self.cos = Some(c);
    }

    pub fn set_shoes(&mut self, s: Shoes) {
        self.shoes = Some(s);
    }
}
