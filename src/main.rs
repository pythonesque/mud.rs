#![feature(phase)]
#[phase(plugin)] extern crate green;

use core::item::{Item, ItemData};
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
                item1.add(item2);
            });
        }
    });
    println!("{}", item1.deref());
}
