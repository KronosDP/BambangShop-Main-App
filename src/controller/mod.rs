pub mod notification;
pub mod product;
use rocket::fairing::AdHoc;

pub fn route_stage() -> AdHoc {
    return AdHoc::on_ignite("Initializing controller routes...", |rocket| async move {
        rocket
            .mount(
                "/product",
                routes![
                    product::create,
                    product::list,
                    product::list,
                    product::read,
                    product::delete
                ],
            )
            .mount("/notification", routes![]);
    });
}
