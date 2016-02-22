use item::Item;

pub trait Actor {
    fn act(&self) -> Item;
}

