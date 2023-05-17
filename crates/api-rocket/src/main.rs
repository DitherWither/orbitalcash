use rocket::launch;
use sqlx::postgres::PgPoolOptions;

mod guards;
mod routes;

#[launch]
async fn rocket() -> _ {
    // get the database url from the environment
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL to be set");

    // create the database connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("to create pool");

    // Create the application service
    let app_service = db::services::ApplicationService::new(pool)
        .await
        .expect("to create app service");

    rocket::build()
        .mount("/api/users", routes::users::get_routes())
        .mount("/api/auth", routes::auth::get_routes())
        .mount("/api/expenses", routes::expenses::get_routes())
        .mount("/api/tags", routes::tags::get_routes())
        .manage(app_service)
}
