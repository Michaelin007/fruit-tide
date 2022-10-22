use dotenv;
use uuid::Uuid;

use sqlx::query;
use sqlx::query_as;
use sqlx::PgPool;
use sqlx::Pool;
use tera::Tera;
use tide_tera::prelude::*;
use tide::prelude::*;

use serde::{Deserialize, Serialize};
use tide::with_state;
use tide::Body;
use tide::Request;
use tide::Response;
use tide::Server;
use tide::{Error};
mod controllers;
mod handlers;

use controllers::fruit;
use controllers::views;


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Fruit {
    id: Uuid,
    name: Option<String>,
    color: Option<String>,
    weight: Option<i32>,
}

#[derive(Clone, Debug)]
pub struct State {
    db_pool: PgPool,
    tera: Tera,
}



#[async_std::main]
async fn main(){
    dotenv::dotenv().ok();

    tide::log::start();
    let db_url = std::env::var("DATABASE_URL").expect("Missing DATABASE URL ENV");
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    let db_pool = make_db_pool(&db_url).await;
    //let fruits_store = Default::default();
    let app = server(db_pool).await;
    let mut listener = app
        .bind(format!("127.0.0.1:{}", port))
        .await
        .expect("can't bind port");

        for info in listener.info().iter(){
            println!("Server listening on {}", info);
        }

        listener.accept().await.unwrap();

   
}

pub async fn make_db_pool(db_url: &str) -> PgPool {
    Pool::connect(db_url).await.unwrap()

}
pub async fn server(db_pool: PgPool) -> Server<State> {
     let mut tera = Tera::new("templates/**/*").expect("Error parsing template");
      tera.autoescape_on(vec!["html"]);

      let state = State {
        db_pool,
        tera
      };

      let mut app = tide::with_state(state);

  //view
      app.at("/").get(views::index);
      app.at("/fruits/:id/edit").get(views::edit);
      app.at("/fruits/new").get(views::new);

      //api
      app.at("/fruits").get(fruit::list).post(fruit::create);

      app.at("/fruits/:id")
          .get(fruit::get)
          .put(fruit::update)
          .delete(fruit::delete);
  
     

    app.at("/static")
          .serve_dir("./static/")
          .expect("Invalid static file directory");
  
      app

}

/*
#[async_std::test]
async fn list_fruits() -> tide::Result<()> {
    dotenv::dotenv().ok();
    use tide::http::{Method, Request, Response, Url};

    let fruit = Fruit {
        id: Uuid::new_v4(),
        name: Some(String::from("test")),
        color: Some(String::from("red")),
        weight: Some(45),
    };
    let db_pool = make_db_pool().await;

    query!(
        r#"
        INSERT INTO fruits (id, name, weight, color) VALUES
        ($1, $2, $3, $4) returning id, name, weight, color
        "#,
        fruit.id,
        fruit.name,
        fruit.weight,
        fruit.color
    )
    .fetch_one(&db_pool)
    .await?;

    let app = server(db_pool).await;
    let url = Url::parse("http://127.0.0.1:8080/fruits").unwrap();
    let req = Request::new(Method::Get, url);
    let mut res: Response = app.respond(req).await?;

    assert_eq!(200, res.status());

    Ok(())
}

#[async_std::test]
async fn create_fruits() -> tide::Result<()> {
    dotenv::dotenv().ok();
    use tide::http::{Method, Request, Response, Url};

    let fruit = Fruit {
        id: Uuid::new_v4(),
        name: Some(String::from("test")),
        weight: Some(45),
        color: Some(String::from("red")),
    };
    let db_pool = make_db_pool().await;
    let app = server(db_pool).await;

    let url = Url::parse("http://127.0.0.1:8080/fruits").unwrap();
    let mut req = Request::new(Method::Post, url);
    req.set_body(serde_json::to_string(&fruit)?);
    let mut res: Response = app.respond(req).await?;
    assert_eq!(201, res.status());

    Ok(())
}
*/