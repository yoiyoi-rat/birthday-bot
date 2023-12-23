use actix_web::{HttpResponse, Responder, web::Data};
use awc::Client;
use serde::Serialize;
use crate::ChannelInfo;




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
pub async fn get_test() -> impl Responder {
    log::info!("received get http requent");
    HttpResponse::Ok().body("Hello world!")
}

pub async fn post_test(req_body: String, appinfo: Data<ChannelInfo>) -> impl Responder {
    log::info!("received post http request");
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
    let _res = client
            .post("https://api.line.me/v2/bot/message/reply") 
            .bearer_auth(appinfo.channel_access_token_.to_string())
            .content_type("application/json")
            .send_json(&request)
            .await;

    // Debug
    println!("{}", req_body);
    println!("{}", request);
    HttpResponse::Ok().body("")
}

