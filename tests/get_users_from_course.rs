extern crate canvasapi;
use canvasapi::prelude::*;
use dotenv::dotenv;

#[cfg(feature = "blocking")]
mod tests_blocking {
    use super::*;

    #[test]
    fn get_students() {
        dotenv().ok();

        let base_url = std::env::var("CANVAS_BASE_URL").unwrap();
        let canvas_token = std::env::var("CANVAS_ACCESS_TOKEN").unwrap();

        let canvas = CanvasInformation::new(&base_url, &canvas_token);

        let course = Canvas::get_course(13369)
            .unwrap()
            .fetch(&canvas)
            .unwrap()
            .inner();

        let users = course
            .get_users()
            .unwrap()
            .add_parameter(EnrollmentType::Student)
            .fetch(&canvas)
            .unwrap()
            .inner();

        println!("{:#?}", users);
    }
}

#[cfg(not(feature = "blocking"))]
mod tests_async {
    use super::*;

    #[tokio::test]
    async fn get_students() {
        dotenv().ok();

        let base_url = std::env::var("CANVAS_BASE_URL").unwrap();
        let canvas_token = std::env::var("CANVAS_ACCESS_TOKEN").unwrap();

        let canvas = CanvasInformation::new(&base_url, &canvas_token);

        let course = Canvas::get_course(13369)
            .unwrap()
            .fetch(&canvas)
            .await
            .unwrap()
            .inner();

        let users = course
            .get_users()
            .unwrap()
            .add_parameter(EnrollmentType::Student)
            .fetch(&canvas)
            .await
            .unwrap()
            .inner();

        println!("{:#?}", users);
    }

    #[tokio::test]
    async fn get_courses() {
        dotenv().ok();

        let base_url = std::env::var("CANVAS_BASE_URL").unwrap();
        let canvas_token = std::env::var("CANVAS_ACCESS_TOKEN").unwrap();

        let canvas = CanvasInformation::new(&base_url, &canvas_token);

        let _ = Course::courses()
            .unwrap()
            .fetch(&canvas)
            .await
            .unwrap()
            .inner();
    }

    #[tokio::test]
    async fn get_assignments() {
        dotenv().ok();

        let base_url = std::env::var("CANVAS_BASE_URL").unwrap();
        let canvas_token = std::env::var("CANVAS_ACCESS_TOKEN").unwrap();

        let canvas = CanvasInformation::new(&base_url, &canvas_token);

        let course = Canvas::get_course(13312)
            .unwrap()
            .fetch(&canvas)
            .await
            .unwrap()
            .inner();

        let _ = course
            .get_assignments()
            .unwrap()
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
            .unwrap()
            .fetch(&canvas)
            .await
            .unwrap()
            .inner();

        let files = course
            .get_files()
            .unwrap()
            .fetch(&canvas)
            .await
            .unwrap()
            .inner();

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

    #[tokio::test]
    async fn get_conv_unread_count() {
        dotenv().ok();

        let base_url = std::env::var("CANVAS_BASE_URL").unwrap();
        let canvas_token = std::env::var("CANVAS_ACCESS_TOKEN").unwrap();

        let canvas = CanvasInformation::new(&base_url, &canvas_token);

        let count = Canvas::conversations_unread_count()
            .unwrap()
            .fetch(&canvas)
            .await
            .unwrap()
            .inner();

        println!("{}", count);
    }

    #[tokio::test]
    #[cfg(feature = "devel")]
    async fn get_todos() {
        dotenv().ok();

        let base_url = std::env::var("CANVAS_BASE_URL").unwrap();
        let canvas_token = std::env::var("CANVAS_ACCESS_TOKEN").unwrap();

        let canvas = CanvasInformation::new(&base_url, &canvas_token);

        let todos = Canvas::get_todo_items()
            .unwrap()
            .fetch(&canvas)
            .await
            .unwrap()
            .inner();

        println!("{:#?}", todos);
    }
}
