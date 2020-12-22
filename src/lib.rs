pub mod capi;
mod downloader;

pub use crate::downloader::*;

#[cfg(test)]
mod tests {
    use super::downloader;
    use std::sync::Arc;

    use actix_rt;
    use actix_web::{http::header, web, App, HttpResponse, HttpServer};

    use tempfile;
    use tokio;

    async fn serve() -> std::io::Result<()> {
        let server = HttpServer::new(|| {
            App::new()
                .route(
                    "/file",
                    web::get().to(|| HttpResponse::Ok().body("file content")),
                )
                .route(
                    "/foo",
                    web::get().to(|| {
                        HttpResponse::Ok()
                            .header(header::CONTENT_DISPOSITION, "attachment; filename=foo.bar")
                            .body("file content")
                    }),
                )
        })
        .bind("127.0.0.1:8080")?
        .run();

        let _ = tokio::spawn(server);

        Ok(())
    }

    #[actix_rt::test]
    async fn writes_default_filename() {
        let _srv = serve().await;

        let tmpdir = Arc::new(tempfile::tempdir().expect("Could not create temp dir for test"));

        let tmp = tmpdir.clone();
        let file = actix_web::web::block(move || {
            downloader::download("http://127.0.0.1:8080/file", tmp.path())
        })
        .await;

        assert!(file.unwrap() == tmpdir.path().join("out.download"));
    }

    #[actix_rt::test]
    async fn uses_content_disposition() {
        let _srv = serve().await;

        let tmpdir = Arc::new(tempfile::tempdir().expect("Could not create temp dir for test"));

        let tmp = tmpdir.clone();
        let file = actix_web::web::block(move || {
            downloader::download("http://127.0.0.1:8080/foo", tmp.path())
        })
        .await;

        assert!(file.unwrap() == tmpdir.path().join("foo.bar"));
    }
}
