use item::Item;
use actor::Actor;

pub struct TextActor {
    text: String,
}

impl TextActor {

    pub fn new(s: String) -> TextActor {
        TextActor { text: s }
    }

}

impl Actor for TextActor {

    fn act(&self) -> Item {
        Item::new("custom-text".to_owned()).with_text(self.text.clone())
    }

}
