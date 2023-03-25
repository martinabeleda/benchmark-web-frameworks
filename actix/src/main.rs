use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use sqlx::postgres::PgPool;
use std::env;

// Define the expense structure
#[derive(Debug, Deserialize, Serialize)]
struct Expense {
    id: i32,
    title: String,
    amount: f64,
}

async fn create_expense(pool: web::Data<PgPool>, expense: web::Json<Expense>) -> impl Responder {
    let result = sqlx::query("INSERT INTO expenses (title, amount) VALUES (?, ?)")
        .bind(&expense.title)
        .bind(expense.amount)
        .execute(pool.as_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Created().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn read_expenses(pool: web::Data<PgPool>) -> impl Responder {
    let rows = sqlx::query("SELECT id, title, amount FROM expenses")
        .fetch_all(pool.as_ref())
        .await;

    match rows {
        Ok(expenses) => {
            let expenses: Vec<Expense> = expenses
                .into_iter()
                .map(|row| Expense {
                    id: row.get(0),
                    title: row.get(1),
                    amount: row.get(2),
                })
                .collect();
            HttpResponse::Ok().json(expenses)
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// Entry point
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let pool = PgPool::connect(&db_url).await.expect("Error connecting to the database");

    // Create the expenses table if it doesn't exist
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS expenses (
            id SERIAL PRIMARY KEY,
            title TEXT NOT NULL,
            amount NUMERIC NOT NULL
        )",
    )
    .execute(&pool)
    .await
    .expect("Error creating expenses table");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/expenses", web::post().to(create_expense))
            .route("/expenses", web::get().to(read_expenses))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
