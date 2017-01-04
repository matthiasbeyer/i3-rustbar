#![allow(dead_code)] // we must allow dead code because we are recompile-to-reconfigure
#![deny(
    non_camel_case_types,
    non_snake_case,
    path_statements,
    trivial_numeric_casts,
    unstable_features,
    unused_allocation,
    unused_import_braces,
    unused_imports,
    unused_must_use,
    unused_mut,
    unused_qualifications,
    while_true,
)]

extern crate chrono;
extern crate serde;
extern crate serde_json;
#[macro_use] extern crate version;

use std::collections::BTreeMap;
use std::io::stdout;
use std::thread::sleep;
use std::time::Duration;

mod actor;
mod actors;
mod item;

use actor::Actor;
use actors::text::TextActor;
use actors::time::TimeActor;
use actors::memory::MemActor;
use actors::memory::Unit as MemUnit;
use item::Item;

use serde_json::ser::Serializer;
use serde_json::ser::to_string;
use serde::Serialize;

fn main() {
    let mut serializer = Serializer::new(stdout());
    {
        let mut map = BTreeMap::new();
        map.insert("version", 1);
        let _ = map.serialize(&mut serializer);
    }
    println!("\n["); // Yes this is an ugly hack... but hey, am I perfect?

    let actors : Vec<Box<Actor>> = vec![
    //     DHCPActor::new(),
    //     BatteryActor::new(),
    //     InternetActor::new(),
    //     AudioActor::new(),
            Box::new(TimeActor::new()),
            Box::new(TextActor::new(String::from("i3-rustbar is awesome!"))),
            Box::new(MemActor::new(MemUnit::MB)),
    ];

    loop {
        let ary : Vec<Item> = actors
            .iter()
            .map(|a| a.act())
            .collect();
        println!("{},", to_string(&ary).unwrap());
        sleep(Duration::new(1, 0));
    }
}
