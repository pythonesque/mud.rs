use std::collections::enum_set::CLike;
use std::collections::EnumSet;
use std::str::MaybeOwned;

use core::cap::{Actor, CapType, CapRef, CapSet, Command};
use core::item::ItemCapSet;
use core::mob::MobCapSet;

cap_type_set!(RoomCap,
    CapTransfer = 0,
)

#[deriving(Show)]
pub enum RoomCmd {
    GiveMob(MobCapSet),
    GiveItem(ItemCapSet),
}

impl Command<RoomCap> for RoomCmd {
    fn cap_type(&self) -> RoomCap {
        match *self {
            GiveMob(_) => CapTransfer,
            GiveItem(_) => CapTransfer,
        }
    }
}

#[deriving(Show)]
pub struct RoomData<'a> {
    pub title: MaybeOwned<'a>,
    pub desc: MaybeOwned<'a>,
}

#[deriving(Show)]
pub struct Room<'a> {
    data: RoomData<'a>,
    mobs: Vec<MobCapSet>,
    items: Vec<ItemCapSet>,
}

pub type RoomCapSet = CapSet<RoomCap, Box<CapRef<RoomCmd> + Send>>;

impl<'a> Room<'a> {
    pub fn make(data: RoomData) -> Room {
        Room { data: data, mobs: Vec::new(), items: Vec::new() }
    }
}

impl Actor<RoomCap, RoomCmd> for Room<'static> {
   fn handle(&mut self, cmd: RoomCmd) {
        match cmd {
            GiveMob(mob) => {
                self.mobs.push(mob);
            }
            GiveItem(item) => {
                self.items.push(item);
            }
        }
    }
}
