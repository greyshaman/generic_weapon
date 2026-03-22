/// Ошибки стрельбы
#[derive(Debug, thiserror::Error)]
pub enum FiringError {
    #[error("No ammo")]
    NoAmmo,

    #[error("Ammo loaded already")]
    AmmoBusy,

    #[error("Misfire")]
    Misfire,

    #[error("Target out of range")]
    TargetOutOfRange,

    #[error("Overheat")]
    Overheat,

    #[error("{0}")]
    IncompatibleAmmo(String),
}
