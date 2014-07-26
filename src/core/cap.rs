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

   fn write_lock() -> EnumSet<Self>;
   fn read_lock() -> EnumSet<Self>;
}

/// Implement this to allow commands to be send across a capability.
/// In general, the command is allowed to be any type of data structure,
/// but it must advertise which capability it is intended to support, as
/// this can be checked against the capabilities provided to the actor at
/// send time. This allows the handler to simply assume that any command
/// it receives was sent through the correct channel.
pub trait Command<T>: Send {
    fn cap_type(&self) -> T;
}

enum CmdWrap<C> {
    Drop,
    Write(Box<Writer + Send>),
    Cmd(C),
    ReadLock(SyncSender<Box<CapLock + Send>>),
}

trait CapLock: fmt::Show + Send { // har har
    fn read_lock(&mut self, trans: Trans) -> Result<Trans, Trans>;
    //fn write_lock(&mut self, trans: Trans) -> Result<Trans, Trans>;
}

impl fmt::Show for Box<CapLock + Send> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt(f)
    }
}

/// We deliberately do not implement Clone for this.  Anyone who wants to do so
/// must wrap it in a Arc first.
pub trait CapRef<C>: fmt::Show + CapLock {
    fn send_cmd(&mut self, c: C) -> Result<(), C>;
}

struct CapMemRef<A> {
    inner: A,
}

impl<T: CapType, C: Command<T>, A: Actor<T, C>> CapRef<C> for CapMemRef<A> {
    fn send_cmd(&mut self, cmd: C) -> Result<(), C> {
        self.inner.handle(cmd);
        Ok(())
    }
}

impl<T: CapType, C: Command<T>, A: Actor<T, C>> CapLock for CapMemRef<A> {
    fn read_lock(&mut self, trans: Trans) -> Result<Trans, Trans> {
        //self.inner.handle(cmd);
        Ok(trans)
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

impl<C: Send> CapLock for CapTaskRef<C> {
    fn read_lock(&mut self, trans: Trans) -> Result<Trans, Trans> {
        let (stx, srx) = sync_channel(0);
        self.tx.send_opt(ReadLock(stx)).ok()
        .and_then(|_| srx.recv_opt().ok() )
        .map_or( Err(trans), |cap_lock| Ok(Trans { actor: Some(cap_lock) } ) )
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

/// Justification: this is only constructed in the cap module,
/// and we guarantee that the types are Send where it is
/// constructed.
/// (EnumSet<T> is Send because it's internally a uint).
#[unsafe_destructor]
impl<C: Send> Drop for CapTaskRef<C> {
    fn drop(&mut self) {
        self.tx.send_opt(Drop::<C>).unwrap_or(())
    }
}

pub struct CapSet<T, R> {
    cap_types: EnumSet<T>,
    cap_ref: R,
}

impl<T, C> fmt::Show for CapSet<T, Box<CapRef<C> + Send>> {
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
            // Not the most portable thing in the world...
            CapReadLock = (uint::BITS - 2) as int,
            CapWriteLock = (uint::BITS - 1) as int,
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
                cap_type_set.add(CapReadLock);
                cap_type_set.add(CapWriteLock);
                cap_type_set
            }

            #[inline(always)]
            fn read_lock() -> EnumSet<$cap_type> {
                let mut cap_type_set = EnumSet::empty();
                cap_type_set.add(CapReadLock);
                cap_type_set
            }

            #[inline(always)]
            fn write_lock() -> EnumSet<$cap_type> {
                let mut cap_type_set = EnumSet::empty();
                cap_type_set.add(CapWriteLock);
                cap_type_set
            }
         }
    );
    ($c:ident, $($v:ident = $i:expr),+, ) => (cap_type_set!($c, $($v = $i),+))
)

/// A Trans is a copy-on-write version of an existing Actor.  This is a bit more convoluted
/// than most actions because you actually need a local reference to the structure.
///
/// Generally speaking, in order to acquire a Trans for an actor of type A, one must:
/// 1. Create a new sync_channel with capacity 1 and data type Trans<A>.
/// 2. Acquire a CapTransRead capability to the actor.
/// 3. Create a BeginTrans command with the SyncSender half of your channel.
/// 4. Send the command along the CapTrans capability.
/// 5. Do a blocking read on the receiver.  The returned value should now be populated with a
///    Trans<A> that can be used to perform copy-on-write modification of the Actor.
///
/// Note that capabilities still apply; having the ability to begin (or end) a transaction does not
/// necessarily imply that you will actually be able to read anything from the actor, or that your
/// writes will commit successfully.
///
/// Once you have a Trans, you may modify it freely in any way you desire until you wish to
/// commit your changes, roll back, or disconnect (the latter two will cause your changes to be
/// discarded).  If the actor was modified after your Trans began but before it committed, your
/// transaction will be rolled back.  If this is problematic, you may write_lock() the Trans before
/// you begin your changes, which will cause other attempts to acquire the Trans to fail.
///
/// The two capabilities involved are:
///   CapReadLock - acquire a read lock on a transaction (allows direct reads from the actor).
///   CapWriteLock - acuqire a write lock on a transaction (allows direct writes to the actor).
#[deriving(Show)]
pub struct Trans {
    pub actor: Option<Box<CapLock + Send>>,
}

impl Drop for Trans {
    fn drop(&mut self) {
    }
}

impl<T: CapType, C: Command<T>> CapSet<T, Box<CapRef<C> + Send>> {
    pub fn send_cmd(&mut self, cmd: C) -> Result<(), C> {
        if self.cap_types.contains_elem(cmd.cap_type()) {
            self.cap_ref.send_cmd(cmd)
        } else {
            Err(cmd)
        }
    }
/*}

impl<T: CapType, C: Command<T>> CapLock for CapSet<T, Box<CapRef<C> + Send>> {*/
    pub fn read_lock(mut self, trans: Trans) -> Result<Trans, (Trans, CapSet<T, Box<CapRef<C> + Send>>)> {
        // The way we do this is a bit of a hack.  Because under our current model capabilities can
        // normally only be downgraded, not upgraded, and because we fail rather than block when
        // locking, we can use the status of the CapWriteLock bit in the capability set to
        // simultaneously determine both whether we are locked or not, and whether the user
        // actually has permission to perform a read lock anyway.  If some of these assumptions
        // change, we probably won't be able to do this anymore, though :(  This also relies on
        // read_lock always happening on any path that involves cloning the Result set, which is
        // why this method consumes self.
        let write_lock = CapType::write_lock();
        let cap_types = self.cap_types;
        if cap_types.contains(CapType::read_lock()) {
            // IMPORTANT: update trans *before* unsetting the write lock bit.  Otherwise task
            // failure could lead to incorrect state.
            match self.cap_ref.read_lock(trans).map( |trans| {
                self.cap_types = cap_types - write_lock;
                trans
            } ) {
                Ok(trans) => Ok(trans),
                Err(trans) => Err((trans, self))
            }
        } else {
            Err((trans, self))
        }
    }
}

pub trait Actor<T: CapType, C: Command<T>>: fmt::Show + Send {
    /// This is the main command handler called in an event loop.
    /// cmd: Command structure received as an argument.
    fn handle(&mut self, cmd: C);
}

pub fn make_actor<T: CapType, C: Command<T>, A: Actor<T, C> + fmt::Show + Send>(actor: A) -> CapSet<T, Box<CapRef<C> + Send>> {
    CapSet { cap_types: CapType::all(), cap_ref: box CapMemRef { inner: actor } as Box<CapRef<C> + Send> }
}

#[allow(dead_code)]
pub fn spawn_actor<T: CapType, C: Command<T>, A: Actor<T, C> + fmt::Show + Send>(actor: A) -> CapSet<T, Box<CapRef<C> + Send>> {
    let (tx, rx) = channel();
    let clone_tx = tx.clone();
    spawn(proc() {
        let mut actor = actor;
        let mut iter : Messages<CmdWrap<C>> = rx.iter();
        let mut refcount = 0u;
        for cmd in iter {
            match cmd {
                Cmd(cmd) => actor.handle(cmd),
                ReadLock(stx) => {
                    let ntx = tx.clone();
                    refcount += 1;
                    stx.send_opt(box CapTaskRef { tx: ntx } as Box<CapLock + Send>).ok();
                },
                Write(mut w) => {(write!(&mut w, "{}", actor)).ok(); },
                Drop => if refcount == 0 { break } else { refcount -= 1; }
            }
        }
    });
    CapSet { cap_types: CapType::all(), cap_ref: box CapTaskRef { tx: clone_tx } as Box<CapRef<C> + Send> }
}
