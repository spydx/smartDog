use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(ormx::Table)]
#[ormx(table = "history", id = content_id, insertable)]
pub struct WaterBowlContent {
    #[ormx(column = "id")]
    #[ormx(get_one = get_by_content_id )]
    pub content_id: i32,
    pub waterbowl_id: i32,
    pub waterlevel: i32,
    #[ormx(default, set)]
    pub timestamp: Option<NaiveDateTime>,
}

#[derive(Serialize, ormx::Table)]
#[ormx(table = "bowls", id = bowl_id)]
pub struct Waterbowl {
    #[ormx(column = "id")]
    #[ormx(get_one = get_by_bowl_id)]
    pub bowl_id: i32,
    pub name: String,
    pub disabled: String,
}
