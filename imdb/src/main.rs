use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use reqwest::{Client, Response};
use reqwest::Url;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::io::Read;

// load function from lib.rs
use imdb::{downloadFile, deleteFile, parseFile};

// A hello world handler
#[get("/")]
async fn index() -> impl Responder {
    println!("Hello!");
    HttpResponse::Ok().body("Hello world!")
}

// hello
#[get("/hello/{name}")]
async fn hello(name: web::Path<String>) -> impl Responder {
    println!("Hello {}!", name);
    HttpResponse::Ok().body(format!("Hello {}!", name))
}

// download file from url
#[get("/download")]
async fn get_imdb_data() -> impl Responder {
    // download
    let url = "https://datasets.imdbws.com/title.basics.tsv.gz";
    let name = "data/title.basics.tsv.gz";
    downloadFile(url, name).await;

    // parse
    parseFile(name);

    // get the first line
    let mut file = File::open("data/title.basics.tsv").unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    let mut lines = s.lines();
    let first_line = lines.next().unwrap();
    println!("{}", first_line);


    HttpResponse::Ok().body("Downloaded file from url")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(hello)
            .service(get_imdb_data)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}