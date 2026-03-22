use crate::firing_product::FiringProduct;

/// Электромагнитный импульс
#[derive(Default)]
pub struct ElectromagneticPulse {
    pub intensity: f64,
}

impl FiringProduct for ElectromagneticPulse {}
