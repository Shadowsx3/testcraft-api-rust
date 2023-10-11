use std::env;
use actix_web::{http, HttpResponse};
use async_stream::stream;
use serde_json::json;
use crate::models::responses::chat::{ChatCompletionChunk, ChatCompletionChunkResponse};
use actix_web::Error;
use futures::StreamExt;

pub async fn call_openai_api(prompt: String, role: &str) -> HttpResponse {
    let key = env::var("OPEN_AI_KEY").expect("OPEN_AI_KEY environment variable is not set");
    let model = env::var("MODEL").expect("MODEL environment variable is not set");

    let res = reqwest::Client::new()
        .post("https://api.openai.com/v1/chat/completions")
        .header("Content-Type", "application/json")
        .bearer_auth(&key)
        .json(&json!({
            "model": &model,
            "messages": [
                {"role": "system", "content": role},
                {"role": "user", "content": prompt}
            ],
            "temperature": 0.5,
            "stream": true,
            "user": "TestCraftUser"
        }))
        .send()
        .await
        .expect("Failed to send request to OpenAI");

    let mut bytes_stream = res.bytes_stream();
    let stream = stream! {
        while let Some(item) = bytes_stream.next().await {
            let item = item.expect("Error while reading response bytes");
            let s = match std::str::from_utf8(&item) {
                Ok(v) => v,
                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
            };

            for p in s.split("\n\n") {
                if let Some(p) = p.strip_prefix("data: ") {
                    if p == "[DONE]" {
                        break;
                    }

                    let d = serde_json::from_str::<ChatCompletionChunk>(p)
                        .expect(&format!("Couldn't parse: {}", p));

                    let response = ChatCompletionChunkResponse { choices: d.choices };

                    let res = format!("data: {}\n\n", serde_json::to_string(&response).unwrap());
                    yield Ok::<_, Error>(res.into());
                }
            }
        }
    };

    HttpResponse::Ok()
        .append_header((http::header::TRANSFER_ENCODING, "chunked"))
        .content_type("text/event-stream")
        .streaming(stream)
}