#![feature(macro_rules)]
#![feature(phase)]
#![feature(unsafe_destructor)]
#[phase(plugin)] extern crate green;

use core::cap::Actor;
use core::item::{Item, ItemData};
use core::mob;
use core::mob::Mob;
use std::sync::Arc;

mod core;

green_start!(main)

fn main() {
    let item1 = Arc::new(Item::make(ItemData));
    let item1_clone = item1.clone();
    spawn(proc() {
        let item1 = item1_clone;
        for _ in range(0,10000u) {
            let item1_clone = item1.clone();
            spawn(proc() {
                let item1 = item1_clone;
                let item2 = Item::make(ItemData);
                item1.clone_add(item2);
            });
        }
    });
    println!("{}", item1.deref());

    let mob = Mob::make();
    let mobcap = Actor::make_actor(box mob);
    mobcap.send_cmd_async(mob::Transfer).unwrap();
}
