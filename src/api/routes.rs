use rocket::{
    http::Status,
    serde::json::{json, Json, Value},
};

use crate::models::Ticket;

// use log::error;
// use reqwest::Error;
// use std::fs;

// use crate::utils::{get_category_data, update_common_fields};
use crate::services::get_issuance_invitation_code;

// /// Fetches an issuance invitation code by making a request with the ticket details.
// async fn get_issuance_invitation_code(ticket: Ticket) -> Result<Value, Error> {
//     let client = reqwest::Client::new();
//     let base_url = "https://sandbox-ssi.extrimian.com/v1/credentialsbbs/wacioob";

//     // Read the base JSON template from a file.
//     let string_local_json_data = fs::read_to_string("json_model/base_ticket_template.json")
//         .expect("Can't read the json file");

//     let mut json_value: Value =
//         serde_json::from_str(&string_local_json_data).expect("JSON was not well-formatted");

//     let category = &ticket.category;
//     let data = get_category_data(category);

//     // Update JSON with ticket and category details.
//     update_common_fields(
//         &mut json_value,
//         &ticket,
//         &ticket.create_new_id(),
//         &ticket.generate_issuance_date(),
//         &ticket.generate_expiration_date(8),
//     );

//     json_value["outputDescriptor"]["display"]["title"]["text"] = json!(data.title);
//     json_value["outputDescriptor"]["display"]["description"]["text"] = json!(data.description);
//     json_value["outputDescriptor"]["styles"]["hero"]["uri"] = json!(data.hero_uri);
//     json_value["issuer"]["styles"]["hero"]["uri"] = json!(data.hero_uri);
//     json_value["outputDescriptor"]["styles"]["background"]["color"] = json!(data.background_color);
//     json_value["issuer"]["styles"]["background"]["color"] = json!(data.background_color);

//     let updated_json_string =
//         serde_json::to_string(&json_value).expect("Failed to convert JSON value to string");

//     // Make a PUT request with the updated JSON.
//     let request_response = client
//         .put(base_url)
//         .header("Content-type", "application/json")
//         .body(updated_json_string.to_owned())
//         .send()
//         .await;

//     match request_response {
//         Ok(response) => {
//             let response = response.error_for_status()?;
//             let response_body: Value = response.json().await?;
//             Ok(response_body)
//         }
//         Err(err) => {
//             error!("Error making request: {:?}", err);
//             Err(err)
//         }
//     }
// }

/// Simple endpoint to check server status.
#[get("/ping")]
pub fn ping() -> &'static str {
    "pong"
}

#[put("/issue-vc", format = "json", data = "<ticket>")]
/// Endpoint to issue a new credential based on ticket information.
pub async fn create_new_vc(ticket: Json<Ticket>) -> Result<Value, Status> {
    match get_issuance_invitation_code(ticket.0).await {
        Ok(json) => Ok(json!(json)),
        Err(_) => Err(Status::InternalServerError),
    }
}
