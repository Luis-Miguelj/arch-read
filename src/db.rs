use std::env;

use sqlx::postgres::{PgPoolOptions, PgPool};
use sqlx::FromRow;

#[derive(FromRow, Debug)]
pub struct Usuarios {
  id: i32,
  name: String
}


pub async fn connection() -> Result<PgPool, sqlx::Error> {
  
  let _ = dotenvy::dotenv();

  let database_url = env::var("DATABASE_URL")
  .map_err(|_| sqlx::Error::Configuration("DATABASE_URL não encontrado.".into()))?;

  PgPoolOptions::new()
    .max_connections(5)
    .connect(&database_url)
    .await
}

pub async fn select_users(pool: &PgPool) -> Result<(), sqlx::Error> {
  let usuarios = sqlx::query_as::<_, Usuarios>(
    "SELECT id, name FROM usuarios"
  ).fetch_all(pool).await?;

  for u in &usuarios {
    println!("Usuarios cadastrados:");
    println!("{}: {}", u.id, u.name);
  }

  Ok(())

}

pub async fn access(pool: &PgPool, username: String) -> Result<bool, sqlx::Error> {
  let user = sqlx::query_as::<_, Usuarios>(
    "SELECT * FROM usuarios WHERE name = $1"
  ).bind(&username.to_lowercase()).fetch_optional(pool).await?;

  match user {
      Some(_) => Ok(true),
      None => Ok(false)
  }
}

pub async fn cadastro(pool: &PgPool, username: String) -> Result<bool, sqlx::Error> {


  let valid_user = sqlx::query_as::<_, Usuarios>(
    "SELECT id, name FROM usuarios WHERE name = $1"
  ).bind(&username.trim().to_lowercase()).fetch_optional(pool).await?;

  match valid_user {
      Some(_) => return Ok(false),
      None => {
        sqlx::query(
        "INSERT INTO usuarios (name) VALUES ($1) RETURNING *"
        ).bind(&username.trim().to_lowercase()).execute(pool).await?;

        return Ok(true)
      },
  }

}

// #[tokio::main]
// pub async fn main() -> Result<(), sqlx::Error> {

//   let url_db = "postgres://postgres:2106@localhost:5435/teste";

//   let pool = PgPoolOptions::new()
//     .max_connections(5)
//     .connect(url_db)
//     .await?;

//   println!("Conexão feita com sucesso!");

//   let select = select_users(&pool).await?;

//   println!("{:?}", select);

//   Ok(())
// }


