use std::collections::EnumSet;
use std::collections::enum_set::CLike;

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

pub struct CapSet<T, C> {
    cap_types: EnumSet<T>,
    tx: Sender<Option<C>>,
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
impl<T: CapType, C: Command<T> + Send> CapSet<T, C> {
    pub fn send_cmd_async(&self, cmd: C) -> Result<(), C> {
        if self.cap_types.contains_elem(cmd.cap_type()) {
            // Justification for unwrap: if it comes back it should be the same
            // value.
            self.tx.send_opt(Some(cmd)).map_err( |cmd| cmd.unwrap())
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
impl<T: CapType, C: Command<T> + Send> Drop for CapSet<T, C> {
   fn drop(&mut self) {
        self.tx.send_opt(None).unwrap_or(());
    }
}

pub trait Actor<T: CapType, C: Command<T> + Send>: Send {
    /// This is the main command handler called in an event loop.
    /// cmd: Command structure received as an argument.
    fn handle(&mut self, cmd: C, cap_set: &CapSet<T, C>);

    fn make_actor(actor: Box<Self>) -> CapSet<T, C> {
        let (tx, rx) = channel();
        let cap_types = CapType::all();
        let tx_clone = tx.clone();
        spawn(proc() {
            let mut actor = actor;
            let cap_set = CapSet { cap_types: cap_types, tx: tx };
            for cmd in rx.iter() {
                match cmd {
                    Some(cmd) => actor.handle(cmd, &cap_set),
                    None => break
                }
            }
        });
        CapSet { cap_types: cap_types, tx: tx_clone }
    }
}
