use actix_web::{get, web::{self, Path, Json}, App, HttpResponse, HttpServer, post, delete, put};

use deadpool_postgres::Pool;
use serde_json::json;

use crate::user::{CreateUserSchema, User};

mod postgres;
mod user;


#[get("/healthchecker")]
async fn health_checker() -> HttpResponse {
    const MESSAGE: &str = "Build Simple CRUD API with Rust, SQLX, Postgres,and Actix Web";

    return HttpResponse::Ok().json(json!({"name":"Jill", "followee": "Jim" , "msg": MESSAGE }))
}


#[get("/users")]
async fn list_users(pool: web::Data<Pool>) -> HttpResponse {
    
    let client = match pool.get().await {
        Ok(client) => client,
        Err(err) => {
            log::debug!("unable to get postgres client: {:?}", err);
            return HttpResponse::InternalServerError().json("unable to get postgres client");
        }
    };

    match user::User::all(&**client).await {
        Ok(list) => HttpResponse::Ok().json(list),
        Err(err) => {
            log::debug!("unable to fetch users: {:?}", err);
            return HttpResponse::InternalServerError().json("unable to fetch users");
        }
    }
}

#[post("/users")]
async fn create_user(pool: web::Data<Pool>, body: Json<CreateUserSchema>) -> HttpResponse {
    
    let person =  CreateUserSchema {
        login: body.login.to_string(),
    };

    let client = match pool.get().await {
        Ok(client) => client,
        Err(err) => {
            log::debug!("unable to get postgres client: {:?}", err);
            return HttpResponse::InternalServerError().json("unable to get postgres client");
        }
    };

    match user::User::create(&**client, person).await {
        Ok(list) => HttpResponse::Created().json(list),
        Err(err) => {
            log::debug!("unable to create users: {:?}", err);
            return HttpResponse::InternalServerError().json("unable to create users");
        }
    }
}


#[put("/users/{id}")]
async fn update_user(pool: web::Data<Pool>, path: Path<i32>,  body: Json<CreateUserSchema>) -> HttpResponse {
    let id: i32 = path.into_inner();
    
    let user = User{
         id: id,
         login: body.login.to_string()
    };
    
    
    let client = match pool.get().await {
        Ok(client) => client,
        Err(err) => {
            log::debug!("unable to get postgres client: {:?}", err);
            return HttpResponse::InternalServerError().json("unable to get postgres client");
        }
    };

    let res =  user::User::update(&**client, user).await;

    match res {
        Ok(list) => HttpResponse::Ok().json(list),
        Err(err) => {
            log::debug!("unable to update users: {:?}", err);
            return HttpResponse::InternalServerError().json("unable to update users");
        }
    }
}


#[delete("/users/{id}")]
async fn delete_user(pool: web::Data<Pool>,path: Path<i32>) -> HttpResponse {
    let id: i32 = path.into_inner();
    let client = match pool.get().await {
        Ok(client) => client,
        Err(err) => {
            log::debug!("unable to get postgres client: {:?}", err);
            return HttpResponse::InternalServerError().json("unable to get postgres client");
        }
    };

    let res =  user::User::delete(&**client, id).await;

    match res {
        Ok(list) => HttpResponse::NoContent().json(list),
        Err(err) => {
            log::debug!("unable to delete users: {:?}", err);
            return HttpResponse::InternalServerError().json("unable to delete users");
        }
    }
}


fn address() -> String {
    std::env::var("ADDRESS").unwrap_or_else(|_| "127.0.0.1:8001".into())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let pg_pool = postgres::create_pool();
    postgres::migrate_up(&pg_pool).await;

    let address = address();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pg_pool.clone()))
            .service(list_users)
            .service(create_user)
            .service(delete_user)
            .service(update_user)
            .service(health_checker)
    })
    .bind(&address)?
    .run()
    .await
}
