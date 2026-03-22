use crate::{
    assoc_type::{Countable, Shoot},
    firing_error::FiringError,
    firing_product::wind_rustle::WindRustle,
    shell::arrow::Arrow,
};

/// Арбалет
pub struct Crossbow {
    arrow: Option<Arrow>,
    draw_weight: f64,
}

impl Crossbow {
    pub fn new(draw_weight: f64) -> Self {
        Self {
            arrow: None,
            draw_weight,
        }
    }
}

impl Shoot for Crossbow {
    /// тип снаряда - стрела
    type Ammo = Arrow;

    /// Сопутствующий выстрелу продукт - шелест ветра
    type Byproduct = WindRustle;

    type Error = FiringError;

    fn shoot(&mut self, target: &str) -> Result<(Self::Ammo, Self::Byproduct), Self::Error> {
        if let Some(arrow) = self.arrow.clone() {
            let rustle = WindRustle { volume: 10.0 };
            println!("Арбалет стреляет в {}! Фью!!!", target);
            self.arrow = None;
            Ok((arrow, rustle))
        } else {
            Err(self::FiringError::NoAmmo)
        }
    }

    fn reload(&mut self, ammo: Self::Ammo) -> Result<(), Self::Error> {
        if self.arrow.is_none() {
            self.arrow = Some(ammo);
            Ok(())
        } else {
            Err(FiringError::AmmoBusy)
        }
    }
}

impl Countable for Crossbow {
    fn size(&self) -> usize {
        match self.arrow {
            Some(_) => 1,
            None => 0,
        }
    }
}
