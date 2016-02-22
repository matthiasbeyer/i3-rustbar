extern crate serde;
extern crate serde_json;
#[macro_use] extern crate version;

use std::collections::BTreeMap;
use std::io::stdout;
use std::io::stderr;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;

mod actor;
mod actors;
mod item;

use actor::Actor;
use actors::text::TextActor;
use item::Item;

use serde_json::ser::PrettyFormatter;
use serde_json::ser::Serializer;
use serde_json::ser::to_string;
use serde::Serialize;
use version::*;

fn main() {
    let mut serializer = Serializer::new(stdout());
    {
        let mut map = BTreeMap::new();
        map.insert("version", format!("{}", version!()));
        map.serialize(&mut serializer);
    }
    println!("["); // Yes this is an ugly hack... but hey, am I perfect?

    let actors = vec![
    //     DHCPActor::new(),
    //     BatteryActor::new(),
    //     InternetActor::new(),
    //     AudioActor::new(),
    //     TimeActor::new(),
            TextActor::new(String::from("i3-rustbar is awesome!")),
    ];

    loop {
        let ary : Vec<Item> = actors
            .iter()
            .map(|a| a.act())
            .collect();
        println!("{}", to_string(&ary).unwrap());
        sleep(Duration::new(1, 0));
    }
}
