#[macro_use] extern crate rocket;
use rocket::{Build, Rocket};
use rocket::fs::{NamedFile, relative};
use std::path::{PathBuf, Path};

// #[get("/")]
// fn index() -> &'static str {
//     "Hello, world!"
// }

#[get("/<file..>")]
pub async fn files(file: PathBuf) -> Option<NamedFile> {
    println!("path arg: {}", file.display());
    let mut path = Path::new(relative!("build")).join(file);
    if path.is_dir() {
        path = Path::new("./build").join("index.html");
    }
    NamedFile::open(path).await.ok()
}
#[get("/")]
pub async fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("./build/index.html")).await.ok()
}

#[launch]
fn rocket() -> Rocket<Build>{
    rocket::build().mount("/", routes![index, files])
}