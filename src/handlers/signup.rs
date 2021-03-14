use actix_web::web::{Data, Json};
use actix_web::Responder;
use serde::Deserialize;

use crate::domain::services::dto::user::RegisterUserDto;
use crate::state::State;

#[derive(Debug, Deserialize)]
pub struct SignupPayload {
    pub email: String,
    pub username: String,
    pub name: String,
    pub surname: Option<String>,
    pub password: String,
}

pub async fn signup(state: Data<State>, payload: Json<SignupPayload>) -> impl Responder {
    let payload = RegisterUserDto {
        email: payload.email.clone(),
        username: payload.username.clone(),
        name: payload.name.clone(),
        surname: payload.surname.clone(),
        password: payload.password.clone(),
    };

    match state.services.user_service.register(payload).await {
        Ok(_) => format!("Successful Request"),
        Err(e) => format!("Request failed! {}", e.to_string()),
    }
}
