pub trait Bite {
    fn bite(self: &mut Self);
}

#[derive(Debug)]
pub struct Grapes {
    pub grapes_left: i32,
}

impl Bite for Grapes {
    fn bite(self: &mut Self) {
        self.grapes_left -= 1;
    }
}

pub fn bunny_nibbles<T: Bite>(item: &mut T) {
    item.bite();
    item.bite();
    item.bite();
    item.bite();
}
