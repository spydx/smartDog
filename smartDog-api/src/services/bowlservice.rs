use diesel::prelude::*;
use std::option::Option;
use uuid::Uuid;

//use crate::models::bowls::Bowls;
//use crate::schema::bowls::dsl::*;
use schema::bowls::dsl::*;

use diesel::result::Error;
use crate::models::Bowls;
use crate::schema::bowls::dsl::{id};


pub fn find_all_bowls(con: &SqliteConnection) -> Result<QueryResult<Vec<Bowls>>, Error> {
    let list = bowls.load(con);
    Ok(list)
}

pub fn find_bowl_by_uuid(uid: Uuid, con: &SqliteConnection) -> Result<Option<Bowls>, Error> {
    let bowl = bowls
        .filter(id.eq(uid.to_string()))
        .first::<Bowls>(con)
        .optional()?;

    Ok(bowl)
}

pub fn update_bowl_id(
    uid: Uuid,
    con: &SqliteConnection,
    data: i32,
) -> Result<Option<Bowls>, Error> {
    let bowl = bowls
        .filter(id.eq(uid.to_string()))
        .first::<Bowls>(con)
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

pub fn delete_bowl_id(uid: Uuid, con: &SqliteConnection) -> Result<usize, diesel::result::Error> {
    let bowl = bowls
        .filter(id.eq(uid.to_string()))
        .first::<Bowls>(con)
        .optional()?;

    if let Some(bowl) = bowl {
        let res = diesel::delete(&bowl)
            .execute(con)
            .expect("Error deleting bowl");
        Ok(res)
    } else {
        Err(Error::NotFound)
    }
}

pub fn insert_bowl(
    bowl: Bowls,
    con: &SqliteConnection,
) -> Result<Option<Bowls>, diesel::result::Error> {
    let uid = bowl.id.clone();

    diesel::insert_into(bowls).values(&bowl).execute(con)?;

    let res = bowls.filter(id.eq(uid)).first::<Bowls>(con).optional()?;

    Ok(res)
}
