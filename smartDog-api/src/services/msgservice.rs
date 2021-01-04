use fcm::{MessageBuilder, Client};
use std::collections::HashMap;
use crate::Fcm;
use actix_web::web::Data;

pub async fn push_msg(con: Data<Fcm>) -> Result<(), fcm::Error> {
    //unimplemented!("WOrk in progess");
    let client = Client::new();
    let mut map = HashMap::new();
    map.insert("message", "Howdy");

    let mut builder = MessageBuilder::new(&con.key,&con.id);
    builder.data(&map);

    let res = client.send(builder.finalize()).await?;
    dbg!(format!("Sent: {:?}", res));
    Ok(())
}

