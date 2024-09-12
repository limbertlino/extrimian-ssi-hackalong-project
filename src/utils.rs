use image::Luma;
use qrcode::QrCode;

use crate::models::{Category, CategoryData, Ticket};
extern crate rocket;
use rocket::serde::json::{json, Value};

pub fn generate_qr(data: &str, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let code = QrCode::new(data)?;

    let image = code.render::<Luma<u8>>().build();

    image.save(path)?;
    Ok(())
}

//TODO agregar un subtitulo personalizado para cada tipo
/// Returns metadata for a given ticket category.
pub fn get_category_data(category: &Category) -> CategoryData {
    match category {
        Category::Standard => CategoryData {
            title: "Regular Pass",
            description: "With this credential, you have the following benefits/access:\n- Can access the main attractions\n- Free water at designated points\n- Welcome snack and ice cream\n- Personalized welcome upon entering the park",
            background_color: "#FFFFFF",
            hero_uri: "https://limbertlino.github.io/schemas/images/s.png",
        },
        Category::Vip => CategoryData {
            title: "Vip Pass",
            description: "With this credential, you have the following benefits/access:\n- Access to the park's premium facilities (15 premium + 15 main attractions)\n- Priority entrance to attractions\n- Fast pass for 5 attractions\n- Access to the general food buffet\n- Unlimited soft drinks and water at all points in the park\n- Unlimited photos within the park\n- Access to the park's pools\n- Access to VIP lounge areas\n- 50% discount on fast pass\n- Priority access to the night show and a 35% discount",
            background_color: "#FFFFFF",
            hero_uri: "https://limbertlino.github.io/schemas/images/v.png",
        },
        Category::Fast => CategoryData {
            title: "Fast Pass",
            description: "With this credential, you have the following benefit/access:\n- Fast pass to all attractions",
            background_color: "#FFFFFF",
            hero_uri: "https://limbertlino.github.io/schemas/images/f.png",
        },
        Category::Extra => CategoryData {
            title: "Extra Pass",
            description: "With this credential, you have the following benefits/access:\n- Access to the full food buffet (25% discount on seasonal special meals)\n- Access to the pool in the morning and afternoon\n- Access to the night show\n- Unlimited photos within the park\n- Rental of a locker for valuable items\n- Priority reservation at the restaurant\n- In-park transportation service",
            background_color: "#FFFFFF",
            hero_uri: "https://limbertlino.github.io/schemas/images/e.png",
        },
    }
}

/// Updates common fields in the JSON structure with ticket details.
pub fn update_common_fields(
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
