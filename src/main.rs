#![feature(macro_rules)]
#![feature(phase)]
#![feature(unsafe_destructor)]
#[phase(plugin)] extern crate green;

use core::cap::Actor;
use core::item;
use core::item::{Item, ItemData};
use core::mob;
use core::mob::{Mob, MobData};
use std::sync::Future;

mod core;

green_start!(main)

fn main() {
    let item_data = Item::make(ItemData);
    let mut item = Actor::make_actor(item_data);
    range(0u, 16).map( |i| {
        Future::spawn(proc() {
            println!("Creating item {}", i);
            let mut item = Actor::make_actor(Item::make(ItemData));
            for _ in range(0u, 612_500) {
                let new_item = if /*i % 2 == 0*/false {
                    Actor::spawn_actor(Item::make(ItemData))
                } else {
                    Actor::make_actor(Item::make(ItemData))
                };
                item.send_cmd(item::Give(new_item)).unwrap();
            }
            println!("Done populating item {}", i);
            item
        })
    }).collect::<Vec<Future<_>>>()
    .move_iter().map( |future| future.unwrap() )
    .map( |new_item| item::Give(new_item) )
    .map( |new_item| item.send_cmd(new_item) )
    .all( |result| result.is_ok() );
    let mob = Mob::make(MobData { title: "Zombie".into_maybe_owned(), desc: "Shuffling aimlessly.".into_maybe_owned()});
    let mut mobcap = Actor::make_actor(mob);
    mobcap.send_cmd(mob::Give(item)).unwrap();
}
