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
    static NUM_ITEMS: uint = 10_000_000;
    static NUM_TASKS: uint = 8;
    static ITEM_DATA: ItemData = ItemData;
    let item_data = Item::make(ITEM_DATA);
    let mut item = Actor::make_actor(item_data);
    range(0u, NUM_TASKS).map( |i| {
        static ITEMS_PER_TASK: uint = NUM_ITEMS / NUM_TASKS;
        Future::spawn(proc() {
            let mut item = Actor::make_actor(Item::make(ITEM_DATA));
            println!("Creating item {}", i);
            for _ in range(0u, ITEMS_PER_TASK) {
                let new_item = Actor::make_actor(Item::make(ItemData));
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
    println!("Done.");
}
