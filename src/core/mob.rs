use std::collections::enum_set::CLike;
use std::collections::EnumSet;

use core::cap::{Actor, CapType, CapRef, CapSet, Command};
use core::item::{ItemCapSet};
use std::fmt;

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

#[deriving(Show,Clone)]
pub struct MobData {
    pub title: String,
    pub desc: String,
}

#[deriving(Show)]
pub struct Mob {
    data: MobData,
    inv: Vec<ItemCapSet>,
}

pub type MobCapSet = CapSet<MobCap, Box<CapRef<MobCmd>>>;

impl Mob {
    pub fn make(data: MobData) -> Mob {
        Mob { data: data, inv: Vec::new() }
    }
}

impl Actor<MobCap, MobCmd> for Mob {
   fn handle(&mut self, cmd: MobCmd) {
        match cmd {
            Give(item) => {
                self.inv.push(item);
            }
        }
    }
}
