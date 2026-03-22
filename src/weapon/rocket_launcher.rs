use crate::{
    assoc_type::{Countable, Shoot},
    firing_error::FiringError,
    firing_product::plasma_trail::PlasmaTrail,
    shell::rocket::Rocket,
};

pub struct RocketLauncher {
    rockets: Vec<Rocket>,
}

impl RocketLauncher {
    pub fn new() -> Self {
        Self {
            rockets: Vec::new(),
        }
    }
}

impl Shoot for RocketLauncher {
    const MAX_AMMO: usize = 16;

    /// Тип выстрела - ракета
    type Ammo = Rocket;

    /// Сопутствующий продукт выстрела - плазменный след
    type Byproduct = PlasmaTrail;

    type Error = FiringError;

    fn shoot(&mut self, target: &str) -> Result<(Self::Ammo, Self::Byproduct), Self::Error> {
        self.rockets
            .pop()
            .and_then(|rocket| {
                let trail = PlasmaTrail {
                    temperature: 3000.0,
                };
                println!("Ракетная установка стреляет в {}! Вжух-бах!!!", target);
                Some((rocket, trail))
            })
            .ok_or(FiringError::NoAmmo)
    }

    fn reload(&mut self, ammo: Self::Ammo) -> Result<(), Self::Error> {
        if self.size() < Self::MAX_AMMO {
            self.rockets.push(ammo);
            Ok(())
        } else {
            Err(FiringError::AmmoBusy)
        }
    }
}

impl Countable for RocketLauncher {
    fn size(&self) -> usize {
        self.rockets.len()
    }
}
