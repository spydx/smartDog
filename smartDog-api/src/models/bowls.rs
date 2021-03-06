use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewBowl {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Bowls {
    pub id: String,
    pub name: String,
    pub waterlevel: i32,
    pub timestamp: String,
}

// from IOT device
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WaterLevel {
    pub waterlevel: i32,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum AlertLevel {
    Drought,
    LowWater,
    PlentyWater,
    FullWater,
}
