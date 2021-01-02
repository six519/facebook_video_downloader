use std::env;
use regex::Regex;
use reqwest::header::USER_AGENT;
use select::document::Document;
use select::predicate::Attr;
use std::io;
use std::fs::File;

fn download_link(url: &str, file_name: &str) {
    println!("Downloading {}...", url);
    let mut resp = reqwest::blocking::get(url).expect("request failed");
    let mut out = File::create(format!("{}", file_name)).expect("failed to create file");
    io::copy(&mut resp, &mut out).expect("failed to copy content");
    println!("Done...");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        let url = args.remove(1);
        let re = Regex::new(r"^https://www.facebook.com/.*").unwrap();

        if re.is_match(&url) {
            let client = reqwest::blocking::Client::new();
            let res = client.get(&url)
                .header(USER_AGENT, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/87.0.4280.88 Safari/537.36")
                .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9")
                .header("Accept-Language", "en-US,en;q=0.9")
                .header("Cookie", "dpr=2; m_pixel_ratio=2; locale=en_US; wd=1675x436")
                .send()?
                .text()?;

            Document::from_read(res.as_bytes())
            .unwrap()
            .find(Attr("property", "og:video:url"))
            .filter_map(|n| n.attr("content"))
            .for_each(|x| download_link(x, "video.mp4"));
        } else {
            println!("Invalid URL.");
        }
        
    } else {
        println!("Invalid URL.");
    }

    Ok(())
}