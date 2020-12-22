use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BowlContent {
    pub name: String,
    pub alert: AlertLevel,
    pub waterlevel: u16,
    pub time: String
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct BowlDTO {
    pub waterlevel: i16
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Health {
    pub db: bool,
    pub status: AlertLevel,
    pub bowl: BowlContent,
    pub time: String
}
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum AlertLevel {
    LowWater,
    PlentyWater,
    FullWater
}
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum HealthAlert {
    Ok,
    Warning,
    Failed
}
