use rocket::serde::json::json;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, InputFile};
use teloxide::{dispatching::dialogue::InMemStorage, prelude::*};

use crate::models::{ResponseData, State};
use crate::utils::generate_qr;

type MyDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

fn make_keyboard() -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];

    let ticket_types = ["Standard", "Vip", "Fast", "Extra"];

    for tickets in ticket_types.chunks(2) {
        let row = tickets
            .iter()
            .map(|&ticket| InlineKeyboardButton::callback(ticket.to_owned(), ticket.to_owned()))
            .collect(); // se colectan todos estos botones en un vector

        keyboard.push(row);
    }

    InlineKeyboardMarkup::new(keyboard)
}

pub async fn start(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    bot.send_message(
        msg.chat.id,
        "Welcome! Please enter the visitor's full name.",
    )
    .await?;
    dialogue.update(State::ReceiveName).await?;
    Ok(()) // operacion exitosa
}

pub async fn receive_name(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    if let Some(name) = msg.text() {
        bot.send_message(msg.chat.id, "Choose your ticket type:")
            .reply_markup(make_keyboard())
            .await?;

        dialogue
            .update(State::ReceiveCategory {
                name: name.to_string(),
            })
            .await?;
    } else {
        bot.send_message(msg.chat.id, "Please enter a valid name.")
            .await?;
    }

    Ok(())
}

pub async fn receive_category(bot: Bot, dialogue: MyDialogue, q: CallbackQuery) -> HandlerResult {
    if let Some(category) = q.data {
        if let Some(State::ReceiveCategory { name }) = dialogue.get().await? {
            bot.send_message(
                q.from.id,
                format!("Processing ticket for {} ({})", &name, &category),
            )
            .await?;

            // Send data to server
            let client = reqwest::Client::new();
            let base_url = "http://localhost:8000/issue-vc";

            let request_body = json!({"name": name, "category": category});

            let request_response = client
                .put(base_url)
                .header("Content-type", "application/json")
                .json(&request_body)
                .send()
                .await?;

            let response_data: ResponseData = request_response.json().await?;

            let oob_data = response_data.oob_content_data;

            match generate_qr(&oob_data, "oob_ticket_data.png") {
                Ok(_) => println!("Qr image succes generated"),
                Err(e) => eprint!("Error: {}", e),
            }

            let qr_image = InputFile::file("oob_ticket_data.png");
            bot.send_photo(q.from.id, qr_image).await?;

            bot.send_message(q.from.id, format!("Your ticket has been processed."))
                .await?;
            dialogue.exit().await?;
        }
    }
    Ok(())
}
