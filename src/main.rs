#![feature(macro_rules)]
#![feature(phase)]
#![feature(unsafe_destructor)]
use std::comm::Messages;
use std::fmt;
use std::io::{ChanReader, ChanWriter};

pub type CapType = ();

pub trait Command<T: fmt::Show>: fmt::Show + Send {
}

enum CmdWrap<C> {
    Write(Box<Writer + Send>),
}

pub trait CapRef<C>: fmt::Show {
}

struct CapTaskRef<C> {
    tx: Sender<CmdWrap<C>>,
}

impl<C: Send> CapRef<C> for CapTaskRef<C> {}

impl<C: Send> fmt::Show for CapTaskRef<C> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { Ok(()) }
}

pub struct CapSet<T, R> {
    cap_types: (),
    cap_ref: R,
}

impl<T, C: fmt::Show> fmt::Show for CapSet<T, Box<CapRef<C> + Send>> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { Ok(()) }
}

#[unsafe_destructor]
impl<T, C: Command<T> + Send> Drop for CapTaskRef<C> {
    fn drop(&mut self) {} 
}

pub trait Actor<T, C: Command<T>>: fmt::Show + Send {
    fn spawn_actor(actor: Self) -> CapSet<(), Box<CapRef<C> + Send>> {
        let (tx, rx) = channel();
        let tx_clone = tx.clone();
        spawn(proc() {
            let mut actor = actor;
            let mut iter : Messages<CmdWrap<C>> = rx.iter();
        });
        let cap_set : CapSet<(), Box<CapRef<C> + Send>> = CapSet { cap_types: (), cap_ref: box CapTaskRef { tx: tx_clone } as Box<CapRef<C> + Send> };
        cap_set
    }
}

impl Command<()> for () {}
impl Actor<(), ()> for () {}

fn main() {
    let itemcap = Actor::spawn_actor(());
}
