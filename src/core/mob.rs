use std::collections::enum_set::CLike;
use std::collections::EnumSet;
use std::fmt;

use core::cap::{Actor, CapSet, CapType, Command};
use core::item::{ItemCapSet};

cap_type_set!(MobCap,
    CapTransfer = 0,
)

pub enum MobCmd{
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

pub struct Mob {
    data: MobData,
    inv: Vec<ItemCapSet>,
}

impl fmt::Show for Mob {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Mob data={} inv={}", self.data, self.inv)
    }
}

impl Mob {
    pub fn make(data: MobData) -> Mob {
        Mob { data: data, inv: Vec::new() }
    }
}

impl Actor<MobCap, MobCmd> for Mob {
   fn handle(&mut self, cmd: MobCmd, _cap_set: &CapSet<MobCap, MobCmd>) {
        match cmd {
            Give(item) => {
                //println!("{}: Got {}!", *self, item);
                self.inv.push(item);
            }
        }
    }
}
