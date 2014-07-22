use std::collections::EnumSet;
use std::collections::enum_set::CLike;
use std::comm::Messages;
use std::fmt;
use std::io::{ChanReader, ChanWriter};

pub trait CapType: CLike {
   /// This should return a "full" set of all capability types
   /// that the actor cares about.  The reason for this is so
   /// we have something to return to the creator, and also
   /// so that it can be used as the only source of new
   /// capabilities for this actor.  Otherwise, the actor
   /// would have to create new Capabiities using the same
   /// Sender and there would have to be "new capability creation"
   /// functions that take a Sender outside the module, which
   /// breaks encapsulation for no good reason.
   fn all() -> EnumSet<Self>;
}

/// Implement this to allow commands to be send across a capability.
/// In general, the command is allowed to be any type of data structure,
/// but it must advertise which capability it is intended to support, as
/// this can be checked against the capabilities provided to the actor at
/// send time. This allows the handler to simply assume that any command
/// it receives was sent through the correct channel.
pub trait Command<T> {
    fn cap_type(&self) -> T;
}

enum CmdWrap<C> {
    Drop,
    Write(Box<Writer + Send>),
    Cmd(C)
}

enum CapRef<C, A> {
    Task(Sender<CmdWrap<C>>),
    Ref(Box<A>),
}

pub struct CapSet<T, C, A> {
    cap_types: EnumSet<T>,
    cap_ref: CapRef<C, A>,
}

impl<A: fmt::Show, T, C: Send> fmt::Show for CapSet<T, C, A> {
    /// WARNING: could cause recursive task failure!  Only call this if you
    /// directly own the capability you are calling it on!
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.cap_ref {
            Task(ref tx) => {
                let (ftx, frx) = channel();
                let fw: ChanWriter = ChanWriter::new(ftx);
                let mut fr = ChanReader::new(frx);
                tx.send_opt(Write(box fw)).ok()
                .and_then( |_| fr.read_to_end().ok() )
                .map_or( Err(fmt::WriteError), |buf| f.write(buf.as_slice()))
            },
            Ref(ref actor) => {
                actor.fmt(f)
            },
        }
    }
}

/// sugar for easily creating a capability type set
macro_rules! cap_type_set(
    ($cap_type:ident,
        $($var:ident = $i:expr),*
    ) => (
        #[deriving(Clone, FromPrimitive)]
        enum $cap_type {
            $($var = $i),*
        }

       impl CLike for $cap_type {
            fn to_uint(&self) -> uint {
                *self as uint
            }

            fn from_uint(v: uint) -> $cap_type {
                FromPrimitive::from_uint(v).unwrap()
            }
        }

        impl CapType for $cap_type {
            #[inline(always)]
            fn all() -> EnumSet<$cap_type> {
                let mut cap_type_set = EnumSet::empty();
                $( cap_type_set.add($var); )*
                cap_type_set
            }
        }
    );
    ($c:ident, $($v:ident = $i:expr),+, ) => (cap_type_set!($c, $($v = $i),+))
)

/// We deliberately do not implement Clone for this.  Anyone who wants to do so
/// must wrap it in a Arc first.
impl<T: CapType, C: Command<T> + Send, A: Actor<T, C>> CapSet<T, C, A> {
    pub fn send_cmd_async(&mut self, cmd: C) -> Result<(), C> {
        if self.cap_types.contains_elem(cmd.cap_type()) {
            match self.cap_ref {
                Task(ref tx) => {
                    // Justification for the fail!: if it comes back it should be the same
                    // value.
                    tx.send_opt(Cmd(cmd)).map_err( |cmd|
                        match cmd {
                            Cmd(c) => c,
                            _ => fail!("Can't happen")
                        })
                },
                Ref(ref mut actor) => {
                    actor.handle(cmd);
                    Ok(())
                }
            }
        } else {
            Err(cmd)
        }
    }
}

/// Justification: this is only constructed in the cap module,
/// and we guarantee that the types are Send where it is
/// constructed.
/// (EnumSet<T> is Send because it's internally a uint).
#[unsafe_destructor]
impl<T: CapType, C: Command<T> + Send, A> Drop for CapSet<T, C, A> {
   fn drop(&mut self) {
        match self.cap_ref {
            Task(ref tx) => tx.send_opt(Drop).unwrap_or(()),
            _ => ()
        }
    }
}

pub trait Actor<T: CapType, C: Command<T> + Send>: fmt::Show + Send {
    /// This is the main command handler called in an event loop.
    /// cmd: Command structure received as an argument.
    fn handle(&mut self, cmd: C/*, cap_set: &CapSet<T, C, Self>*/);

    fn make_actor(actor: Box<Self>) -> CapSet<T, C, Self> {
        let cap_types : EnumSet<T> = CapType::all();
        let cap_set : CapSet<T, C, Self> = CapSet { cap_types: cap_types, cap_ref: Ref(actor) };
        cap_set
    }

    fn spawn_actor(actor: Box<Self>) -> CapSet<T, C, Self> {
        let (tx, rx) = channel();
        let cap_types = CapType::all();
        let tx_clone = tx.clone();
        spawn(proc() {
            let mut actor = actor;
            /*let cap_set : CapSet<T, C, Self> = CapSet { cap_types: cap_types, cap_ref: Task(tx) };*/
            let mut iter : Messages<CmdWrap<C>> = rx.iter();
            for cmd in iter {
                match cmd {
                    Cmd(cmd) => actor.handle(cmd),
                    Write(mut w) => {(write!(&mut w, "{}", actor)).ok(); },
                    Drop => break
                }
            }
        });
        CapSet { cap_types: cap_types, cap_ref: Task(tx_clone) }
    }
}
