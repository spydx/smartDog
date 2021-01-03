use serde::{Serialize, Deserialize};

use crate::schema::bowls;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewBowl {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone,  Queryable, Insertable,Identifiable)]
#[table_name = "bowls"]
pub struct Bowls {
    pub id: String,
    pub name: String,
    pub waterlevel: i32,
    pub timestamp: String
}

// from IOT device
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WaterLevel {
    pub waterlevel: i32
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum AlertLevel {
    Drought,
    LowWater,
    PlentyWater,
    FullWater
}
/*
// systemstatus
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Health {
    pub db: bool,
    pub status: AlertLevel,
    pub bowl: vec!BowlContent,
    pub time: String
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum HealthAlert {
    Ok,
    Warning,
    Failed,
}
*/