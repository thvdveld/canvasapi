extern crate canvasapi;
use canvasapi::prelude::*;

use dotenv::dotenv;
use serde_json;

#[actix_rt::test]
async fn get_students() {
    dotenv().ok();

    let base_url = std::env::var("CANVAS_BASE_URL").unwrap();
    let canvas_token = std::env::var("CANVAS_ACCESS_TOKEN").unwrap();

    let canvas = Canvas::new(base_url, canvas_token);
    let course = Course::with_id(&canvas, 13369)
        .fetch()
        .await
        .unwrap()
        .inner()
        .unwrap();

    let users = course
        .get_users(&canvas)
        .add_parameter(EnrollmentType::Student)
        .fetch()
        .await
        .unwrap()
        .inner();

    println!("{}", serde_json::to_string_pretty(&users).unwrap());
    assert!(users.unwrap().len() > 0);
}
