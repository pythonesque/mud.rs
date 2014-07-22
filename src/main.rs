#![feature(macro_rules)]
#![feature(phase)]
#![feature(unsafe_destructor)]
#[phase(plugin)] extern crate green;

use core::cap::Actor;
use core::item;
use core::item::{Item, ItemCapSet, ItemData};
use core::mob;
use core::mob::{Mob, MobCapSet, MobData};

mod core;

green_start!(main)

fn main() {
    let item_data = ItemData;
    let item = Item::make(item_data);
    let itemcap = Actor::spawn_actor(item);
    for i in range(0u, 2) {
        let new_item = if i % 2 == 0 {
            Actor::spawn_actor(Item::make(ItemData))
        } else {
            Actor::make_actor(Item::make(ItemData))
        };
        //itemcap.send_cmd(item::Give(new_item)).unwrap();
    }

    let mob_data = MobData { title: "Zombie".to_string(), desc: "Shuffling aimlessly.".to_string()};
    let mob = Mob::make(mob_data);
    let mut mobcap = Actor::make_actor(mob);
    mobcap.send_cmd(mob::Give(itemcap)).unwrap();
    println!("{}", mobcap);
}
