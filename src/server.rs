use std::io::Result;

use actix_web::{get, post, web::Json, App, HttpResponse, HttpServer, Responder};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub name: String,
    pub id: u32,
    url: String,
}

impl User {
    pub fn new(name: &str, id: u32, url: &str) -> Self {
        User {
            name: name.to_string(),
            id,
            url: url.to_string(),
        }
    }
}

#[post("/connect")]
async fn user_post(mut user: Json<User>) -> impl Responder {
    user.id += 1;
    user
}

// ステータス200でレスポンス
#[get("/check")]
async fn check() -> impl Responder {
    HttpResponse::Ok()
}

// ローカルホストサーバーをつくる
#[actix_web::main]
pub async fn server() -> Result<()> {
    HttpServer::new(|| App::new().service(user_post).service(check))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

// 構造体を指定したURLにPostしてレスポンスを返す
#[actix_web::main]
pub async fn post<T: DeserializeOwned + Serialize>(url: &str, data: T) -> T {
    let client = reqwest::Client::new();

    let res = client
        .post(url)
        .json(&data)
        .send()
        .await
        .expect("post error");

    res.json::<T>().await.expect("to json error")
}

// サーバーが動作しているかチェクする
#[actix_web::main]
pub async fn server_check(url: &str) -> bool {
    if let Err(_) = reqwest::get(url).await {
        return false;
    }

    true
}
