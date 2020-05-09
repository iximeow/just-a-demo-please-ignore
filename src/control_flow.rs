use core::fmt;
use yaxpeax_arch::{Address, AddressDiff, AddressDiffAmount};

#[derive(Clone, Debug, PartialEq)]
pub struct Effect<Addr: AddressDiffAmount + fmt::Debug> {
    pub(crate) stop_after: bool,
    pub dest: Option<Target<Addr>>
}

impl <Addr: Address + fmt::Debug> Effect<Addr> {
    pub fn is_stop(&self) -> bool {
        self.stop_after
    }

    pub fn stop() -> Effect<Addr> {
        Effect {
            stop_after: true,
            dest: None
        }
    }
    pub fn stop_and(dest: Target<Addr>) -> Effect<Addr> {
        Effect {
            stop_after: true,
            dest: Some(dest)
        }
    }
    pub fn cont() -> Effect<Addr> {
        Effect {
            stop_after: false,
            dest: None
        }
    }
    pub fn cont_and(dest: Target<Addr>) -> Effect<Addr> {
        Effect {
            stop_after: false,
            dest: Some(dest)
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Target<Addr: AddressDiffAmount + fmt::Debug> {
    Relative(AddressDiff<Addr>),
    Absolute(Addr),
    Multiple(usize), // TODO: ?? jump tables? tableid
    Indeterminate       // Unknowns? rets? idk
}
