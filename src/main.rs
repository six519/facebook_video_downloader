mod downloader;

use std::env::args;

fn main() {
    let url = args()
        .skip(1)
        .next().expect("Facebook video URL expected.");
    
    downloader::get_video(url);
}