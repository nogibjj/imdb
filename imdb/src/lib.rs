use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use reqwest::{Client, Response};
use reqwest::Url;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::io::Read;

// down load file from url
pub async fn downloadFile(url: &str, name: &str) {
    let url = Url::parse(url).unwrap();
    let mut resp = reqwest::get(url).await.unwrap();
    // save it to data/title.basics.tsv.gz
    let mut out = BufWriter::new(File::create(name).unwrap());
    while let Some(chunk) = resp.chunk().await.unwrap() {
        out.write_all(&chunk).unwrap();
    }
}

// delete file
pub fn deleteFile(name: &str) {
    std::fs::remove_file(name).unwrap();
}

// parse file
pub fn parseFile(name: &str) {
    // parse .gz file
    let file = File::open(name).unwrap();
    let mut decoder = flate2::read::GzDecoder::new(file);
    let mut s = String::new();
    decoder.read_to_string(&mut s).unwrap();

    // save it
    // get the name of the file, which is name without .gz
    let name_tsv = name.trim_end_matches(".gz");
    let mut file = File::create(name_tsv).unwrap();
    file.write_all(s.as_bytes()).unwrap();

    // delete .gz file
    deleteFile(name);
}
