use rocket;

mod models;
mod hochzeiger;
mod util;
mod api;

#[rocket::main]
async fn main() -> anyhow::Result<()> {

    let _ = rocket::build().mount("/", rocket::routes![api::get_metrics]).launch().await?;

    Ok(())
}
