#![feature(macro_rules)]
#![feature(phase)]
#![feature(unsafe_destructor)]
#[phase(plugin)] extern crate green;

use core::cap::Actor;
use core::item;
use core::item::{Item, ItemData};
use core::mob;
use core::mob::{Mob, MobData};

mod core;

green_start!(main)

fn main() {
    let item = Actor::make_actor(box Item::make(ItemData));
    for _ in range(0,10_000u) {
        let new_item = Actor::make_actor(box Item::make(ItemData));
        item.send_cmd_async(item::Give(new_item)).unwrap();
    }

    let mob = Mob::make(MobData { title: "Zombie".to_string(), desc: "Shuffling aimlessly.".to_string()});
    let mobcap = Actor::make_actor(box mob);
    println!("{}", mobcap);
    mobcap.send_cmd_async(mob::Give(item)).unwrap();
}
