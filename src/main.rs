extern crate rocket;

use rocket_contrib::serve::StaticFiles;

fn main() -> std::io::Result<()> {
    rocket::ignite()
        .mount("/", StaticFiles::from(std::env::current_dir()?))
        .launch();

    Ok(())
}
