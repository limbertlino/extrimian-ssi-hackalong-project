#[macro_use]
extern crate rocket;

use reqwest::Error;
use rocket::{
    http::Status,
    serde::json::{json, Value},
};

async fn get_issuance_invitation_code() -> Result<Value, Error> {
    let client = reqwest::Client::new();
    let base_url = "https://sandbox-ssi.extrimian.com/v1/credentialsbbs/wacioob";

    let json_data = r##"{
  "did": "did:quarkid:EiD-CWTXMNUwg1Yp-k2AviPN17INFEnwG4cMA7BS4yLHxw",
  "oneTimeUse": false,
  "vc": {
    "@context": [
      "https://w3id.org/security/bbs/v1",
      "https://www.w3.org/2018/credentials/v1",
      "https://limbertlino.github.io/schemas/passport.json"
    ],
    "id": "17253638365188",
    "type": ["VerifiableCredential", "TuristCredential"],
    "issuer": {
      "id": "did:quarkid:EiD-CWTXMNUwg1Yp-k2AviPN17INFEnwG4cMA7BS4yL345",
      "name": "Club tech"
    },
    "issuanceDate": "Tue Sep 03 2024 09:02:03 GMT-0400 (Atlantic Standard Time)",
    "expirationDate": "2025-08-31",
    "credentialSubject": {
      "name": "Limbert Lino Mattos",
      "passport": "B-43214583",
      "period": "2024-08-31 - 2025-08-31",
      "gender": "female"
    }
  },
  "outputDescriptor": {
    "id": "turist-output",
    "schema": "turist-output",
    "display": {
      "title": {
        "text": "Club Tech"
      },
      "subtitle": {
        "text": "Credencial de socio"
      },
      "description": {
        "text": "Con tu credencial de tech puedes obtener descuentos y acceder a nuestr"
      },
      "properties": [
        {
          "label": "Nombre",
          "path": ["$.credentialSubject.name"],
          "schema": {
            "type": "string"
          },
          "fallback": "Name"
        },
        {
          "label": "Número de pasaporte",
          "path": ["$.credentialSubject.passport"],
          "schema": {
            "type": "string"
          },
          "fallback": "Passport"
        },
        {
          "label": "Período de estadía",
          "path": ["$.credentialSubject.period"],
          "schema": {
            "type": "string"
          },
          "fallback": "Period"
        },
        {
          "label": "Genero",
          "path": ["$.credentialSubject.gender"],
          "schema": {
            "type": "string"
          },
          "fallback": "Genero"
        }
      ]
    },

    "styles": {
      "thumbnail": {
        "uri": "https://cdn.dribbble.com/users/2068059/screenshots/4456420/tech_logo.png",
        "alt": "Logo"
      },
      "hero": {
        "uri": "https://i.ibb.co/bHF3g7m/Credencial-Hack-Along-1.png",
        "alt": "Background"
      },
      "background": {
        "color": "#FFFFFF"
      },
      "text": {
        "color": "#2B2B2D"
      }
    }
  },
  "issuer": {
    "name": "Club tech de Limbert",
    "styles": {
      "thumbnail": {
        "uri": "https://cdn.dribbble.com/users/2068059/screenshots/4456420/tech_logo.png",
        "alt": "Logo"
      },
      "hero": {
        "uri": "https://i.ibb.co/bHF3g7m/Credencial-Hack-Along-1.png",
        "alt": "Background"
      },
      "background": {
        "color": "#FFFFFF"
      },
      "text": {
        "color": "#2B2B2D"
      }
    }
  }
}"##;

    let request_response = client
        .put(base_url)
        .header("Content-type", "application/json")
        .body(json_data.to_owned())
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

#[put("/issue-vc", format = "json")]
async fn create_new_vc() -> Result<Value, Status> {
    match get_issuance_invitation_code().await {
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
