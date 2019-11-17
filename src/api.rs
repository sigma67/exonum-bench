use serde_json;
use serde_json::json;

use reqwest::{r#async::Client, Response, StatusCode};
//for async
use tokio;
use futures::{stream, Future, Stream};

const URL: &str = "http://localhost:8200/api/explorer/v1/transactions";
const PARALLEL_REQUESTS: usize = 100;

pub fn post(data: &String) -> String{
    let client = reqwest::Client::new();

    let query = &json!({ "tx_body": data });

    //println!("POST {}", url);

    let builder = client.post(&URL.to_string());
    //println!("Body: {}", serde_json::to_string_pretty(&query).unwrap());
    let builder = builder.json(&query);
    let response = builder.send().expect("Unable to send request");

    self::response_to_api_result(response)
}

pub fn post_async(items: Vec<String>) {
    let client = Client::new();
    let bodies = stream::iter_ok(items)
        .map(move |item| {
            let json = &json!({ "tx_body": item });
            client
                .post(&URL.to_string())
                .json(json)
                .send()
                .and_then(|res| res.into_body().concat2().from_err())
        })
        .buffer_unordered(PARALLEL_REQUESTS);

    let work = bodies
        .for_each(|b| {
            Ok(())
        })
        .map_err(|e| panic!("Error while processing: {}", e));

    tokio::run(work);
}

pub fn post_batch(data: Vec<String>){
}

/// Converts reqwest Response to api::Result.
fn response_to_api_result(response: Response) -> String
{
    fn extract_description(body: &str) -> Option<String> {
        match serde_json::from_str::<serde_json::Value>(body).ok()? {
            serde_json::Value::Object(ref object) if object.contains_key("description") => {
                Some(object["description"].as_str()?.to_owned())
            }
            serde_json::Value::String(string) => Some(string),
            _ => None,
        }
    }

    fn error(mut response: Response) -> String {
        let body = response.text().expect("Unable to get response text");
        extract_description(&body).unwrap_or(body)
    }

    match response.status() {
        StatusCode::OK => String::from("OK"),
        _ => error(response),
    }.parse().unwrap()
}
