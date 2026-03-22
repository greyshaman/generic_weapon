use crate::{
    assoc_type::{Countable, Shoot},
    firing_error::FiringError,
    firing_product::electromagnetic_pulse::ElectromagneticPulse,
    shell::laser_beam::LaserBeam,
};

/// Бластер
pub struct Blaster {
    charge: f64,
    cooldown: f64,
}

impl Blaster {
    pub fn new() -> Self {
        Self {
            charge: 100.0,
            cooldown: 0.0,
        }
    }
}

impl Shoot for Blaster {
    const MAX_AMMO: usize = 100;

    /// Тип выстрела - лазерный луч
    type Ammo = LaserBeam;

    /// сопутствующий продукт - электромагниный импульс
    type Byproduct = ElectromagneticPulse;

    type Error = FiringError;

    fn shoot(&mut self, target: &str) -> Result<(Self::Ammo, Self::Byproduct), Self::Error> {
        if self.charge < 10.0 {
            return Err(FiringError::NoAmmo);
        }

        if self.cooldown > 90.0 {
            return Err(FiringError::Overheat);
        }

        self.charge -= 20.0;
        self.cooldown += 10.0;
        let beam = LaserBeam::new(10.0, 532.0);
        let pulse = ElectromagneticPulse { intensity: 10.0 };
        println!("Бластер стреляет лазерным лучём в {}! Пиу!!!", target);
        Ok((beam, pulse))
    }

    fn reload(&mut self, ammo: Self::Ammo) -> Result<(), Self::Error> {
        if self.size() > Self::MAX_AMMO {
            return Err(FiringError::AmmoBusy);
        }
        self.charge += ammo.power();
        self.cooldown -= ammo.power() / 2.0;
        Ok(())
    }
}

impl Countable for Blaster {
    fn size(&self) -> usize {
        self.charge as usize
    }
}
