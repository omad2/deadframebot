use std::process::Command;

pub fn execute_ffmpeg_cmd(args: Vec<&str>) {
    let mut process = Command::new("ffmpeg")
        .args(args)
        .spawn()
        .expect("Unable to spawn process");

    if let Err(e) = process.wait() {
        println!("Unable to wait for ffmpeg process: {}", e);
    }
}

pub fn parse_filetype(content_type: &str) -> Result<String, &str> {
    match content_type {
        "video/x-matroska" => Ok(String::from("mkv")),
        "video/x-msvideo" => Ok(String::from("avi")),
        "video/mp4" => Ok(String::from("mp4")),
        "video/quicktime" => Ok(String::from("mov")),
        _ => Err("The filetype provided is not supported."),
    }
}

pub fn check_filetype(file_type: &str) -> Result<(), &str> {
    match file_type {
        "mkv" => Ok(()),
        "avi" => Ok(()),
        "mp4" => Ok(()),
        "mov" => Ok(()),
        _ => Err("The filetype provided is not supported."),
    }
}
