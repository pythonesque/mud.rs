use std::collections::enum_set::CLike;
use std::collections::EnumSet;
use std::str::MaybeOwned;

use core::cap::{Actor, CapType, CapRef, CapSet, Command};
use core::item::{ItemCapSet};

cap_type_set!(MobCap,
    CapTransfer = 0,
)

#[deriving(Show)]
pub enum MobCmd {
    Give(ItemCapSet),
}

impl Command<MobCap> for MobCmd {
    fn cap_type(&self) -> MobCap {
        match *self {
            Give(_) => CapTransfer,
        }
    }
}

#[deriving(Show)]
pub struct MobData<'a> {
    pub title: MaybeOwned<'a>,
    pub desc: MaybeOwned<'a>,
}

#[deriving(Show)]
pub struct Mob<'a> {
    data: MobData<'a>,
    inv: Vec<ItemCapSet>,
}

pub type MobCapSet = CapSet<MobCap, Box<CapRef<MobCmd> + Send>>;

impl<'a> Mob<'a> {
    pub fn make(data: MobData) -> Mob {
        Mob { data: data, inv: Vec::new() }
    }
}

impl Actor<MobCap, MobCmd> for Mob<'static> {
   fn handle(&mut self, cmd: MobCmd) {
        match cmd {
            Give(item) => {
                self.inv.push(item);
            }
        }
    }
}
