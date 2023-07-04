use chrono::Utc;
use data_encoding::BASE64;
use futures::{SinkExt, Stream, StreamExt};
use ring::hmac;
use serde_json::{json, Value};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use tungstenite::error::Error as WsError;
use url::Url;

pub struct SparkAI {
    app_id: String,
    api_key: String,
    api_secret: String,
    api_url: String,
}

impl SparkAI {
    pub fn new(app_id: &str, api_key: &str, api_secret: &str, api_url: &str) -> Self {
        Self {
            app_id: app_id.to_string(),
            api_key: api_key.to_string(),
            api_secret: api_secret.to_string(),
            api_url: api_url.to_string(),
        }
    }

    pub fn get_authorization_url(&self) -> String {
        let url = Url::parse(&self.api_url).unwrap();
        let date = Utc::now().format("%a, %d %b %Y %H:%M:%S %Z").to_string();
        let signature_origin = format!(
            "host: {}\ndate: {}\nGET {} HTTP/1.1",
            url.host_str().unwrap(),
            date,
            url.path()
        );
        let s_key = hmac::Key::new(hmac::HMAC_SHA256, self.api_secret.as_bytes());
        let tag = hmac::sign(&s_key, signature_origin.as_bytes());
        let signature = BASE64.encode(tag.as_ref());
        let authorization_origin = format!(
            "api_key=\"{}\",algorithm=\"{}\",headers=\"{}\",signature=\"{}\"",
            self.api_key, "hmac-sha256", "host date request-line", signature
        );
        let authorization = BASE64.encode(authorization_origin.as_bytes());
        let params = vec![
            ("authorization", authorization),
            ("date", date),
            ("host", url.host_str().unwrap().to_string()),
        ];
        let ws_url = format!(
            "{}?{}",
            self.api_url,
            url::form_urlencoded::Serializer::new(String::new())
                .extend_pairs(params)
                .finish()
        );
        // println!("ws url: {}", ws_url);
        ws_url
    }

    pub fn get_prompt(&self, query: &str, history: &mut Vec<Value>) -> Value {
        let user_message = json!({"role": "user", "content": query});
        history.push(user_message);
        let message = json!({ "text": history });
        message
    }

    pub fn build_inputs(
        &self,
        message: Value,
        user_id: &str,
        domain: &str,
        temperature: f32,
        max_tokens: u32,
    ) -> String {
        let input_dict = json!({
            "header": {
                "app_id": self.app_id,
                "uid": user_id
            },
            "parameter": {
                "chat": {
                    "domain": domain,
                    "temperature": temperature,
                    "max_tokens": max_tokens,
                }
            },
            "payload": {
                "message": message
            }
        });
        input_dict.to_string()
    }

    pub fn process_response(&self, response_str: &str, history: &mut Vec<Value>) -> (String, i64) {
        let res_dict: Value = serde_json::from_str(response_str).unwrap();
        let header = res_dict["header"].as_object().unwrap();
        let code = header["code"].as_i64().unwrap();
        let status = header.get("status").map_or(2, |v| v.as_i64().unwrap());
        let mut response = String::new();
        if code == 0 {
            let text = res_dict["payload"]["choices"]["text"][0]
                .as_object()
                .unwrap();
            let res_content = text["content"].as_str().unwrap();
            if !text.is_empty() && !res_content.is_empty() {
                let mut res_dict = text.clone();
                res_dict.remove("index");
                response = res_content.to_string();
                if status == 0 {
                    history.push(res_dict.into());
                } else {
                    let last_message = history.last_mut().unwrap().as_object_mut().unwrap();
                    let mut content = last_message
                        .get_mut("content")
                        .unwrap()
                        .as_str()
                        .unwrap()
                        .to_string();
                    content.push_str(res_content);
                    response = content.clone();
                    last_message.insert("content".to_string(), Value::String(content));
                }
            }
        } else {
            eprintln!("error code {}", code);
            eprintln!("you can see this website to know code detail");
            eprintln!("https://www.xfyun.cn/doc/spark/%E6%8E%A5%E5%8F%A3%E8%AF%B4%E6%98%8E.html");
        }
        (response, status)
    }

    pub async fn chat_stream<'a>(
        &'a mut self,
        query: &str,
        history: &'a mut Vec<Value>,
        user_id: &str,
        domain: &str,
        max_tokens: u32,
        temperature: f32,
    ) -> impl Stream<Item = Result<(String, Vec<Value>), WsError>> + '_ {
        let max_tokens = max_tokens.min(4096);
        let url = self.get_authorization_url();
        // println!("url: {}", url);
        // let (ws_stream, _) = connect_async(Url::parse(&url).unwrap()).await.unwrap();
        let parsed_url = Url::parse(&url);
        match parsed_url {
            Ok(valid_url) => {
                match connect_async(valid_url).await {
                    Ok((ws_stream, _)) => {
                        // Continue with your logic here using ws_stream
                        let (mut write, read) = ws_stream.split();
                        let message = self.get_prompt(query, history);
                        let input_str =
                            self.build_inputs(message, user_id, domain, temperature, max_tokens);
                        tokio::spawn(async move {
                            write.send(Message::Text(input_str)).await.unwrap();
                        });
                        read.map(move |message| match message {
                            Ok(msg) => {
                                if msg.is_text() || msg.is_binary() {
                                    let response_str = msg.to_text().unwrap();
                                    let (response, _status) =
                                        self.process_response(response_str, history);
                                    Ok((response.clone(), history.clone()))
                                } else {
                                    Err(WsError::Protocol(
                                        tungstenite::error::ProtocolError::NonZeroReservedBits,
                                    ))
                                }
                            }
                            Err(e) => Err(e),
                        })
                    }
                    Err(e) => {
                        if let tungstenite::error::Error::Http(response) = e {
                            println!("HTTP status: {}", response.status());
                            if response.status() == 401 {
                                panic!("The provided API secret may be incorrect.");
                            } else {
                                panic!("An error occurred: {:#?}", response);
                            }
                        } else {
                            panic!("An error occurred: {}", e);
                        }
                    }
                }
            }
            Err(e) => {
                panic!("URL parsing failed: {}", e);
            }
        }
    }
}
