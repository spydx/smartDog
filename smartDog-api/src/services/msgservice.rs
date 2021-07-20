use crate::Fcm;
use actix_web::http::StatusCode;
use actix_web::web::Data;
use actix_web::{HttpResponse, ResponseError};
use fcm::{Client, Error, FcmError, MessageBuilder};
use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;

pub async fn push_msg(con: Data<Fcm>) -> Result<(), CustErr> {
    //unimplemented!("WOrk in progess");
    let client = Client::new();
    let mut map = HashMap::new();
    map.insert("message", "Howdy");

    let mut builder = MessageBuilder::new(&con.key, &con.id);
    builder.data(&map);

    let response = client.send(builder.finalize()).await?;

    dbg!(format!("Sent: {:?}", response));
    Ok(())
}

#[derive(Copy, Clone, Debug)]
struct CustErr {
    msg: &'static str,
}
impl fmt::Display for CustErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl ResponseError for CustErr {
    fn error_response(&self) -> HttpResponse {
        match self.kind_ref() {
            _ => HttpResponse::InternalServerError().finish(),
        }
    }
}
