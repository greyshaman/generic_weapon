use std::f64;

use crate::{
    assoc_type::{Countable, Shoot},
    firing_error::FiringError,
    firing_product::gunpowder_gas::GunpowderGas,
    shell::bullet::Bullet,
};

/// Пистолет
pub struct Pistol {
    ammo: Vec<Bullet>,
    caliber: f64,
}

impl Pistol {
    pub fn new(caliber: f64) -> Self {
        Self {
            ammo: Vec::new(),
            caliber,
        }
    }
}

impl Shoot for Pistol {
    const MAX_AMMO: usize = 6;

    /// Типа снаряда - пуля
    type Ammo = Bullet;

    /// Сопутствующий выстрелу продукт - пороховые газы
    type Byproduct = GunpowderGas;

    type Error = FiringError;

    fn shoot(&mut self, target: &str) -> Result<(Self::Ammo, Self::Byproduct), Self::Error> {
        // Извлекаем патрон из магазина
        if let Some(bullet) = self.ammo.pop() {
            let gas = GunpowderGas { pressure: 2500.0 };
            // производим выстрел
            println!("Пистолет стреляет в {}! Бах!", target);
            Ok((bullet, gas))
        } else {
            Err(FiringError::NoAmmo)
        }
    }

    fn reload(&mut self, ammo: Self::Ammo) -> Result<(), Self::Error> {
        if self.size() == Self::MAX_AMMO {
            return Err(FiringError::AmmoBusy);
        }

        // вставляем патрон в магазин с проверкой калибра
        if f64::abs(ammo.caliber() - self.caliber) < std::f64::EPSILON {
            self.ammo.push(ammo);
            Ok(())
        } else {
            Err(FiringError::IncompatibleAmmo("Illegal caliber".to_string()))
        }
    }
}

impl Countable for Pistol {
    fn size(&self) -> usize {
        self.ammo.len()
    }
}
