use std::collections::enum_set::CLike;
use std::collections::EnumSet;

use core::cap::{Actor, CapSet, CapType, Command};

#[deriving(Clone)]
enum MobCap {
    CapTransfer,
}

impl CLike for MobCap {
    fn to_uint(&self) -> uint {
        *self as uint
    }

    fn from_uint(v: uint) -> MobCap {
        match v {
            0 => CapTransfer,
            _ => {
                // should never happen if only used with EnumSet, but check to be safe
                fail!(format!("{} is not associated with any MobCap type", v))
            }
        }
    }
}

impl CapType for MobCap {
    fn all() -> EnumSet<MobCap> {
        let mut cap_set = EnumSet::empty();
        cap_set.add(CapTransfer);
        cap_set
    }
}

#[deriving(Show)]
pub enum MobCmd{
    Transfer,
}

impl Command<MobCap> for MobCmd{
    fn cap_type(&self) -> MobCap {
        match *self {
            Transfer => CapTransfer,
        }
    }
}

pub struct Mob;

impl Mob {
    pub fn make() -> Mob {
        Mob
    }
}

impl Actor<MobCap, MobCmd> for Mob {
   fn handle(&mut self, cmd: MobCmd, _cap_set: &CapSet<MobCap, MobCmd>) {
        match cmd {
            Transfer => {
                println!("Transferred!");
            }
        }
    }
}
