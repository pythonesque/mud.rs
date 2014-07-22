use std::collections::enum_set::CLike;
use std::collections::EnumSet;
use std::fmt;

use core::cap::{Actor, CapType, Command};
use core::item::{ItemCapSet};

cap_type_set!(MobCap,
    CapTransfer = 0,
)

pub enum MobCmd {
    Give(ItemCapSet),
}

impl fmt::Show for MobCmd {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Give(_) => write!(f, "Give")
        }
    }
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
