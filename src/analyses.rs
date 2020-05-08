pub trait DFG<V> {
    type Location;

    fn read_loc(&self, loc: Self::Location) -> V;
    fn write_loc(&mut self, loc: Self::Location, value: V);
}

pub trait Value: Sized {
    fn unknown() -> Self;

    fn from_const(_c: u64) -> Self {
        Self::unknown()
    }

    fn from_set(xs: &[Self]) -> Self;

    fn to_const(&self) -> Option<u64>;

    fn as_bool(&self) -> Option<bool> {
        None
    }

    fn add(&self, _other: &Self) -> Self {
        Self::unknown()
    }

    fn sub(&self, _other: &Self) -> Self {
        Self::unknown()
    }

    fn eq(&self, _other: &Self) -> Self {
        Self::unknown()
    }

    fn not(&self) -> Self {
        Self::unknown()
    }
}
