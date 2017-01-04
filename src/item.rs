use std::collections::BTreeMap;

use serde::Serializer;
use serde::Serialize;

pub type Color = String;
pub type Instance = String;
pub type Markup = String;

pub struct Item {
    color: Option<Color>,
    full_text: Option<String>,
    instance: Option<Instance>,
    markup: Option<Markup>,
    name: String,
}

impl Item {
    pub fn new(name: String) -> Item {
        Item {
            color: None,
            full_text: None,
            instance: None,
            markup: None,
            name: name,
        }
    }

    pub fn with_color(mut self, c: Color) -> Item {
        self.color = Some(c);
        self
    }

    pub fn with_text(mut self, t: String) -> Item {
        self.full_text = Some(t);
        self
    }

    pub fn with_instance(mut self, i: Instance) -> Item {
        self.instance = Some(i);
        self
    }

    pub fn with_markup(mut self, m: Markup) -> Item {
        self.markup = Some(m);
        self
    }
}

impl Serialize for Item {

    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        let mut map = BTreeMap::new();

        if self.color.is_some() {
            map.insert("color", self.color.clone().unwrap());
        }

        if self.full_text.is_some() {
            map.insert("full_text", self.full_text.clone().unwrap());
        }

        if self.instance.is_some() {
            map.insert("instance", self.instance.clone().unwrap());
        }

        if self.markup.is_some() {
            map.insert("markup", self.markup.clone().unwrap());
        }

        map.insert("name", self.name.clone());
        map.serialize(serializer)
    }
}

