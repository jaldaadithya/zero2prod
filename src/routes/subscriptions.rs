use actix_web::{Responder, web, HttpResponse};
use sqlx::{ MySqlPool};
use tracing::Subscriber;
use unicode_segmentation::UnicodeSegmentation;

use crate::domain::{NewSubscriber, SubscriberName};


#[tracing::instrument(
    name="Adding a new subscriber",
    skip(form,connection),
    fields(
        subscriber_email=%form.email,
        subscriber_name=%form.name
    )
)]
pub async fn subscribe(form: web::Form<FormData>, connection: web::Data<MySqlPool>) -> impl Responder {

    let name = match SubscriberName::parse(form.0.name) {
        Ok(name) => name,
        Err(_) => return HttpResponse::BadRequest().finish()
    };
    let subscriber = NewSubscriber{email: form.0.email,name};
    match insert_subscriber(&connection, &subscriber).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish() 
    }
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database", skip(subscriber, connection)
)]
async fn insert_subscriber(connection: &MySqlPool, subscriber: &NewSubscriber) -> Result<(), sqlx::Error>{
    sqlx::query!( r#"
    INSERT INTO subscriptions (email, name)
    VALUES (?, ?)
"#,
subscriber.email, subscriber.name.as_ref()
    )
    .execute(connection)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e); 
        e
    })?;
    Ok(())
}

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String
}