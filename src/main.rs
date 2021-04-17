extern crate rocket;

use rocket::fairing::AdHoc;
use rocket_contrib::serve::{Options, StaticFiles};

fn main() -> std::io::Result<()> {
    rocket::ignite()
        .attach(AdHoc::on_response("google sucks", |_, resp| {
            resp.set_raw_header("Permissions-Policy", "interest-cohort=()");
        }))
        .mount(
            "/",
            StaticFiles::new(std::env::current_dir()?, Options::Index),
        )
        .launch();

    Ok(())
}
