#[async_std::test]
async fn list_fruits() -> tide::Result<()> {
    use tide::http::{Method, Request, Response, Url};
    let fruit = Fruit {
        name: String::from("test"),
        color:String::from("red"),
        weight: 45

    };
    let mut fruit_store =  HashMap::new();
    fruit_store.insert(fruit.name.clone(), fruit);
    let fruit: Vec<Fruit> = fruit_store.values().cloned().collect();
    let fruit_as_json_string = serde_json::to_string(&fruit)?;
    let state = Arc::new(RwLock::new(fruit_store));
    let app = server(state).await;

    let url= Url::parse("http://127.0.0.1:8080/fruits").unwrap();
    let req = Request::new(Method::Get, url);
    let mut res: Response= app.response(req).await?;
    let v = res.body_string().await?;
    assert_eq!(fruit_as_json_string, v);

    Ok(())


}

#[async_std::test]
async fn create_fruits() -> tide::Result<()> {
    use tide::http::{Method, Request, Response, Url};
    let fruit = Fruit {
        name: String::from("test"),
        color:String::from("red"),
        weight: 45

    };
    let fruit_store =  HashMap::new();
    
    let state = Arc::new(RwLock::new(fruit_store));
    let app = server(state).await;

    let url= Url::parse("http://127.0.0.1:8080/fruits").unwrap();
    let req = Request::new(Method::Post, url);
    req.set_body(serde_json::to_string(&fruit)?);
    let mut res: Response= app.response(req).await?;
    assert_eq!(201, res.status());

    Ok(())


}

use async_std::sync::RwLock;
use dotenv;
use serde_json::json;
use sqlx::postgres::Postgres;
use sqlx::query;
use sqlx::PgPool;
use sqlx::Pool;
use std::collections::HashMap;
use std::sync::Arc;
use tide::http::StatusCode;
use tide::prelude::*;
use tide::Request;
use tide::Response;
use tide::Server;
use tide::with_state;
use tide::Body;

#[derive(Clone, Debug)]
struct State {
    //  db_pool: PgPool,
    fruits: Arc<RwLock<HashMap<String, Fruit>>>,
}
async fn fruits_create(mut req: Reequest<State>) -> tide::Result{
    
}
#[derive(Debug,Clone, Deserialize, Serialize)]
struct Fruit {
    name: String,
    color: String,
    weight: u16,
}
async fn server(fruits_store: Arc<RwLock<HashMap<String,Fruit>>>) -> Server<State>{
    let state = State {
        fruits: fruits_store,
    };
    let mut app = tide::with_state(state);
    //post request
    app.at("/fruits")
        .post(|mut req: Request<State>| async move {
        let body: Fruit = req.body_json().await?;
        let mut fruits = req.state().fruits.write().await;
        fruits.insert(String::from(&body.name), body.clone());
        let mut res = Response::new(201);
        res.set_body(tide::Body::from_json(&body)?);
        Ok(res)
    });
    // get request 
    app.at("/fruits")
        .get(|req: Request<State>| async move {
        let fruits = req.state().fruits.read().await;
        //get all fruits as a vector
        let fruits_vec: Vec<Fruit> = fruits.values().cloned().collect();

        let mut res = Response::new(200);
        res.set_body(Body::from_json(&fruits_vec)?);
        Ok(res)
        });

        app
}

#[async_std::main]
async fn main() -> tide::Result<()> {
  
    tide::log::start();
    let fruits_store = Default::default();
    let app= server(fruits_store).await;
    

    app.listen("127.0.0.1:8080").await?;
    Ok(())
}



#[async_std::test]
async fn list_fruits() -> tide::Result<()> {
    use tide::http::{Method, Request, Response, Url};
    let fruit = Fruit {
        name: String::from("test"),
        color:String::from("red"),
        weight: 45

    };
    let mut fruit_store =  HashMap::new();
    fruit_store.insert(fruit.name.clone(), fruit);
    let fruit: Vec<Fruit> = fruit_store.values().cloned().collect();
    let fruit_as_json_string = serde_json::to_string(&fruit)?;
    let state = Arc::new(RwLock::new(fruit_store));
    let app = server(state).await;

    let url= Url::parse("http://127.0.0.1:8080/fruits").unwrap();
    let req = Request::new(Method::Get, url);
    let mut res: Response= app.respond(req).await?;
    let v = res.body_string().await?;
    assert_eq!(fruit_as_json_string, v);

    Ok(())


}

#[async_std::test]
async fn create_fruits() -> tide::Result<()> {
    use tide::http::{Method, Request, Response, Url};
    let fruit = Fruit {
        name: String::from("test"),
        color:String::from("red"),
        weight: 45

    };
    let fruit_store =  HashMap::new();
    
    let state = Arc::new(RwLock::new(fruit_store));
    let app = server(state).await;

    let url= Url::parse("http://127.0.0.1:8080/fruits").unwrap();
    let mut req = Request::new(Method::Post, url);
    req.set_body(serde_json::to_string(&fruit)?);
    let mut res: Response= app.respond(req).await?;
    assert_eq!(201, res.status());

    Ok(())


}


