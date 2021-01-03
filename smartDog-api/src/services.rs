use diesel::prelude::*;
use uuid::Uuid;

use crate::models;
use crate::models::{Bowls};
use crate::schema::bowls::dsl::*;
use diesel::result::{Error};

pub fn find_bowl_by_uuid(
    uid: Uuid,
    con: &SqliteConnection,
) -> Result<Option<models::Bowls>,Error> {

    let bowl = bowls
        .filter(id.eq(uid.to_string()))
        .first::<models::Bowls>(con)
        .optional()?;

    Ok(bowl)
}

pub fn update_bowl_id(
    uid: Uuid,
    con : &SqliteConnection,
    data: i32,
    ) -> Result<Option<models::Bowls>, Error> {

    let bowl = bowls
        .filter(id.eq(uid.to_string()))
        .first::<models::Bowls>(con)
        .optional()?;

    if let Some(bowl) = bowl {
        diesel::update(&bowl)
            .set(waterlevel.eq(data))
            .execute(con)?;

        let update = Bowls {
            waterlevel: data,
            ..bowl
        };

        Ok(Some(update))
    } else {
        Err(diesel::NotFound)
    }


}

pub fn insert_bowl(
    bowl: Bowls,
    con: &SqliteConnection,
    ) -> Result<Option<models::Bowls>, diesel::result::Error> {
    let uid = bowl.id.clone();

    diesel::insert_into(bowls)
        .values(&bowl)
        .execute(con)?;

    let res = bowls
        .filter(id.eq(uid))
        .first::<models::Bowls>(con)
        .optional()?;

    Ok(res)

}