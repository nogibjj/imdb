use actix_web::{get, web, HttpResponse, Result, Responder};
use reqwest::Url;
use std::fs::File;
use std::io::{BufWriter, Write};

// hello world
#[get("/")]
async fn index() -> impl Responder {
    println!("Hello!");
    HttpResponse::Ok().body(
        "<html>
            <head>
                <title>Hello</title>
            </head>
            <body>
                <h1><center>Hello!</center></h1>
                <p>This is a simple web server for IMDB data.</p>
            </body>
        </html>",
    )
}

// download file from url
#[get("/imdb-data")]
async fn get_imdb_data() -> impl Responder {
    let url = Url::parse("https://datasets.imdbws.com/title.basics.tsv.gz").unwrap();
    let mut resp = reqwest::get(url).await.unwrap();
    let mut out = BufWriter::new(File::create("title.basics.tsv.gz").unwrap());
    while let Some(chunk) = resp.chunk().await.unwrap() {
        out.write_all(&chunk).unwrap();
    }
    HttpResponse::Ok().body("Downloaded file")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    actix_web::HttpServer::new(|| {
        actix_web::App::new()
            .service(index)
            .service(get_imdb_data)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
