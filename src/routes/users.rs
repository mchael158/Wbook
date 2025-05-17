// Arquivo: src/routes/users.rs

use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use uuid::Uuid;
use bcrypt::{hash, verify, DEFAULT_COST};
use crate::models::user::{User, RegisterData, LoginData};
use crate::auth::create_jwt;

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login))
    );
}

async fn register(data: web::Json<RegisterData>, db: web::Data<PgPool>) -> impl Responder {
    let hashed_pwd = hash(&data.password, DEFAULT_COST).unwrap();
    let user = sqlx::query_as!(User,
        "INSERT INTO users (id, username, email, password_hash) VALUES ($1, $2, $3, $4) RETURNING *",
        Uuid::new_v4(),
        data.username,
        data.email,
        hashed_pwd
    )
    .fetch_one(db.get_ref())
    .await;

    match user {
        Ok(u) => HttpResponse::Ok().json(u),
        Err(_) => HttpResponse::InternalServerError().body("Erro ao cadastrar"),
    }
}

async fn login(data: web::Json<LoginData>, db: web::Data<PgPool>) -> impl Responder {
    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1", data.email)
        .fetch_optional(db.get_ref())
        .await;

    match user {
        Ok(Some(u)) => {
            if verify(&data.password, &u.password_hash).unwrap_or(false) {
                let token = create_jwt(u.id);
                HttpResponse::Ok().body(token)
            } else {
                HttpResponse::Unauthorized().body("Senha incorreta")
            }
        }
        _ => HttpResponse::Unauthorized().body("Usuário não encontrado"),
    }
}
