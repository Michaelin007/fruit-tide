use super::*;
use crate::Fruit;
use sqlx::{query,query_as, PgPool};

pub async fn create(fruit: Fruit, db_pool: &PgPool) -> tide::Result<Fruit>{

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
    .fetch_one(db_pool)
    .await
    .map_err(|e| Error::new(409, e))?;

    
    Ok(row)
}
pub async fn list(db_pool: &PgPool) -> tide::Result<Vec<Fruit>> {

    let rows = query_as!(
        Fruit,
        r#"
        SELECT id, name, color, weight from fruits
        "#
    )
    .fetch_all(db_pool)
    .await
    .map_err(|e| Error::new(409, e))?;

    Ok(rows)
}
pub async fn get(id: Uuid, db_pool: &PgPool) -> tide::Result<Option<Fruit>> {
    let row = query_as!(
        Fruit,
        r#"
    SELECT id, name, color, weight  from fruits
    WHERE id = $1
    "#,
        id
    )
    .fetch_optional(db_pool)
    .await
    .map_err(|e| Error::new(409, e))?;

    Ok(row)
}
pub async fn update(id: Uuid, fruit_update: Fruit, db_pool:PgPool) -> tide::Result<Option<Fruit>> {
   
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
    .await
    .map_err(|e| Error::new(409, e))?;
   

    Ok(row)
}
pub async fn delete(id: Uuid, db_pool: &PgPool) -> tide::Result<Option<()>> {
   
    let row = query!(
        r#"
    delete from fruits
    WHERE id = $1
    returning "id"
    "#,
        id
    )
    .fetch_optional(db_pool)
    .await
    .map_err(|e| Error::new(409, e))?;
    let res = match row {
        None => None,
        Some(_) => Some(()),
    };

    Ok(res)
}