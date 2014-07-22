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
    let item_data = Item::make(ItemData);
    let mut item = Actor::spawn_actor(box item_data);
    for i in range(0u, 10_000) {
        let new_item = if i % 2 == 0 {
            Actor::make_actor(box Item::make(ItemData))
        } else {
            Actor::spawn_actor(box Item::make(ItemData))
        };
        item.send_cmd_async(item::Give(new_item)).unwrap();
    }

    let mob = Mob::make(MobData { title: "Zombie".to_string(), desc: "Shuffling aimlessly.".to_string()});
    let mut mobcap = Actor::make_actor(box mob);
    mobcap.send_cmd_async(mob::Give(item)).unwrap();
    println!("{}", mobcap);
}
