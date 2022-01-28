mod error;

use std::env;
use actix_form_data::{Error, Field, Form, Value};
use actix_web::{App, HttpResponse, HttpServer, Responder, post};
use actix_web::middleware::{DefaultHeaders, Logger};
use futures::StreamExt;
use log::debug;
use crate::error::UploadError::{InvalidFormData};

#[post("/")]
pub async fn image_default(
    payload: Value<Vec<u8>>,
) -> Result<impl Responder, actix_web::Error> {
    debug!("{:?}", &payload);

    let file = match payload.map() {
        None => Err(InvalidFormData("Expected Map".to_string())),
        Some(mut map) => {
            let file = match map.remove("file") {
                None => Err(InvalidFormData(
                    "Expected value with \"file\" key".to_string(),
                )),
                Some(file_content) => match file_content.file() {
                    None => Err(InvalidFormData(
                        "Expected \"file\" to be of type file".to_string(),
                    )),
                    Some(file) => Ok(file),
                },
            }?;

            let json_string = match map.remove("json_payload") {
                None => Err(InvalidFormData(
                    "Expected value with \"json_payload\" key".to_string(),
                )),
                Some(json_content) => match json_content.text() {
                    None => Err(InvalidFormData(
                        "Expected \"json_payload\" to be of type text".to_string(),
                    )),
                    Some(json) => Ok(json),
                },
            }?;

            Ok(file)
        }
    }?;

    Ok(HttpResponse::Ok()
        .body(
            file.result,
        )
        .customize()
        .insert_header(actix_web::http::header::ContentType::jpeg()))
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let port = env::var("PORT").unwrap_or("9015".to_string());

    let form = Form::new()
        .field(
            "file",
            Field::file(|_, _, mut stream| async move {
                let mut bytes = Vec::new();
                while let Some(res) = stream.next().await {
                    let clone = res?.clone().to_vec();
                    for byte in clone {
                        bytes.push(byte);
                    }
                }
                Ok(bytes) as Result<Vec<u8>, Error>
            }),
        )
        .field("json_payload", Field::text());

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(DefaultHeaders::default())
            .service(
                actix_web::web::scope("")
                    .wrap(form.clone())
                    .service(image_default),
            )
    })
        .bind(format!("0.0.0.0:{}", port))?
        .run()
        .await?;

    Ok(())
}