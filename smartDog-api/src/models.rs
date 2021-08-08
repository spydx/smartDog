use chrono::NaiveDateTime;

#[derive(ormx::Table)]
#[ormx(table = "history", id = content_id, insertable)]
struct WaterBowlContent {
    #[ormx(column = "id")]
    #[ormx(get_one = get_by_content_id )]
    content_id: i32,
    waterbowl_id: i32,
    waterlevel: i32,
    #[ormx(default, set)]
    timestamp: Option<NaiveDateTime>,
}

#[derive(ormx::Table)]
#[ormx(table = "bowls", id = bowl_id)]
struct Waterbowl {
    #[ormx(column = "id")]
    #[ormx(get_one = get_by_bowl_id)]
    bowl_id: i32,
    name: String,
    disabled: String,
}
