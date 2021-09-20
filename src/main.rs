use std::{fs, thread};
use env_logger::Env;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use clap::{App, Arg};

#[macro_use]
extern crate log;

fn main(){
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let matches = App::new("static-server")
        .about("static http server")
        .arg(
            Arg::new("PORT")
                .about("port where the server  will run")
                .default_value("8080")
        )
        .arg(
            Arg::new("FOLDER")
                .about("The path o the folder to be served")
                .default_value(".")
        )
        .get_matches();

    let port = matches.value_of("PORT").unwrap();
    let folder =  String::from(matches.value_of("FOLDER").unwrap());

    info!("Starting server at http://127.0.0.1:{}/", port);

    let listener = TcpListener::bind(format!("localhost:{}", port)).unwrap();
    for stream in listener.incoming(){
        let html_folder = folder.clone();
        let stream = stream.unwrap();
        thread::spawn(move || {
            handle_connection(stream, &html_folder)
        });
    }
}

fn handle_connection(mut stream: TcpStream, html_folder: &str){
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let request_data = String::from_utf8_lossy(&buffer[..]);
    let request_data_lines: Vec<&str> = request_data.split('\n').collect();
    let first_line = request_data_lines[0];
    let start: Vec<&str> = first_line.split(" ").collect();
    
    let protocol = "HTTP/1.1"; 
    let method = start[0];
    let path = start[1];
    
    if method != "GET" {
        let response = format!("{} 405 Method Not Allowed\r\n\r\n", protocol);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
        info!("{} {} HTTP/1.1 {}",method, path, "405");
    }

    
    let filename = match path {
        "/" => format!("{}index.html", html_folder),
        _ => {
            let filename = path.replacen("/", "", 1);
            format!("{}/{}.html", html_folder, filename)
        }
    };
    

    let contents = match fs::read_to_string(filename) {
        Ok(filename) => filename,
        Err(_e) => {
            let new_path = path.replacen("/", "", 1);
            fs::read_to_string(format!("{}/{}/index.html",html_folder, new_path)).unwrap_or("Not Found".to_string())
        },
    };
    

    let status_line = match contents.as_str() {
        "Not Found" => "404 NOT FOUND".to_string(),
        _ => "200 OK".to_string(),
    }; 


    let response = format!(
        "{} {}\r\nContent-Length: {}\r\n\r\n{}",
        protocol,
        status_line,
        contents.len(),
        contents
    );
    
    info!("{} {} {} {}",method, path, protocol, status_line);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}
