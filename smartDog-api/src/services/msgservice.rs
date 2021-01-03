use std::collections::HashMap;
use env_logger::builder;

pub async fn push_msg() -> () {
    unimplemented!("WOrk in progess");
    let client = fcm::Client::new();
    let mut map = HashMap::new();
    map.insert("message", "Howdy");

    let mut builder = fcm::MessageBuilder::new("","");
    builder.data(&map);

    //let res = client.send(builder.finalize()).await?;
    //dbg!(format!("Sent: {:?}", res));


}