#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Coord {
    pub row: usize,
    pub col: usize,
}

impl Coord {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum State {
    Numbered(usize),
    White,
    Black,
    Unknown,
}

mod region_id {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct RegionID(usize);

    impl RegionID {
        pub unsafe fn from_raw(raw: usize) -> Self {
            Self(raw)
        }

        pub fn to_raw(self) -> usize {
            self.0
        }
    }
}

pub use region_id::RegionID;
