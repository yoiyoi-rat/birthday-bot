// //use actix_web::{get, post, web, HttpServer, App, HttpRequest, HttpResponse, Responder};
use actix_web::get;
use actix_web::post;
use actix_web::web;
use actix_web::HttpServer;
use actix_web::App;
// use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::Responder;


use dotenv::dotenv;
use serde::Serialize;
use std::env;
// use serde::Deserialize;
// use serde_json::Value;


struct AppInfo {
    // channel_id_: String,
    // channel_secret_: String,
    channel_access_token_: String,
}

impl AppInfo {
    pub fn new() -> AppInfo {
        dotenv().ok();
        AppInfo {
            // channel_id_: env::var("CHANNEL_ID").unwrap(),
            // channel_secret_: env::var("CHANNEL_SECRET").unwrap(),
            channel_access_token_: env::var("CHANNEL_ACCESS_TOKEN").unwrap(),
        }
    }
}



#[derive(Serialize)]
struct Message {
    r#type: String,
    text: String,
}

#[derive(Serialize)]
struct ResponseData {
    reply_token_: String,
    messages: Vec<Message>,
}


#[get("/webhook")]
async fn get_test() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/webhook")]
async fn post_test(req_body: String) -> impl Responder {
    let appinfo = AppInfo::new();

    let v:serde_json::Value = serde_json::from_str(&req_body).unwrap();
    let replytoken = v["events"][0]["replyToken"].to_string();
    let message_text = v["events"][0]["message"]["text"].to_string();
    let authorization_value = "Bearer ".to_string() + appinfo.channel_access_token_.as_str();

    HttpResponse::Ok()
        .append_header(("Authorization", authorization_value))
        .content_type("application/json")
        //.body(message_text)
        .json(ResponseData {
            reply_token_: replytoken,
            messages: vec![
                Message {
                    r#type: "text".to_string(),
                    text: message_text,
                }
            ]
        })
}

// async fn manual_hello() -> impl Responder {
//     HttpResponse::Ok().body("Hey there!")
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    //let appinfo = AppInfo::new();

    HttpServer::new(|| {
        App::new()
            //.app_data(web::Data::new(appinfo))
            .service(get_test)
            .service(post_test)
            //.route("/", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}





// #[derive(Deserialize)]
// struct Message {
//     id: String,
//     message_type: String,
//     text: String,
// }

// #[derive(Deserialize)]
// struct Event {
//     replyToken_: String,
//     event_type_: String,
//     messages_: Vec<Message>,
// }

// #[derive(Deserialize)]
// struct Events {
//     events_: Vec<Event>,
// }

// #[post("/webhook")]
// pub async fn index(events: web::Json<Events>) -> impl Responder {
//     HttpResponse::Ok().body(events) 
// }