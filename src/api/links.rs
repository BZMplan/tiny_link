use actix_web::{web::Json, Responder};
use serde::{Deserialize,Serialize};
use nanoid::nanoid;
use actix_web::{post,get,web::{Path},HttpResponse,http::header};

#[derive(Deserialize,Serialize,Debug,Clone)]
struct Links{
    tiny_code:String,
    origin_url:String,
}

#[derive(Deserialize,Clone)]
struct ApiAddLink{

    origin_url:String,
}

impl ApiAddLink {
    fn to_new_link(self) -> Links {
        Links{
            tiny_code:nanoid!(5),
            origin_url:self.origin_url,
        }

    }
}

#[post("/create")]
async fn create_link(link:Json<ApiAddLink>)-> impl Responder{
    let new_link = link.0.to_new_link();
    let new_code = new_link.tiny_code.clone();
    Json(new_code)
}

#[get("/{code}")]
async fn get_from_link()-> impl Responder{
    let url = "http://baidu.com";
    HttpResponse::Found().append_header((header::LOCATION,url)).finish()
}
