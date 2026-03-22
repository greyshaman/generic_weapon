use crate::firing_product::FiringProduct;

/// Пороховые газы
#[derive(Default)]
pub struct GunpowderGas {
    pub pressure: f64,
}

impl FiringProduct for GunpowderGas {}
