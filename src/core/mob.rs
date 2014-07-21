use std::collections::enum_set::CLike;
use std::collections::EnumSet;

use core::cap::{Actor, CapSet, CapType, Command};

cap_type_set!(MobCap,
    CapTransfer = 0,
)

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
