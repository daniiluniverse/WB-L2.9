// Задача
// Реализовать утилиту wget с возможностью скачивать сайты целиком.

use reqwest::blocking::{Client};
use reqwest::{Url};
use scraper::{Html, Selector};
use std::fs;
use std::path::{Path};

fn download_resource(client: &Client, url: &Url, output_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    let resource_response = client.get(url.as_str()).send()?.bytes()?;
    let resource_path = Path::new(output_dir).join(url.path().split('/').last().unwrap());
    fs::create_dir_all(resource_path.parent().unwrap())?;
    fs::write(resource_path, resource_response)?;
    Ok(())
}

fn download(url: &str, output_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let response = client.get(url).send()?.text()?;  // Исправленная строка
    let parsed = Html::parse_document(&response);
    let selector = Selector::parse("link[href], script[src], img[src]").unwrap();

    for element in parsed.select(&selector) {
        let resource_url = element.value().attr("href").or_else(|| element.value().attr("src"));

        if let Some(resource_url) = resource_url {
            let full_url = Url::parse(resource_url).unwrap_or_else(|_| {
                Url::parse(url).unwrap().join(resource_url).unwrap()
            });

            if let Err(err) = download_resource(&client, &full_url, output_dir) {
                eprintln!("Не удалось скачать ресурс {}: {}", full_url, err);
            }
        }
    }

    // Сохранение HTML
    let html_path = Path::new(output_dir).join("index.html");
    fs::write(html_path, response)?;
    Ok(())
}



fn main() {
    let url = "https://realty.ya.ru/offer/4452448424922536228/"; // URL for downloading
    let output_dir = "downloaded_site"; // Directory for saving

    match download(url, output_dir) {
        Ok(_) => println!("Download completed!"),
        Err(err) => eprintln!("Error: {}", err),
    }
}
