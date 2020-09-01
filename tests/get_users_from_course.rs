extern crate canvasapi;
use canvasapi::prelude::*;

use dotenv::dotenv;

#[tokio::test]
async fn get_students() {
    dotenv().ok();

    let base_url = std::env::var("CANVAS_BASE_URL").unwrap();
    let canvas_token = std::env::var("CANVAS_ACCESS_TOKEN").unwrap();

    let canvas = CanvasInformation::new(&base_url, &canvas_token);

    let course = Canvas::get_course(13369)
        .fetch(&canvas)
        .await
        .unwrap()
        .inner();

    let _ = course
        .get_users()
        .add_parameter(EnrollmentType::Student)
        .fetch(&canvas)
        .await
        .unwrap()
        .inner();
}

#[tokio::test]
async fn get_courses() {
    dotenv().ok();

    let base_url = std::env::var("CANVAS_BASE_URL").unwrap();
    let canvas_token = std::env::var("CANVAS_ACCESS_TOKEN").unwrap();

    let canvas = CanvasInformation::new(&base_url, &canvas_token);

    let _ = Course::courses().fetch(&canvas).await.unwrap().inner();
}

#[tokio::test]
async fn get_assignments() {
    dotenv().ok();

    let base_url = std::env::var("CANVAS_BASE_URL").unwrap();
    let canvas_token = std::env::var("CANVAS_ACCESS_TOKEN").unwrap();

    let canvas = CanvasInformation::new(&base_url, &canvas_token);

    let course = Canvas::get_course(13312)
        .fetch(&canvas)
        .await
        .unwrap()
        .inner();

    let _ = course
        .get_assignments()
        .fetch(&canvas)
        .await
        .unwrap()
        .inner();
}

#[tokio::test]
async fn get_files() {
    dotenv().ok();

    let base_url = std::env::var("CANVAS_BASE_URL").unwrap();
    let canvas_token = std::env::var("CANVAS_ACCESS_TOKEN").unwrap();

    let canvas = CanvasInformation::new(&base_url, &canvas_token);

    let course = Canvas::get_course(13312)
        .fetch(&canvas)
        .await
        .unwrap()
        .inner();

    let files = course.get_files().fetch(&canvas).await.unwrap().inner();

    files.last().unwrap().download(&canvas, ".").await.unwrap();
}

// #[tokio::test]
// async fn get_assignment_groups() {
// dotenv().ok();

// let base_url = std::env::var("CANVAS_BASE_URL").unwrap();
// let canvas_token = std::env::var("CANVAS_ACCESS_TOKEN").unwrap();

// let canvas = CanvasInformation::new(base_url, canvas_token);

// let course = Canvas::get_course(13369)
// .fetch(&canvas)
// .await
// .unwrap()
// .inner();

// let assignment_groups = course
// .get_assignment_groups()
// .fetch(&canvas)
// .await
// .unwrap()
// .inner();
// }
