use std::collections::enum_set::CLike;
use std::collections::EnumSet;
use std::fmt;

use core::cap::{Actor, CapSet, CapType, Command};

cap_type_set!(ItemCap,
    CapTransfer = 0,
)

pub enum ItemCmd {
    Give(ItemCapSet),
}

pub type ItemCapSet = CapSet<ItemCap, ItemCmd>;

impl fmt::Show for ItemCmd {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Give(_) => write!(f, "Give")
        }
    }
}

impl Command<ItemCap> for ItemCmd {
    fn cap_type(&self) -> ItemCap {
        match *self {
            Give(_) => CapTransfer,
        }
    }
}

#[deriving(Clone,Show)]
pub struct ItemData /*{
}*/;

#[deriving(Show)]
pub struct Item {
    data: ItemData,
    contents: Vec<ItemCapSet>,
}

impl Item {
    pub fn make(data: ItemData) -> Item {
        let contents = Vec::new();
        Item { data: data, contents: contents }
    }
}

impl Actor<ItemCap, ItemCmd> for Item {
   fn handle(&mut self, cmd: ItemCmd, _cap_set: &ItemCapSet) {
        match cmd {
            Give(item) => {
                self.contents.push(item);
            }
        }
    }
}
