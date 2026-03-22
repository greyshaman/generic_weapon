use std::fmt::Debug;

pub trait Countable {
    fn size(&self) -> usize;
}

pub trait Shoot: Countable {
    /// FIXME: Может быть есть лучшее решение?
    const MIN_AMMO: usize = 0;
    const MAX_AMMO: usize = 1;

    type Ammo;
    type Byproduct;
    type Error: Debug;

    fn shoot(&mut self, target: &str) -> Result<(Self::Ammo, Self::Byproduct), Self::Error>;
    fn reload(&mut self, ammo: Self::Ammo) -> Result<(), Self::Error>;
}
