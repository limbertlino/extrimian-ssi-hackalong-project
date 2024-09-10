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

impl std::fmt::Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let category_str = match self {
            Category::Standard => "Standard",
            Category::Vip => "Vip",
            Category::Fast => "Fast Pass",
            Category::Extra => "Extra",
        };
        write!(f, "{}", category_str)
    }
}

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct Ticket {
    name: String,
    category: Category,
}

impl Ticket {
    fn create_new_id(&self) -> String {
        nanoid!()
    }

    fn generate_issuance_date(&self) -> String {
        let current_date = Local::now();
        let formated_current_date = current_date.format("%Y-%m-%dT%H:%M:%S%:z").to_string();
        formated_current_date
    }

    fn generate_expiration_date(&self, hours: i64) -> String {
        let expiration_date = Local::now() + Duration::hours(hours);
        let formated_expiration_date = expiration_date.format("%Y-%m-%dT%H:%M:%S%:z").to_string();
        formated_expiration_date
    }
}

struct CategoryData {
    title: &'static str,
    description: &'static str,
    hero_uri: &'static str,
    background_color: &'static str,
}

fn get_category_data(category: &Category) -> CategoryData {
    match category {
        Category::Standard => CategoryData {
            title: "Regular Pass",
            description: "With this credential, you have the following benefits/access:\n- Can access the main attractions\n- Free water at designated points\n- Welcome snack and ice cream\n- Personalized welcome upon entering the park",
            background_color: "#245A8B",
            hero_uri: "https://limbertlino.github.io/schemas/images/regular.png",
        },
        Category::Vip => CategoryData {
            title: "Vip Pass",
            description: "With this credential, you have the following benefits/access:\n- Access to the park's premium facilities (15 premium + 15 main attractions)\n- Priority entrance to attractions\n- Fast pass for 5 attractions\n- Access to the general food buffet\n- Unlimited soft drinks and water at all points in the park\n- Unlimited photos within the park\n- Access to the park's pools\n- Access to VIP lounge areas\n- 50% discount on fast pass\n- Priority access to the night show and a 35% discount",
            background_color: "#FFD700",
            hero_uri: "https://limbertlino.github.io/schemas/images/vip.png",
        },
        Category::Fast => CategoryData {
            title: "Fast Pass",
            description: "With this credential, you have the following benefit/access:\n- Fast pass to all attractions",
            background_color: "#28B463",
            hero_uri: "https://limbertlino.github.io/schemas/images/fast.png",
        },
        Category::Extra => CategoryData {
            title: "Extra Pass",
            description: "With this credential, you have the following benefits/access:\n- Access to the full food buffet (25% discount on seasonal special meals)\n- Access to the pool in the morning and afternoon\n- Access to the night show\n- Unlimited photos within the park\n- Rental of a locker for valuable items\n- Priority reservation at the restaurant\n- In-park transportation service",
            background_color: "#E67E22",
            hero_uri: "https://limbertlino.github.io/schemas/images/extra.png",
        },
    }
}

fn update_common_fields(
    json_value: &mut Value,
    ticket: &Ticket,
    id: &str,
    issuance_date: &str,
    expiration_date: &str,
) {
    json_value["vc"]["id"] = json!(id);
    json_value["vc"]["issuanceDate"] = json!(issuance_date);
    json_value["vc"]["expirationDate"] = json!(expiration_date);
    json_value["vc"]["credentialSubject"]["name"] = json!(&ticket.name);
    json_value["vc"]["credentialSubject"]["category"] = json!(ticket.category.to_string());
}

async fn get_issuance_invitation_code(ticket: Ticket) -> Result<Value, Error> {
    let client = reqwest::Client::new();
    let base_url = "https://sandbox-ssi.extrimian.com/v1/credentialsbbs/wacioob";

    let string_local_json_data = fs::read_to_string("json_model/base_ticket_template.json")
        .expect("Can't read the json file");

    let mut json_value: Value =
        serde_json::from_str(&string_local_json_data).expect("JSON was not well-formatted");

    let category = &ticket.category;
    let data = get_category_data(category);

    update_common_fields(
        &mut json_value,
        &ticket,
        &ticket.create_new_id(),
        &ticket.generate_issuance_date(),
        &ticket.generate_expiration_date(8),
    );

    json_value["outputDescriptor"]["display"]["title"]["text"] = json!(data.title);
    json_value["outputDescriptor"]["display"]["description"]["text"] = json!(data.description);
    json_value["outputDescriptor"]["styles"]["hero"]["uri"] = json!(data.hero_uri);
    json_value["issuer"]["styles"]["hero"]["uri"] = json!(data.hero_uri);
    json_value["outputDescriptor"]["styles"]["background"]["color"] = json!(data.background_color);
    json_value["issuer"]["styles"]["background"]["color"] = json!(data.background_color);

    let updated_json_string =
        serde_json::to_string(&json_value).expect("Failed to convert JSON value to string");

    let request_response = client
        .put(base_url)
        .header("Content-type", "application/json")
        .body(updated_json_string.to_owned())
        .send()
        .await;

    match request_response {
        Ok(response) => {
            let response = response.error_for_status()?;
            let response_body: Value = response.json().await?;
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
fn handle_not_found() -> Value {
    json!({ "error": 404, "message": "Resource not found" }
    )
}

#[catch(500)]
fn handle_just_500() -> Value {
    json!({ "error": 500, "message": "Internal server error" }
    )
}

#[catch(422)]
fn handle_unproccessable_entity() -> Value {
    json!({ "error": 422, "message": "Unprocessable entity: Validation failed" }
    )
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", routes![ping, create_new_vc])
        .register(
            "/",
            catchers![
                handle_not_found,
                handle_just_500,
                handle_unproccessable_entity
            ],
        )
        .launch()
        .await;
}
