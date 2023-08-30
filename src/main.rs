use std::env;
use reqwest::blocking::Client;
use reqwest::header::HeaderMap;

fn main() {
    // Get the command-line argument (URL)
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: httpdebug <URL>");
        std::process::exit(1);
    }
    let url = &args[1];

    // Create a Reqwest client
    let client = Client::new();

    // Build the request
    let request = client.get(url).build().unwrap();

    // Print the request headers
    println!("<<< Request to: {}", url);
    print_headers(&request.headers());

    // Make the HTTP request
    match client.execute(request) {
        Ok(response) => {
            println!(">>> Response from: {}", url);
            for line in response.text().unwrap().lines() {
                println!("{}", line);
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

fn print_headers(headers: &HeaderMap) {
    for (name, value) in headers.iter() {
        println!("<<< {}: {:?}", name, value);
    }
}
