use chrono::*;

use item::Item;
use actor::Actor;

pub struct TimeActor {
    seconds: bool,
    weekday: bool,
}

impl TimeActor {

    pub fn new() -> TimeActor {
        TimeActor {
            seconds: false,
            weekday: false,
        }
    }

    pub fn show_seconds(mut self, b: bool) -> TimeActor {
        self.seconds = b;
        self
    }

    pub fn show_weekday(mut self, b: bool) -> TimeActor {
        self.weekday = b;
        self
    }

}

impl Actor for TimeActor {

    fn act(&self) -> Item {
        let fmt = {
            let s = String::from("%Y-%m-%d");
            let s = {
                if self.weekday {
                    format!("{} %a", s)
                } else { s }
            };
            let s = format!("{} %H:%M", s);
            let s = {
                if self.seconds {
                    format!("{}:%S", s)
                } else { s }
            };
            s
        };
        let ymd = UTC::now().format(&fmt[..]).to_string();
        Item::new(String::from("time"))
            .with_text(ymd)
    }
}
