pub struct Sequence;

pub fn in_sequence(mut setups: impl FnMut()) -> Sequence {
    setups();
    Sequence
}

impl Sequence {
    pub fn verify(self) {}
}