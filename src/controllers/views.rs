use super::*;
use tide::{Request, Response};

pub async fn index(req: Request<State>) -> tide::Result{
   let tera = req.state().tera.clone();
   let db_pool = req.state().db_pool.clone();
   let rows = handlers::fruit::list(&db_pool).await?;

   tera.render_response(
    "index.html",
    &context! {
        "title" => String::from("Tide Basic Crud with Authentication"),
        "fruits" => rows
    },
   )

}

pub async fn new(req: Request<State>) -> tide::Result{
    let tera = req.state().tera.clone();

    tera.render_response(
        "form.html",
        &context! {
            "title" => String::from("Create New Fruit")
        },
    )
}

pub async fn edit(req: Request<State>) -> tide::Result{
  let tera = req.state().tera.clone();
  let db_pool = req.state().db_pool.clone();
  let id: Uuid = Uuid::parse_str(req.param("id")?).unwrap();
  let row = handlers::fruit::get(id, &db_pool).await?;
  let res = match row {
    None => Response::new(404),
    Some(row) => {
        let mut r = Response::new(200);
        let b = tera.render_body(
            "form.html",
            &context! {
                "title"=> String::from("Edit Fruit"),
                "fruit" => row
            },
        )?;
        r.set_body(b);
        r
    }
};
Ok(res)
}



