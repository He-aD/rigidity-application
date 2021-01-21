use actix_web::{web, client, HttpRequest, web::Payload, HttpResponse};
use futures::StreamExt;
use crate::errors::{AppError, AppResult};
use serde_json::{from_slice};
use serde::Deserialize;

pub async fn sns(
    req: HttpRequest,
    mut stream: Payload,
) -> AppResult<HttpResponse> {
    let error = Err(AppError::BadRequest(String::from("x-amz-sns-message-type header is unknown or missing.")));

    if let Some(message_type_header)  = req
        .headers()
        .get(String::from("x-amz-sns-message-type")) {
        if let Ok(message_type) = message_type_header.to_str() {
            let mut body = web::BytesMut::new();
            while let Some(item) = stream.next().await {
                match item {
                    Ok(chunk) => body.extend_from_slice(&chunk),
                    Err(_) => return Err(AppError::BadRequest(String::from("Corrupted body.")))
                }
            }
            match message_type {
                "SubscriptionConfirmation" => {
                    return handle_sns_subscription(body).await
                },
                "Notification" => {
                    return handle_sns_notification(body)
                },
                _ => {
                    return error
                }
            }
        }
    }

    error
}

async fn handle_sns_subscription(
    body: web::BytesMut
) -> AppResult<HttpResponse> {
    #[derive(Deserialize)]
    #[allow(non_snake_case)]
    struct SnsData {
        pub SubscribeURL: String
    }
    if let Ok(obj) = from_slice::<SnsData>(&body) {
        let obj: SnsData = obj;

        let client = client::Client::default();
        match client.get(obj.SubscribeURL).send().await {
            Ok(_) => return Ok(HttpResponse::Ok().finish()),
            Err(err) => {
                return Err(AppError::BadRequest(err.to_string()))
            }
        }
    }

    Err(AppError::BadRequest(String::from("Json body has wrong format.")))    
}

fn handle_sns_notification(
    body: web::BytesMut
) -> AppResult<HttpResponse> {
    #[derive(Deserialize)]
    #[allow(non_snake_case)]
    struct SnsData {
        pub Message: String
    }
    if let Ok(_obj) = from_slice::<SnsData>(&body) {
        

    }

    Err(AppError::BadRequest(String::from("Json body has wrong format.")))   
}