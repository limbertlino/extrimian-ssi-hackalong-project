#[macro_use]
extern crate rocket;
use rocket::serde::json::{json, Value};
use rocket::tokio;
use teloxide::{dispatching::dialogue::InMemStorage, prelude::*};

mod api;
mod bot;
mod models;
mod services;
mod utils;

use crate::api::{create_new_vc, ping};
use crate::bot::{receive_category, receive_name, start};
use crate::models::State;

#[tokio::main]
async fn main() {
    let bot_task = tokio::spawn(async {
        run_bot().await;
    });

    let rocket_task = rocket().launch();

    tokio::select! {
        _ = bot_task => {
            println!("Bot task completed")
        }

        _ = rocket_task => {
            println!("Rocket task completed")
        }
    }
}

async fn run_bot() {
    pretty_env_logger::init();
    log::info!("Starting bot...");

    let bot = Bot::from_env();

    Dispatcher::builder(
        bot,
        dptree::entry()
            .branch(
                Update::filter_message()
                    .enter_dialogue::<Message, InMemStorage<State>, State>()
                    .branch(dptree::case![State::Start].endpoint(start))
                    .branch(dptree::case![State::ReceiveName].endpoint(receive_name)),
            )
            .branch(
                Update::filter_callback_query()
                    .enter_dialogue::<CallbackQuery, InMemStorage<State>, State>()
                    .branch(
                        dptree::case![State::ReceiveCategory { name }].endpoint(receive_category),
                    ),
            ),
    )
    .dependencies(dptree::deps![InMemStorage::<State>::new()])
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;
}

fn rocket() -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .mount("/", routes![ping, create_new_vc])
        .register(
            "/",
            catchers![
                handle_not_found,
                handle_just_500,
                handle_unproccessable_entity
            ],
        )
}

#[catch(404)]
/// Handler for 404 Not Found errors.
fn handle_not_found() -> Value {
    json!({ "error": 404, "message": "Resource not found" }
    )
}

#[catch(500)]
/// Handler for 500 Internal Server errors.
fn handle_just_500() -> Value {
    json!({ "error": 500, "message": "Internal server error" }
    )
}

#[catch(422)]
/// Handler for 422 Unprocessable Entity errors (validation issues).
fn handle_unproccessable_entity() -> Value {
    json!({ "error": 422, "message": "Unprocessable entity: Validation failed" }
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Category, Ticket};
    use rocket::http::{ContentType, Status};
    use rocket::local::blocking::Client;
    use serde_json::Value;
    use uuid::Uuid;

    fn test_issue_vc_ticket(category: Category) {
        let client = Client::tracked(rocket()).expect("Valid rocket instance");

        let ticket = Ticket {
            name: "alice".to_string(),
            category,
        };

        let body = serde_json::to_string(&ticket).expect("Failede to serialize ticket");

        let response = client
            .put("/issue-vc")
            .header(ContentType::JSON)
            .body(body)
            .dispatch();

        assert_eq!(response.status(), Status::Ok);

        let response_body: Value = response.into_json().expect("Response was not Json");
        assert!(
            response_body.get("oobContentData").is_some(),
            "oobContentData missing"
        );
        assert!(
            response_body.get("invitationId").is_some(),
            "invitationId missing"
        );

        let invitation_id = response_body["invitationId"].as_str().unwrap();
        assert!(
            Uuid::parse_str(invitation_id).is_ok(),
            "invitationId is not a valid UUID"
        );

        let oob_content = response_body["oobContentData"].as_str().unwrap();
        assert!(
            oob_content.starts_with("didcomm://?_oob="),
            "oobContentData does not have the expected format"
        )
    }

    #[test]
    fn test_issue_vc_standard_ticket() {
        test_issue_vc_ticket(Category::Standard)
    }

    #[test]
    fn test_issue_vc_vip_ticket() {
        test_issue_vc_ticket(Category::Vip)
    }

    #[test]
    fn test_issue_vc_fast_ticket() {
        test_issue_vc_ticket(Category::Fast)
    }

    #[test]
    fn test_issue_vc_extra_ticket() {
        test_issue_vc_ticket(Category::Extra)
    }

    #[test]
    fn test_issue_vc_invalid_data() {
        let client = Client::tracked(rocket()).expect("Valid rocket instance");

        // Casos de datos inválidos
        let cases = vec![
            r#"{"name": "Alice", "category": "InvalidCategory"}"#,
            r#"{"name": "alice", "category": ""}"#,
            r#"{"category": "Standard"}"#,
            r#"{"name": "alice"}"#,
        ];

        for json in cases {
            let response = client
                .put("/issue-vc")
                .header(ContentType::JSON)
                .body(json)
                .dispatch();

            assert_eq!(response.status(), Status::UnprocessableEntity);

            let response_body: Value = response.into_json().expect("Response was not JSON");
            assert_eq!(response_body["error"], 422);
            assert_eq!(
                response_body["message"],
                "Unprocessable entity: Validation failed"
            );
        }
    }
}
