use std::{
    fs,
    io::{Read, Write},
    path::PathBuf,
    str::FromStr,
    time::Instant,
};

use tauri::{AppHandle, Manager};

#[derive(Clone, serde::Serialize)]
struct Progress(usize);

#[derive(Clone, serde::Serialize)]
struct DownloadSize(usize);

#[tauri::command]
pub fn get_download_links() -> Result<String, String> {
    let url = "https://builder.blender.org/download/daily/";

    let request = ureq::get(url)
        .set("User-Agent", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/121.0.0.0 Safari/537.36");

    if let Ok(response) = request.call() {
        let body = response.into_string().unwrap();
        dbg!(&body);
        Ok(body)
    } else {
        return Err("Fetch failed".into());
    }
}

fn create_download_url(url: &str) -> String {
    let mut base_path = String::from_str("https://builder.blender.org/download/daily/").unwrap();
    base_path.push_str(url);

    base_path
}

fn create_path(url: &str) -> Result<PathBuf, String> {
    let base_path = PathBuf::from_str("/home/miguel/blenders/").unwrap();

    let file_path = base_path.join(url);

    Ok(file_path)
}

#[tauri::command]
pub async fn download(app: AppHandle, url: String) -> Result<(), String> {
    let file_path = create_path(&url).expect("Could not generate Filepath");
    let download_url = create_download_url(&url);

    dbg!("downloading from", &download_url);

    let mut file = fs::File::create(file_path).unwrap();

    let r = ureq::get(&download_url)
        .set("User-Agent", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/121.0.0.0 Safari/537.36")
        .call()
        .expect("Could not get Url");

    assert!(r.has("Content-Length"));

    let len: usize = r.header("Content-Length").unwrap().parse().unwrap();

    dbg!("content length", len);

    app.emit_all("content_length", DownloadSize(len)).unwrap();

    
    let mut reader = r.into_reader();

    let mut buffer = [0u8; 100000];

    let mut total_read = 0usize;
    let mut now = Instant::now();
    let mut elapsed;

    while let Ok(n) = reader.read(&mut buffer) {
        if n == 0 {
            if total_read == len {
                println!("Download Finished. Downloaded {total_read}");
                app.emit_all("download_finished", ()).unwrap();
            } else {
                println!("Breaking before finishing: {total_read}/{len}");
            }
            break;
        }

        file.write(&buffer[..n]).unwrap();
        total_read += n;

        elapsed = now.elapsed();

        if elapsed.as_millis() >= 2500 {
            dbg!(total_read);
            app.emit_all("download_progress", Progress(total_read))
                .unwrap();
            now = Instant::now();
        }
    }

    Ok(())
}