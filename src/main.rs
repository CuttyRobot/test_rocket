use rocket::*;
#[macro_use] extern crate rocket;


#[get("/test/<test>")]
fn test(test: &str) -> String {
    format!("Test result: {}", test)
}

//192.144.13.73

#[launch]
fn rocket() -> _ {
    build().mount("/", routes![test])
}