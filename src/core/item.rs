use std::collections::enum_set::CLike;
use std::collections::EnumSet;
use std::uint;

use core::cap::{Actor, CapRef, CapSet, CapType, Command};

cap_type_set!(ItemCap,
    CapTransfer = 0,
)

#[deriving(Show)]
pub enum ItemCmd {
    Give(ItemCapSet),
}

impl Command<ItemCap> for ItemCmd {
    fn cap_type(&self) -> ItemCap {
        match *self {
            Give(_) => CapTransfer,
        }
    }
}

#[deriving(Show)]
pub struct ItemData /*{
}*/;

#[deriving(Show)]
pub struct Item {
    data: ItemData,
    contents: Vec<ItemCapSet>,
}

pub type ItemCapSet = CapSet<ItemCap, Box<CapRef<ItemCmd> + Send>>;

impl Item {
    pub fn make(data: ItemData) -> Item {
        let contents = Vec::new();
        Item { data: data, contents: contents }
    }
}

impl Actor<ItemCap, ItemCmd> for Item {
   fn handle(&mut self, cmd: ItemCmd) {
        match cmd {
            Give(item) => {
                self.contents.push(item);
            }
        }
    }
}
