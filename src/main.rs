#![feature(macro_rules)]
#![feature(phase)]
#![feature(unsafe_destructor)]
//#[phase(plugin)] extern crate green;
extern crate arena;

use arena::TypedArena;

use core::cap::Actor;
use core::item;
use core::item::{Item, ItemCapSet, ItemData};
use core::mob;
use core::mob::{Mob, MobData};

use std::sync::Future;

mod core;

//green_start!(main)

fn main() {
    static NUM_ITEMS: uint = 10_000_000;
    static NUM_TASKS: uint = 8;
    static ITEM_DATA: ItemData = ItemData;
    let item_data = Item::make(ITEM_DATA);
    let mut item = Actor::make_actor(item_data);
    range(0u, NUM_TASKS).map( |i| {
        static ITEMS_PER_TASK: uint = NUM_ITEMS / NUM_TASKS;
        let arena = TypedArena::<Item>::with_capacity(ITEMS_PER_TASK);
        Future::spawn(proc() {
            let mut item = Actor::make_actor(Item::make(ITEM_DATA));
            //let ref mut item = arena.alloc(Item::make(ITEM_DATA));
            println!("Creating item {}", i);
            //let mut arena = arena;
            let new_item = Item::make(ITEM_DATA);
            //let mut item = TypedArena::alloc();
            //let mut item = Actor::make_actor(Item::make(ITEM_DATA));
            for _ in range(0u, ITEMS_PER_TASK) {
                //let new_item = Actor::make_actor(Item::make(ITEM_DATA));
                let new_item = Item::make(ITEM_DATA);
                //let value: &Item = arena.alloc(new_item);
                /*let new_item = Actor::make_actor(Item::make(ITEM_DATA));
                item.send_cmd(item::Give(new_item)).unwrap();*/
            }
            println!("Done populating item {}", i);
            item
        })
    }).collect::<Vec<Future<_>>>()
    .move_iter().map( |future| future.unwrap() )
    .map( |new_item| item::Give(new_item) )
    .map( |new_item| item.send_cmd(new_item) )
    .all( |result| result.is_ok() )
    /*.last();*//*
    let mob = Mob::make(MobData { title: "Zombie".into_maybe_owned(), desc: "Shuffling aimlessly.".into_maybe_owned()});
    let mut mobcap = Actor::make_actor(mob);
    mobcap.send_cmd(mob::Give(item)).unwrap()*/;
    println!("Done.");
}
