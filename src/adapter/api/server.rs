use std::net::{IpAddr,Ipv4Addr};

#[get("/")]
fn hello() -> &'static str {
    "<h1>Hello World</h1>"
}

#[rocket::main]
pub async fn main() -> Result<(), rocket::Error> {
    println!("Starting server...");

    let server_config = rocket::Config::figment()
        .merge(("address", IpAddr::V4(Ipv4Addr::new(0,0,0,0))))
        .merge(("port", 3000));

    rocket::build()
    .configure(server_config)
    .mount("/", routes![hello])
    .launch()
    .await?;


    Ok(())
}