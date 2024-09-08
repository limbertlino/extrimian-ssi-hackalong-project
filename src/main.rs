#[macro_use]
extern crate rocket;

use chrono::{Duration, Local};
use nanoid::nanoid;
use reqwest::Error;
use rocket::{
    http::Status,
    serde::json::{json, Json, Value},
    serde::{Deserialize, Serialize},
};
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
enum Category {
    Standard,
    Vip,
    Fast,
    Extra,
}

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct Ticket {
    name: String,
    category: Category,
}

impl Ticket {
    fn create_new_id() -> String {
        nanoid!()
    }

    fn generate_issuance_date() -> String {
        let current_date = Local::now();
        let formated_current_date = current_date.format("%Y-%m-%dT%H:%M:%S%:z").to_string();
        formated_current_date
    }

    fn generate_expiration_date(hours: i64) -> String {
        let expiration_date = Local::now() + Duration::hours(hours);
        let formated_expiration_date = expiration_date.format("%Y-%m-%dT%H:%M:%S%:z").to_string();
        formated_expiration_date
    }
}

async fn get_issuance_invitation_code(ticket: Ticket) -> Result<Value, Error> {
    let client = reqwest::Client::new();
    let base_url = "https://sandbox-ssi.extrimian.com/v1/credentialsbbs/wacioob";

    let string_json_data = fs::read_to_string("json_model/base_ticket_template.json")
        .expect("Can't read the json file");

    let mut json_value: Value =
        serde_json::from_str(&string_json_data).expect("JSON was not well-formatted");

    //TODO cambiar colores de backup para cada credendial
    match ticket.category {
        Category::Standard => {
            json_value["vc"]["id"] = json!(Ticket::create_new_id());
            json_value["vc"]["issuanceDate"] = json!(Ticket::generate_issuance_date());
            json_value["vc"]["expirationDate"] = json!(Ticket::generate_expiration_date(8));
            json_value["vc"]["credentialSubject"]["name"] = json!(&ticket.name);
            json_value["vc"]["credentialSubject"]["category"] = json!("Standard");
            json_value["outputDescriptor"]["display"]["title"]["text"] = json!("Regular pass");
            json_value["outputDescriptor"]["display"]["description"]["text"] =
                json!("With this credential, you have the following benefits/access:\n- Can access the main attractions\n- Free water at designated points\n- Welcome snack and ice cream\n- Personalized welcome upon entering the park");
            json_value["outputDescriptor"]["styles"]["hero"]["uri"] =
                json!("https://limbertlino.github.io/schemas/images/regular.png");
            json_value["issuer"]["styles"]["hero"]["uri"] =
                json!("https://limbertlino.github.io/schemas/images/regular.png");
            json_value["outputDescriptor"]["styles"]["background"]["color"] = json!("#245A8B");
            json_value["issuer"]["styles"]["background"]["color"] = json!("#245A8B");
        }
        Category::Vip => {
            json_value["vc"]["id"] = json!(Ticket::create_new_id());
            json_value["vc"]["issuanceDate"] = json!(Ticket::generate_issuance_date());
            json_value["vc"]["expirationDate"] = json!(Ticket::generate_expiration_date(8));
            json_value["vc"]["credentialSubject"]["name"] = json!(&ticket.name);
            json_value["vc"]["credentialSubject"]["category"] = json!("Vip");
            json_value["outputDescriptor"]["display"]["title"]["text"] = json!("Vip pass");
            json_value["outputDescriptor"]["display"]["description"]["text"] =
                json!("With this credential, you have the following benefits/access:\n- Access to the park's premium facilities (15 premium + 15 main attractions)\n- Priority entrance to attractions\n- Fast pass for 5 attractions\n- Access to the general food buffet\n- Unlimited soft drinks and water at all points in the park\n- Unlimited photos within the park\n- Access to the park's pools\n- Access to VIP lounge areas\n- 50% discount on fast pass\n- Priority access to the night show and a 35% discount");
            json_value["outputDescriptor"]["styles"]["hero"]["uri"] =
                json!("https://limbertlino.github.io/schemas/images/vip.png");
            json_value["issuer"]["styles"]["hero"]["uri"] =
                json!("https://limbertlino.github.io/schemas/images/vip.png");
            json_value["outputDescriptor"]["styles"]["background"]["color"] = json!("#FFD700");
            json_value["issuer"]["styles"]["background"]["color"] = json!("#FFD700");
        }
        Category::Extra => {
            json_value["vc"]["id"] = json!(Ticket::create_new_id());
            json_value["vc"]["issuanceDate"] = json!(Ticket::generate_issuance_date());
            json_value["vc"]["expirationDate"] = json!(Ticket::generate_expiration_date(8));
            json_value["vc"]["credentialSubject"]["name"] = json!(&ticket.name);
            json_value["vc"]["credentialSubject"]["category"] = json!("Extra");
            json_value["outputDescriptor"]["display"]["title"]["text"] = json!("Extra pass");
            json_value["outputDescriptor"]["display"]["description"]["text"] =
                json!("With this credential, you have the following benefits/access:\n- Access to the full food buffet (25% discount on seasonal special meals)\n- Access to the pool in the morning and afternoon\n- Access to the night show\n- Unlimited photos within the park\n- Rental of a locker for valuable items\n- Priority reservation at the restaurant\n- In-park transportation service");
            json_value["outputDescriptor"]["styles"]["hero"]["uri"] =
                json!("https://limbertlino.github.io/schemas/images/extra.png");
            json_value["issuer"]["styles"]["hero"]["uri"] =
                json!("https://limbertlino.github.io/schemas/images/extra.png");
            json_value["outputDescriptor"]["styles"]["background"]["color"] = json!("#E67E22");
            json_value["issuer"]["styles"]["background"]["color"] = json!("#E67E22");
        }
        Category::Fast => {
            json_value["vc"]["id"] = json!(Ticket::create_new_id());
            json_value["vc"]["issuanceDate"] = json!(Ticket::generate_issuance_date());
            json_value["vc"]["expirationDate"] = json!(Ticket::generate_expiration_date(8));
            json_value["vc"]["credentialSubject"]["name"] = json!(&ticket.name);
            json_value["vc"]["credentialSubject"]["category"] = json!("Fast Pass");
            json_value["outputDescriptor"]["display"]["title"]["text"] = json!("Fast pass");
            json_value["outputDescriptor"]["display"]["description"]["text"] =
                json!("With this credential, you have the following benefit/access:\n- Fast pass to all attractions");
            json_value["outputDescriptor"]["styles"]["hero"]["uri"] =
                json!("https://limbertlino.github.io/schemas/images/fast.png");
            json_value["issuer"]["styles"]["hero"]["uri"] =
                json!("https://limbertlino.github.io/schemas/images/fast.png");
            json_value["outputDescriptor"]["styles"]["background"]["color"] = json!("#28B463");
            json_value["issuer"]["styles"]["background"]["color"] = json!("#28B463");
        }
    }

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
    match get_issuance_invitation_code(ticket.0).await {
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
