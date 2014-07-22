use std::collections::EnumSet;
use std::collections::enum_set::CLike;
use std::comm::Messages;
use std::fmt;
use std::io::{ChanReader, ChanWriter};

//#[deriving(Show)]
pub trait CapType: CLike + fmt::Show {
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
//#[deriving(Show)]
pub trait Command<T: fmt::Show>: fmt::Show + Send {
    fn cap_type(&self) -> T;
}

enum CmdWrap<C> {
    Drop,
    Write(Box<Writer + Send>),
    Cmd(C)
}

/// We deliberately do not implement Clone for this.  Anyone who wants to do so
/// must wrap it in a Arc first.
//#[deriving(Show)]
pub trait CapRef<C>: fmt::Show {
    fn send_cmd(&mut self, c: C) -> Result<(), C>;
}

/*impl<C: fmt::Show> fmt::Show for Box<CapRef<C>> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt(f)
    }
}*/

/*impl<C, R: CapRef<C> + fmt::Show> CapRef<C> for Box<R> {
    fn send_cmd(&mut self, c: C) -> Result<(), C> {
        self.send_cmd(c)
    }
}*/

/*impl<C> fmt::Show for Box<CapRef<C>> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt(f)
    }
}*/

/*impl<T: CapType, C: Command<T> + Send> fmt::Show for Box<CapRef<C> + Send> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt(f)
    }
}*/

struct CapMemRef<A> {
    inner: A,
}

impl<T: CapType, C: Command<T>, A: Actor<T, C>> CapRef<C> for CapMemRef<A> {
    fn send_cmd(&mut self, cmd: C) -> Result<(), C> {
        self.inner.handle(cmd);
        Ok(())
    }
}

impl<A: fmt::Show> fmt::Show for CapMemRef<A> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.inner.fmt(f)
    }
}

struct CapTaskRef<C> {
    tx: Sender<CmdWrap<C>>,
}

impl<C: Send> CapRef<C> for CapTaskRef<C> {
    fn send_cmd(&mut self, cmd: C) -> Result<(), C> {
        // Justification for the fail!: if it comes back it should be the same
        // value.
        self.tx.send_opt(Cmd(cmd)).map_err( |cmd|
            match cmd {
                Cmd(c) => c,
                _ => fail!("Can't happen")
            })
    }
}

impl<C: Send> fmt::Show for CapTaskRef<C> {
    /// WARNING: could cause recursive task failure!  Only call this if you
    /// directly own the capability you are calling it on!
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (ftx, frx) = channel();
        let fw: ChanWriter = ChanWriter::new(ftx);
        let mut fr = ChanReader::new(frx);
        self.tx.send_opt(Write(box fw)).ok()
        .and_then( |_| fr.read_to_end().ok() )
        .map_or( Err(fmt::WriteError), |buf| f.write(buf.as_slice()))
    }
}

//#[deriving(Show)]
pub struct CapSet<T, R> {
    cap_types: EnumSet<T>,
    cap_ref: R,
}

impl<T, C: fmt::Show> fmt::Show for CapSet<T, Box<CapRef<C> + Send>> {
    /// WARNING: could cause recursive task failure!  Only call this if you
    /// directly own the capability you are calling it on!
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.cap_ref.fmt(f)
    }
}


/// sugar for easily creating a capability type set
macro_rules! cap_type_set(
    ($cap_type:ident,
        $($var:ident = $i:expr),*
    ) => (
        #[deriving(Clone, FromPrimitive, Show)]
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

/*impl<T: CapType, C: Command<T>/*, R: CapRef<C>*/> CapRef<C> for CapSet<T, /*R*/Box<CapRef<C>>> {
    fn send_cmd(&mut self, cmd: C) -> Result<(), C> {
        if self.cap_types.contains_elem(cmd.cap_type()) {
            self.cap_ref.send_cmd(cmd)
        } else {
            Err(cmd)
        }
    }
}*/

/*pub impl<T: CapType, C: Command<T>> CapRef<C> for CapSet<T, Box<CapRef<C>>> {
    pub fn send_cmd(&mut self, cmd: C) -> Result<(), C> {
        if self.cap_types.contains_elem(cmd.cap_type()) {
            self.cap_ref.send_cmd(cmd)
        } else {
            Err(cmd)
        }
    }
}*/

impl<T: CapType, C: Command<T>/*, Self: CapRef<C>*/> CapSet<T, Box<CapRef<C> + Send>> {
    /*fn fmt(&self, f: &mut fmt::FormatWriter) -> fmt::Result {
    }*/
    pub fn send_cmd(&mut self, cmd: C) -> Result<(), C> {
        if self.cap_types.contains_elem(cmd.cap_type()) {
            self.cap_ref.send_cmd(cmd)
        } else {
            Err(cmd)
        }
    }
}

/*impl<T: CapType + fmt::Show, C: Command<T> + Send/*, R: CapRef<C>*/> CapRef<C> for CapSet<T, Box</*R*/CapRef<C>>> {
    fn send_cmd(&mut self, cmd: C) -> Result<(), C> {
        if self.cap_types.contains_elem(cmd.cap_type()) {
            self.cap_ref.send_cmd(cmd)
        } else {
            Err(cmd)
        }
    }
}*/

/// Justification: this is only constructed in the cap module,
/// and we guarantee that the types are Send where it is
/// constructed.
/// (EnumSet<T> is Send because it's internally a uint).
#[unsafe_destructor]
impl<T: CapType, C: Command<T> + Send> Drop for CapTaskRef<C> {
    fn drop(&mut self) {
        self.tx.send_opt(Drop::<C>).unwrap_or(())
    } 
}

pub trait Actor<T: CapType, C: Command<T>>: fmt::Show + Send {
    /// This is the main command handler called in an event loop.
    /// cmd: Command structure received as an argument.
    fn handle(&mut self, cmd: C/*, cap_set: &CapSet<T, C, Self>*/);

    fn make_actor<T: CapType/*, C: Command<T> + Send, Self: Actor<T, C>*/>(actor: Self) -> CapSet<T, Box<CapRef<C> + Send>> {
        let cap_set : CapSet<T, Box<CapRef<C> + Send>> =
            CapSet { cap_types: CapType::all(), cap_ref: box CapMemRef { inner: actor } as Box<CapRef<C> + Send> };
        cap_set
    }

    fn spawn_actor/*<T: CapType, C: Command<T> + Send, Self: Actor<T, C>>*/(actor: Self) -> CapSet<T, Box<CapRef<C> + Send>> {
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
        let cap_set : CapSet<T, Box<CapRef<C> + Send>> = CapSet { cap_types: cap_types, cap_ref: box CapTaskRef { tx: tx_clone } as Box<CapRef<C> + Send> };
        cap_set
    }
}
