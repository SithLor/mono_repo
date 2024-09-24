use rocket::data::ToByteUnit;
use rocket::{get, launch, post, routes, Build, Data, Rocket};
use std::io::prelude::*;
use std::process::Command;

//https://rocket.rs/guide/v0.5/overview/#overview

//complie the wasm on the file

#[post("/create_function", data = "<data>")]
async fn create_function(data: Data<'_>) -> String {
    let mut file = std::fs::File::create(format!("./src/tmp/{}.rs", "wasm")).unwrap();
    data.open(1.megabytes())
        .into_string()
        .await
        .unwrap()
        .map(|s| file.write_all(s.as_bytes()).unwrap());

    let output = Command::new("rustc")
        .arg(format!("./src/tmp/{}.rs", "wasm"))
        //.arg("target-feature=+crt-static")
        .arg("--target=wasm32-unknown-unknown")
        .arg("--crate-type=cdylib")
        .arg("-o ./src/tmp/wasm.wasm")
        .output()
        .expect("Failed to execute command");
    if output.status.success() {
        //read the file and return the wasm
        let bytes_hex = std::fs::read(format!("./src/tmp/{}.wasm", "wasm")).unwrap();
        format!("0x{}", hex::encode(bytes_hex))
    } else {
        format!("Error: {}", String::from_utf8_lossy(&output.stderr))
    }
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![create_function])
}
