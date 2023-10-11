use std::env;
use actix_web::{http, HttpResponse};
use async_stream::stream;
use serde_json::json;
use crate::models::responses::chat::ChatCompletionChunk;
use crate::models::responses::chat::ChatCompletionChunkResponse;
use actix_web::web::Bytes;
use actix_web::Error;
use futures::StreamExt;
use actix_web::body::MessageBody;

pub async fn call_openai_api(prompt: String, role: &str) -> HttpResponse {
    let key = env::var("OPEN_AI_KEY").unwrap();

    let res = reqwest::Client::new()
        .post("https://api.openai.com/v1/chat/completions")
        .header("Content-Type", "application/json")
        .bearer_auth(key)
        .json(&json!({
            "model": "gpt-3.5-turbo",
            "messages": [
                {"role": "system", "content": role.to_string()},
                {"role": "user", "content": prompt.to_string()}
            ],
            "temperature": 0.5,
            "stream": true,
            "user": "TestCraftUser"
        }))
        .send()
        .await
        .unwrap();

    let mut bytes_stream = res.bytes_stream();
    let stream = stream! {
        while let Some(item) = bytes_stream.next().await {
            let item = item.unwrap();
            let s = match std::str::from_utf8(&item) {
                Ok(v) => v,
                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
            };

            for p in s.split("\n\n") {
                match p.strip_prefix("data: ") {
                    Some(p) => {
                        if p == "[DONE]" {
                            break;
                        }

                        let d = serde_json::from_str::<ChatCompletionChunk>(p)
                            .expect(format!("Couldn't parse: {}", p).as_str());

                        let response = ChatCompletionChunkResponse { choices: d.choices };

                        let res =
                            format!("data: {}\n\n", serde_json::to_string(&response).unwrap())
                                .try_into_bytes()
                                .unwrap();
                        yield Ok::<Bytes, Error>(res);
                    }
                    None => {}
                }
            }
        }
    };

    HttpResponse::Ok()
        .append_header((http::header::TRANSFER_ENCODING, "chunked"))
        .content_type("text/event-stream")
        .streaming(stream)
}