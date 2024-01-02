use actix_web::{HttpResponse, Responder, web::Data, http::StatusCode};
use awc::Client;
use serde::Serialize;
use ring::hmac;
use data_encoding::BASE64;

use crate::ChannelInfo;
use crate::error::MyError;
use crate::deserialize::*;
//type Result<T> = std::result::Result<T, MyError>;

// reply data struct
#[derive(Debug, Clone, Serialize)]
pub struct ReplyMessage {
    #[serde(rename = "type")]
    message_type: String,
    text: String,
}

#[derive(Debug, Clone, Serialize)]
#[allow(non_snake_case)]
struct ReplyData {
    replyToken: String,
    messages: Vec<ReplyMessage>,
}


async fn reply_message(client: &Client, request: &serde_json::Value, appinfo: &Data<ChannelInfo>) -> Result<StatusCode, MyError> {
    let reply_result_response = client
        .post("https://api.line.me/v2/bot/message/reply") 
        .bearer_auth(appinfo.channel_access_token_.to_string())
        .content_type("application/json")
        .send_json(&request)
        .await?;
    log::info!("res: {:?}", reply_result_response);
    
    Ok(reply_result_response.status())
}


fn verify_signature(header_data: &HeaderData, req_body: &String, appinfo: &Data<ChannelInfo>) -> Result<(), MyError> {
    // use HMAC-SHA256 using channel_secret_ as secret key
    let key = hmac::Key::new(hmac::HMAC_SHA256, appinfo.channel_secret_.as_bytes());
    let msg = req_body;
    let tag = hmac::sign(&key, msg.as_bytes());
    let base64_encoded_tag = BASE64.encode(tag.as_ref());
    if base64_encoded_tag == header_data.get_signature() {
        Ok(())
    } else {
        Err(MyError::FailedToVerifySignature)
    }
}



// handler
pub async fn get_test() -> impl Responder {
    log::info!("received get http requent");
    HttpResponse::Ok().body("Hello world!")
}

pub async fn post_test(header_data: HeaderData, req_body: String, appinfo: Data<ChannelInfo>) -> impl Responder {
    log::info!("req_body: {}", req_body);

    verify_signature(&header_data, &req_body, &appinfo).unwrap();

    // deserialize json and get essential data
    let received_json: ReceivedData = serde_json::from_str(&req_body).expect("failed to deserialize json"); 
    let reply_token = received_json.get_reply_token().expect("cannot reply");
    let message_text = received_json.get_message_text().unwrap_or("このメッセージには対応していません".to_string());

    // make json data to reply
    let request = serde_json::json!(ReplyData {
        replyToken: reply_token,
        messages: vec![
            ReplyMessage {
                message_type: "text".to_string(),
                text: message_text,
            }
        ]
    });

    let client = Client::default();
   
    let reply_result = reply_message(&client, &request, &appinfo).await.unwrap();
    log::info!("reply_result_status_code: {:?}", reply_result);

    HttpResponse::Ok().body("")
}

