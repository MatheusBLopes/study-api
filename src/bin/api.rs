use api::adapter::api;

fn main() {
  match api::server::main() {
    Ok(_) => println!("Server finished"),
    Err(e) => println!("Error: {}", e),
  };
}
