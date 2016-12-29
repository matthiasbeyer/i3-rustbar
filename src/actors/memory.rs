use item::Item;
use actor::Actor;

pub enum Unit {
    KB,
    MB,
    GB,
}

pub struct MemActor {
    showin: Unit,
}

impl MemActor {

    pub fn new(showin: Unit) -> MemActor {
        MemActor { showin: showin }
    }

}

impl Actor for MemActor {

    fn act(&self) -> Item {
        use std::fs::OpenOptions;
        use std::io::Read;

        let mut buf = String::new();
        match OpenOptions::new()
            .read(true)
            .write(false)
            .create(false)
            .open("/proc/meminfo")
            .map(|mut file| file.read_to_string(&mut buf))
        {
            Ok(_) => { },
            Err(e) => return Item::new("memory".to_string()).with_text(format!("Error: {:?}", e.to_string())),
        }

        let map = buf.lines()
            .take(3)
            .fold(vec![], |mut acc, line| {
                let v = line.split(":").map(String::from).last().unwrap();
                let v = v.split_whitespace().next().unwrap();

                acc.push(to_unit(&self.showin, v));
                acc
            });

        let out = format!("T: {} / F: {} / A: {}", map[0], map[1], map[2]);

        Item::new("memory".to_string()).with_text(out)
    }

}

fn to_unit(unit: &Unit, s: &str) -> u64 {
    use std::str::FromStr;

    let i : u64 = FromStr::from_str(s).unwrap();
    i / {
        match *unit {
            Unit::KB => 1,
            Unit::MB => 1024,
            Unit::GB => 1024 * 1024,
        }
    }
}

