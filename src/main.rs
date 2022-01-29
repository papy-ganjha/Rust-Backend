#[macro_use] extern crate rocket;
use rocket::{Build, Rocket};
use rocket::fs::{NamedFile, relative};
use std::path::{PathBuf, Path};


#[get("/<file..>")]
pub async fn files(file: PathBuf) -> Option<NamedFile> {
    println!("path arg: {}", file.display());
    let mut path = Path::new(relative!("build")).join(file);
    if path.is_dir() {
        path = Path::new("./build").join("index.html");
    }
    let result = NamedFile::open(path).await.ok();
    match result {
        None => NamedFile::open("./build/index.html").await.ok(),
        _ => result

    }
}

#[launch]
fn rocket() -> Rocket<Build>{
    rocket::build().mount("/", routes![files])
}