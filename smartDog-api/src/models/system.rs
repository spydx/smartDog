use crate::models::bowls::AlertLevel;
use serde::{Deserialize, Serialize};

// systemstatus
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Health {
    pub db: bool,
    pub status: AlertLevel,
    pub time: String,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum HealthAlert {
    Ok,
    Warning,
    Failed,
}
