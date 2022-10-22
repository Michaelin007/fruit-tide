use super::*;

use tide::{Body, Request, Response};
use crate::handlers;

pub async fn create(mut req: Request<State>) -> tide::Result {
    let fruit: Fruit = req.body_json().await?;
    //let mut fruits = req.state().fruits.write().await;
    let db_pool = req.state().db_pool.clone();
    let row = query_as!(
        Fruit,
        r#"
         INSERT INTO fruits(id,name,color, weight) VALUES($1,$2,$3,$4)
          returning id, name,color, weight
        "#,
        fruit.id,
        fruit.name,
        fruit.color,
        fruit.weight
    )
    .fetch_one(&db_pool)
    .await?;

    let mut res = Response::new(201);
    res.set_body(tide::Body::from_json(&row)?);
    Ok(res)
}
pub async fn list(req: Request<State>) -> tide::Result {
    let db_pool = req.state().db_pool.clone();
    let rows = query_as!(
        Fruit,
        r#"
        SELECT id, name, color, weight from fruits
        "#
    )
    .fetch_all(&db_pool)
    .await?;

    let mut res = Response::new(200);
    res.set_body(Body::from_json(&rows)?);
    Ok(res)
}
pub async fn get(req: Request<State>) -> tide::Result {
    let db_pool = req.state().db_pool.clone();
    let id: Uuid = Uuid::parse_str(req.param("id")?).unwrap();
    let row = query_as!(
        Fruit,
        r#"
    SELECT id, name, color, weight  from fruits
    WHERE id = $1
    "#,
        id
    )
    .fetch_optional(&db_pool)
    .await?;
    let res = match row {
        None => Response::new(404),
        Some(row) => {
            let mut r = Response::new(200);
            r.set_body(Body::from_json(&row)?);
            r
        }
    };

    Ok(res)
}
pub async fn update(mut req: Request<State>) -> tide::Result {
    let fruit_update: Fruit = req.body_json().await?;
    let db_pool = req.state().db_pool.clone();
    let id: Uuid = Uuid::parse_str(req.param("id")?).unwrap();
    let row = query_as!(
        Fruit,
        r#"
    UPDATE fruits SET name =$2, color = $3, weight = $4
    WHERE id = $1
    returning id, name, color, weight

    "#,
        id,
        fruit_update.name,
        fruit_update.color,
        fruit_update.weight
    )
    .fetch_optional(&db_pool)
    .await?;
    let res = match row {
        None => Response::new(404),
        Some(row) => {
            let mut r = Response::new(200);
            r.set_body(Body::from_json(&row)?);
            r
        }
    };

    Ok(res)
}
pub async fn delete(req: Request<State>) -> tide::Result {
    let db_pool = req.state().db_pool.clone();
    let id: Uuid = Uuid::parse_str(req.param("id")?).unwrap();
    let row = query!(
        r#"
    delete from fruits
    WHERE id = $1
    returning "id"
    "#,
        id
    )
    .fetch_optional(&db_pool)
    .await?;
    let res = match row {
        None => Response::new(404),
        Some(_) => Response::new(204),
    };

    Ok(res)
}
