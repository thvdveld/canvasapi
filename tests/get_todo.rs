extern crate canvasapi;
use canvasapi::prelude::*;

#[cfg(not(feature = "blocking"))]
mod test_async_todo {
    use super::*;

    #[tokio::test]
    async fn get_todo() {
        dotenv::dotenv().ok();

        let base_url = std::env::var("CANVAS_BASE_URL").unwrap();
        let canvas_token = std::env::var("CANVAS_ACCESS_TOKEN").unwrap();

        let canvas = CanvasInformation::new(&base_url, &canvas_token);
        let todo = Canvas::get_todo_items()
            .unwrap()
            .fetch(&canvas)
            .await
            .unwrap()
            .inner();

        println!("{:#?}", todo);
    }
}
