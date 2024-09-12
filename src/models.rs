use chrono::{Duration, Local};
use nanoid::nanoid;
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct ResponseData {
    pub _invitation_id: String,
    pub oob_content_data: String,
}

/// Enum representing different ticket categories.
#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub enum Category {
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

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
    ReceiveName,
    ReceiveCategory {
        name: String,
    },
}

/// Struct representing a ticket with a name and category.
#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Ticket {
    pub name: String,
    pub category: Category,
}

impl Ticket {
    /// Creates a new unique ID for the ticket.
    pub fn create_new_id(&self) -> String {
        nanoid!()
    }

    /// Generates the issuance date of the ticket.
    pub fn generate_issuance_date(&self) -> String {
        let current_date = Local::now();
        let formated_current_date = current_date.format("%Y-%m-%dT%H:%M:%S%:z").to_string();
        formated_current_date
    }

    /// Generates the expiration date based on the given number of hours.
    pub fn generate_expiration_date(&self, hours: i64) -> String {
        let expiration_date = Local::now() + Duration::hours(hours);
        let formated_expiration_date = expiration_date.format("%Y-%m-%dT%H:%M:%S%:z").to_string();
        formated_expiration_date
    }
}

/// Struct containing metadata for each category.
pub struct CategoryData {
    pub title: &'static str,
    pub description: &'static str,
    pub hero_uri: &'static str,
    pub background_color: &'static str,
}
