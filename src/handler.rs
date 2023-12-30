use std::io;

use actix_web::{HttpResponse, Responder, web::Data};
use awc::Client;
use serde::{Serialize, Deserialize};
use crate::ChannelInfo;



// struct for json serialize to reply
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Message {
    r#type: String,
    text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(non_snake_case)]
struct LineReplyData {
    replyToken: String,
    messages: Vec<Message>,
}



async fn replyMessage(client: &Client, request: &serde_json::Value, appinfo: &Data<ChannelInfo>) -> Result<(), io::Error> {
//     let res = client
            // .post("https://api.line.me/v2/bot/message/reply") 
            // .bearer_auth(appinfo.channel_access_token_.to_string())
            // .content_type("application/json")
            // .send_json(&request)
            // .await.expect("should reply message");

    // let mut config = ClientConfig::builder();

    // let config = Arc::new(config);

    // let connector = Connector::new().rustls(config).finish();
    // let mut client = Client::builder()
    // .connector(connector)
    // .finish(); 

    let res = client
        .post("https://api.line.me/v2/bot/message/push") 
        .bearer_auth(appinfo.channel_access_token_.to_string())
        .content_type("application/json")
        .send_json(&request)
        .await.expect("res should send post message");

    println!("res: {:?}", res);


    Ok(())
}


// handler
pub async fn get_test() -> impl Responder {
    log::info!("received get http requent");
    HttpResponse::Ok().body("Hello world!")
}

pub async fn post_test(req_body: String, appinfo: Data<ChannelInfo>) -> impl Responder {
    log::info!("req_body: {}", req_body);
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
        .await.expect("should reply message");
    
    // replyMessage(&client, &request, &appinfo).await;

    // Debug
    println!("request: {}", request);
    HttpResponse::Ok().body("")
}

