use actix_web::{Responder, web, HttpResponse};
use sqlx::{ MySqlPool};

pub async fn subscribe(form: web::Form<FormData>, connection: web::Data<MySqlPool>) -> impl Responder {

    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (email, name) VALUES (?, ?)
        "#,
        form.email,
        form.name
        ).execute(connection.as_ref())
        .await{
            Ok(_) => HttpResponse::Ok().body(format!("Hello {} - {}!", &form.name, &form.email)) ,
            Err(e) => {
                println!("Failed to execute query {}",e);
                HttpResponse::InternalServerError().finish()
            }
        }

    // format!("Hello {} - {}!", &form.name, &form.email)
}

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String
}