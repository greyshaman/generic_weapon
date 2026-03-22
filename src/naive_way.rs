use crate::{
    firing_error::FiringError,
    firing_product::{gunpowder_gas::GunpowderGas, wind_rustle::WindRustle},
    shell::{arrow::Arrow, bullet::Bullet},
};

trait Shoot<Shell, FiringProduct> {
    fn shoot(&self, target: &str) -> Result<(Shell, FiringProduct), FiringError>;
}

struct Pistol;
impl Shoot<Bullet, GunpowderGas> for Pistol {
    fn shoot(&self, target: &str) -> Result<(Bullet, GunpowderGas), FiringError> {
        Ok((Bullet::new(9.0), GunpowderGas::default()))
    }
}

impl Shoot<Arrow, GunpowderGas> for Pistol {
    fn shoot(&self, target: &str) -> Result<(Arrow, GunpowderGas), FiringError> {
        panic!("Incorrect ammo");
    }
}

// impl Shoot<GunpowderGas, Bullet> for Pistol {
//     fn shoot(&self, target: &str) -> Result<(GunpowderGas, Bullet), FiringError> {
//         Ok((GunpowderGas::default(), Bullet::new(9.0)))
//     }
// }

struct Crossbow;
impl Shoot<Arrow, WindRustle> for Crossbow {
    fn shoot(&self, target: &str) -> Result<(Arrow, WindRustle), FiringError> {
        Ok((Arrow::new(0.2, "Iron"), WindRustle::default()))
    }
}
