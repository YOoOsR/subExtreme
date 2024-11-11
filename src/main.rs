use clap::{Command, Arg};
use reqwest::Client;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;
use futures::stream::{self, StreamExt};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use ctrlc;
use std::process;
use colored::*;


async fn check_subdomain(client: &Client, domain: &str, subdomain: &str) -> Option<String> {
    let url = format!("https://{}.{}", subdomain, domain);
    let res = client.get(&url).timeout(Duration::from_secs(3)).send().await;

    match res {
        Ok(response) => {
            if response.status().is_success() {
                Some(url)
            } else {
                None
            }
        }
        Err(_) => None,
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {

    display_logo();


    let matches = Command::new("subExtreme")
        .version("0.1.0")
        .author("Ahmed Hamdy <info.gentil.academy@gmail.com>")
        .about(">>Brute Force Subdomain Discovery<<")
        .arg(
            Arg::new("wordlist")
                .short('w')
                .long("wordlist")
                .value_name("FILE")
                .help("Path to the subdomain wordlist file")
                .required(true),
        )
        .arg(
            Arg::new("domain")
                .short('d')
                .long("domain")
                .value_name("DOMAIN")
                .help("Target domain (e.g., example.com)")
                .required(true),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("FILE")
                .help("Output file to save found subdomains")
                .required(true),
        )
        .arg(
            Arg::new("concurrency")
                .short('c')
                .long("concurrency")
                .value_name("NUM")
                .help("Number of concurrent requests")
                .default_value("20")
        )
        .get_matches();


    let wordlist_path = matches
        .get_one::<String>("wordlist")
        .map(|s| s.clone())
        .unwrap();

    let domain = matches
        .get_one::<String>("domain")
        .map(|s| s.clone())
        .unwrap();

    let output_path = matches
        .get_one::<String>("output")
        .map(|s| s.clone())
        .unwrap();
        
    let concurrency_limit = matches
        .get_one::<String>("concurrency")
        .map(|s| s.parse::<usize>().unwrap_or(20))
        .unwrap_or(20);

    let subdomains: Vec<String> = read_lines(wordlist_path)?
        .filter_map(|line| line.ok())
        .collect();

    let client = Client::new();
    let output_file = Arc::new(Mutex::new(File::create(output_path)?));
    let valid_subdomains = Arc::new(Mutex::new(Vec::new()));


    let total_subdomains = subdomains.len();
    let requests_done = Arc::new(Mutex::new(0));

    let output_file_clone = Arc::clone(&output_file);
    let valid_subdomains_clone = Arc::clone(&valid_subdomains);


    match ctrlc::set_handler(move || {
        println!("\nProgram interrupted, saving results...");

        let valid_subdomains = valid_subdomains_clone.lock().unwrap();
        let mut output_file = output_file_clone.lock().unwrap();
        for subdomain in valid_subdomains.iter() {
            writeln!(output_file, "{}", subdomain).expect("Failed to write to file");
        }

        process::exit(0);
    }) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error setting up Ctrl-C handler: {}", e);
            return Err(io::Error::new(io::ErrorKind::Other, "Failed to set Ctrl-C handler"));
        }
    }

    let start_time = Instant::now();

    let subdomain_checks = stream::iter(subdomains)
        .map(|subdomain| {
            let client = &client;
            let domain = domain.clone();
            let valid_subdomains = Arc::clone(&valid_subdomains);
            let output_file = Arc::clone(&output_file);
            let requests_done = Arc::clone(&requests_done);
            let total_subdomains = total_subdomains;

            async move {
                match check_subdomain(client, &domain, &subdomain).await {
                    Some(valid_subdomain) => {
                        println!("[+] Found subdomain: {}", valid_subdomain.green());

                        let mut valid_subdomains = valid_subdomains.lock().unwrap();
                        valid_subdomains.push(valid_subdomain.clone());

                        let mut output_file = output_file.lock().unwrap();
                        writeln!(output_file, "{}", valid_subdomain).expect("Failed to write to file");
                    }
                    None => {}
                }

                let mut requests_done = requests_done.lock().unwrap();
                *requests_done += 1;

                let progress = *requests_done;
                let elapsed_time = start_time.elapsed().as_secs();
                let avg_time_per_request = if progress > 0 {
                    elapsed_time as f64 / progress as f64
                } else {
                    0.0
                };

                let remaining_requests = total_subdomains - progress;
                let remaining_time = (remaining_requests as f64 * avg_time_per_request) as u64;

                if progress % 1000 == 0 || progress == total_subdomains {
                    println!(
                        "{}",
                        format!(
                            "Progress: {}/{} | Time Elapsed: {}s | Avg Time per Request: {:.2}s | Estimated Time Left: {}s",
                            progress, total_subdomains, elapsed_time, avg_time_per_request, remaining_time
                        )
                        .blue()
                    );
                }
            }
        })
        .buffer_unordered(concurrency_limit);

    subdomain_checks
        .for_each(|_| async {})
        .await;

    Ok(())
}


fn read_lines<P>(filename: P) -> io::Result<impl Iterator<Item = io::Result<String>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn display_logo() {
    println!(
        "{}",
        r#"
            _     ______      _                           
           | |   |  ____|    | |                          
  ___ _   _| |__ | |__  __  _| |_ _ __ ___ _ __ ___   ___ 
 / __| | | | '_ \|  __| \ \/ / __| '__/ _ \ '_ ` _ \ / _ \
 \__ \ |_| | |_) | |____ >  <| |_| | |  __/ | | | | |  __/
 |___/\__,_|_.__/|______/_/\_\\__|_|  \___|_| |_| |_|\___|                
                                                          
    "#.blue() 
    );

  
    println!(
        "{}",
        "
	Author:  @ahmedhamdy0x
        YouTube: Gentil Security
        Version: 0.1.0
        
        "
            .white()
    );
}
