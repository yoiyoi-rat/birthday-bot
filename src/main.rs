use actix_web::{middleware::Logger, web::{self, Data}, HttpServer, App, HttpResponse, Responder};
use awc::Client;
use dotenv::dotenv;
use serde::{Serialize, Deserialize};
use std::env;


// define app state information
#[derive(Debug, Clone)]
struct AppInfo {
    channel_id_: String,
    channel_secret_: String,
    channel_access_token_: String,
}

impl AppInfo {
    pub fn new() -> AppInfo {
        dotenv().ok();
        AppInfo {
            channel_id_: env::var("CHANNEL_ID").unwrap(),
            channel_secret_: env::var("CHANNEL_SECRET").unwrap(),
            channel_access_token_: env::var("CHANNEL_ACCESS_TOKEN").unwrap(),
        }
    }
}


// struct for json serialize to reply
#[derive(Serialize)]
struct Message {
    r#type: String,
    text: String,
}

#[derive(Serialize)]
struct LineReplyData {
    replyToken: String,
    messages: Vec<Message>,
}


// handler
async fn get_test() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn post_test(req_body: String, appinfo: Data<AppInfo>) -> impl Responder {
    // extract value from request body json
    let v:serde_json::Value = serde_json::from_str(&req_body).unwrap();
    let reply_token = v["events"][0]["replyToken"].as_str().unwrap().to_string();
    let message_text = v["events"][0]["message"]["text"].as_str().unwrap().to_string();

    // make json data to reply
    let request = serde_json::json!(LineReplyData {
        replyToken: reply_token,
        messages: vec![
            Message {
                r#type: "text".to_string(),
                text: message_text,
            }
        ]
    });

    // TODO: 
    // send post request
    let client = Client::default();
    let res = client
            .post("https://api.line.me/v2/bot/message/reply") 
            .bearer_auth(appinfo.channel_access_token_.to_string())
            .content_type("application/json")
            .send_json(&request)
            .await;

    // Debug
    // println!("{}", req_body);
    // println!("{}", request);
    HttpResponse::Ok().body("")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // import ans set dotenv value
    let appinfo = Data::new(AppInfo::new());

    // logger set
    env_logger::init();

    // build server
    HttpServer::new(move || {
        App::new()
        .wrap(Logger::default())
        .app_data(Data::clone(&appinfo))
        .route("/webhook", web::get().to(get_test))
        .route("/webhook", web::post().to(post_test))
        
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
    
}



