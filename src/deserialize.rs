use serde::{Serialize, Deserialize, Deserializer};
use crate::error::MyError;



#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct ReceivedData {
    destination: String,
    events: Vec<Event>,
}

impl ReceivedData {
    pub fn get_reply_token(&self) -> Result<String, MyError> {
        let option_s = self.events[0].replyToken.clone();
        match option_s {
            Some(s) => Ok(s),
            None => Err(MyError::FailedToGetReplyToken)
        }
    }

    pub fn get_message_text(&self) -> Result<String, MyError> {
        let option_message = self.events[0].message.clone();
        match option_message {
            Some(message) => {
                if message.message_type == MessageType::text {
                    Ok(message.text.unwrap())
                } else {
                    Err(MyError::UnsupportedMessage)
                }
            }
            None => Err(MyError::UnsupportedMessage)
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[allow(non_snake_case)]
pub struct Event {
    #[serde(rename = "type")]
    event_type: EventType,
    message: Option<ReceivedMessage>, // event_typeがmessageのときのみ存在
    webhookEventId: String, //ULID形式
    deliveryContext: DeliveryContext,
    timestamp: i64,
    source: Option<Source>, //option
    replyToken: Option<String>, // option
    mode: Mode, //standbyのときは送信しちゃだめ（このときreplyTokenは含まれない）
}

#[derive(Debug, Clone, Serialize, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub enum EventType {
    message,
    //else
    Other(String),
}

impl<'de> Deserialize<'de> for EventType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: String = Deserialize::deserialize(deserializer)?;
        if value == "message" {
            Ok(EventType::message)
        }
        else {
            Ok(EventType::Other(value))
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Mode {
    active,
    standby,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[allow(non_snake_case)]
pub struct ReceivedMessage {
    #[serde(rename = "type")]
    message_type: MessageType,
    id: String,
    quoteToken: String,
    text: Option<String> //message_typeがtextのときだけ
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub enum MessageType {
    text, //今回はテキスト以外扱わない
    image,
    video,
    audio,
    file,
    location,
    sticker,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[allow(non_snake_case)]
pub struct DeliveryContext {
    isRedelivery: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[allow(non_snake_case)]
pub struct Source {
    r#type: String,
    userId: String,
}




#[cfg(test)]
mod test {
    use super::*;

    fn deserialize_json(req_body: &str) -> Result<ReceivedData, MyError> {
        let received_message: ReceivedData = serde_json::from_str(req_body)?;
        Ok(received_message)
    }

    #[test]
    fn deserialize_json_test() {
        let json_data = r#"{
            "destination": "DESTINATION_ID",
            "events": [
                {
                    "type": "other",
                    "message": {
                        "type": "text",
                        "id": "123123123123123123",
                        "quoteToken": "LONG_QUOTE_TOKEN",
                        "text": "aaaa"
                    },
                    "webhookEventId": "WEBHOOK_EVENT_ID",
                    "deliveryContext": {
                        "isRedelivery": false
                    },
                    "timestamp": 4564564564564,
                    "source": {
                        "type": "user",
                        "userId": "USER_ID"
                    },
                    "replyToken": "REPLY_TOKEN",
                    "mode": "active"
                }
            ]
        }"#;

        let true_data = ReceivedData {
            destination: "DESTINATION_ID".to_string(),
            events: vec![
                Event {
                    event_type: EventType::Other("other".to_string()),
                    message: Some(ReceivedMessage {
                        message_type: MessageType::text,
                        id: "123123123123123123".to_string(),
                        quoteToken: "LONG_QUOTE_TOKEN".to_string(),
                        text: Some("aaaa".to_string())
                    }),
                    webhookEventId: "WEBHOOK_EVENT_ID".to_string(),
                    deliveryContext: DeliveryContext {
                        isRedelivery: false
                    },
                    timestamp: 4564564564564,
                    source: Some(Source {
                        r#type: "user".to_string(),
                        userId: "USER_ID".to_string()
                    }),
                    replyToken: Some("REPLY_TOKEN".to_string()),
                    mode: Mode::active
                }
            ]
        };
            // 関数を呼び出す
        let result_data = deserialize_json(json_data).expect("failed deserialize json");
        assert_eq!(result_data, true_data);
        // assert_eq!j
    }
}

