#[macro_use]
extern crate rocket;

use reqwest::Error;
use rocket::serde::json::Json;
use rocket::{
    http::Status,
    serde::json::{json, Value},
};

use std::fs;

// use serde_json::Value;

use rocket::serde::{Deserialize, Serialize};

// enum Type {
//     Standard,
//     Vip,
//     Extra,
//     Fast,
// }

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct Ticket {
    name: String,
    membership: String,
    period: String,
}

async fn get_issuance_invitation_code(_credential_data: Value) -> Result<Value, Error> {
    let client = reqwest::Client::new();
    let base_url = "https://sandbox-ssi.extrimian.com/v1/credentialsbbs/wacioob";

    // println!("{credential_data}");

    // y si paso este archivo directamente como Value?
    let string_json_data = fs::read_to_string("json_model/base_ticket_template.json")
        .expect("Can't read the json file");

    // let mut json_value = json!(string_json_data);

    let mut json_value: Value =
        serde_json::from_str(&string_json_data).expect("JSON was not well-formatted");

    json_value["vc"]["credentialSubject"]["name"] = json!("nuevo nombre");

    println!("{json_value:#?}");

    let request_response = client
        .put(base_url)
        .header("Content-type", "application/json")
        .body(string_json_data.to_owned())
        .send()
        .await;

    match request_response {
        Ok(response) => {
            let response = response.error_for_status()?;
            let response_body: serde_json::Value = response.json().await?;
            Ok(response_body)
        }
        Err(err) => {
            println!("{err}");
            Err(err)
        }
    }
}

#[get("/ping")]
fn ping() -> &'static str {
    "pong!"
}

#[put("/issue-vc", format = "json", data = "<ticket>")]
async fn create_new_vc(ticket: Json<Ticket>) -> Result<Value, Status> {
    let credential_data = json!(ticket.0);

    match get_issuance_invitation_code(credential_data).await {
        Ok(json) => Ok(json!(json)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[catch(404)]
fn not_found() -> &'static str {
    "Nothing here, sorry!"
}

#[catch(500)]
fn just_500() -> &'static str {
    "Ups, server error!"
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", routes![ping, create_new_vc])
        .register("/", catchers![not_found, just_500])
        .launch()
        .await;
}
