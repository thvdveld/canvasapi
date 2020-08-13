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
    // let course = Course::with_id(13369).fetch(&canvas).await.unwrap().inner();

    let course = Course::with_id(13369).fetch(&canvas).await.unwrap().inner();

    let users = course
        .get_users()
        .add_parameter(EnrollmentType::Student)
        .add_parameter(SortOn::SisId)
        .add_parameter(Include::AvatarUrl)
        .fetch(&canvas)
        .await
        .unwrap()
        .inner();

    println!("{}", serde_json::to_string_pretty(&users).unwrap());
    assert!(users.len() > 0);
}

#[actix_rt::test]
async fn get_courses() {
    dotenv().ok();

    let base_url = std::env::var("CANVAS_BASE_URL").unwrap();
    let canvas_token = std::env::var("CANVAS_ACCESS_TOKEN").unwrap();

    let canvas = Canvas::new(base_url, canvas_token);

    let courses = Course::courses().fetch(&canvas).await.unwrap().inner();

    println!("{:#?}", courses);
}

#[actix_rt::test]
async fn get_assignments() {
    dotenv().ok();

    let base_url = std::env::var("CANVAS_BASE_URL").unwrap();
    let canvas_token = std::env::var("CANVAS_ACCESS_TOKEN").unwrap();

    let canvas = Canvas::new(base_url, canvas_token);
    let course = Course::with_id(13312).fetch(&canvas).await.unwrap().inner();

    let assignments = course
        .get_assignments()
        .fetch(&canvas)
        .await
        .unwrap()
        .inner();

    println!("{}", serde_json::to_string_pretty(&assignments).unwrap());
}
