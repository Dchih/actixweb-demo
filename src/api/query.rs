use actix_web::{get, web};
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
  username: String,
}

#[get("/user/name")]
pub async fn get_user_name(info: web::Query<Info>) -> String {
  let web::Query(Info { username }) = info;
  format!("welcome {}!", username)
}