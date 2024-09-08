#[macro_use]
extern crate rocket;

use reqwest::Error;
use rocket::serde::json::Json;
// use rocket::time::Duration;
use rocket::{
    http::Status,
    serde::json::{json, Value},
};

use chrono::{Duration, Local};
use std::fs;

use nanoid::nanoid;
// use uuid::Uuid;

// use serde_json::Value;

use rocket::serde::{Deserialize, Serialize};



#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct Ticket {
    name: String,
    category: String,
}

//TODO crear funciones o implementar para obtener la fecha actual

//TODO cambiar el comportamiento para que se pongan los datos desde credential data
async fn get_issuance_invitation_code(credential_data: Value) -> Result<Value, Error> {
    let client = reqwest::Client::new();
    let base_url = "https://sandbox-ssi.extrimian.com/v1/credentialsbbs/wacioob";

    //* campos de mi json */
    // let new_id = Uuid::new_v4().to_string();
    let new_id = nanoid!();

    let new_name = credential_data;
    println!("{}", new_name);

    //* hora */
    let issuance_date = Local::now();
    let expiration_date = issuance_date + Duration::hours(8);

    let formatted_issuance_date = issuance_date.format("%Y-%m-%dT%H:%M:%S%:z").to_string();
    let formatted_expiration_date = expiration_date.format("%Y-%m-%dT%H:%M:%S%:z").to_string();

    println!("nanoid: {}", new_id);
    println!("Issuance date: {}", formatted_issuance_date);
    println!("Expiration date: {}", formatted_expiration_date);

    let string_json_data = fs::read_to_string("json_model/base_ticket_template.json")
        .expect("Can't read the json file");

    let mut json_value: Value =
        serde_json::from_str(&string_json_data).expect("JSON was not well-formatted");

    json_value["vc"]["credentialSubject"]["name"] = json!("nuevo nombre");
    json_value["vc"]["id"] = json!(new_id);
    // json_value["vc"]["credentialSubject"] = credential_data.name;

    let updated_json_string =
        serde_json::to_string(&json_value).expect("Failed to convert JSON value to string");

    let request_response = client
        .put(base_url)
        .header("Content-type", "application/json")
        .body(updated_json_string.to_owned())
        .send()
        .await;

    //TODO debo sacar el didcom desde aqui para usarlo en la generacion de qr
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
