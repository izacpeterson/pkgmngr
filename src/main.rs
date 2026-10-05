use std::{format, println};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Repository {
    packages: Vec<Package>,
}

#[derive(Debug, Deserialize)]
struct Package {
    name: String,
    version: String,
    description: String,
    url: String,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    // println!("{:#?}", args);

    let repo = load_repo();

    if args.len() > 1 {
        let command = args[1].as_str();

        match command {
            "list" => list(&repo),

            "install" => {
                if args.len() > 2 {
                    install(args[2].as_str(), &repo);
                } else {
                    println!("Usage: izac install <package>");
                }
            }

            _ => println!("Unknown command"),
        }
    } else {
        println!("Welcome to Izac's Package Manager");
    }
}

fn load_repo() -> Repository {
    let url = "http://vault:8090/repo/index.json";

    let contents = reqwest::blocking::get(url)
        .expect("Failed to connect to repository")
        .text()
        .expect("Failed to read repository");

    let repo: Repository =
        serde_json::from_str(&contents).expect("Failed to Deserialize Repository");

    repo
}

fn list(repo: &Repository) {
    // print!("{:#?}", repo)
    for package in &repo.packages {
        println!(
            "{} - {} - {} - {}",
            package.name, package.version, package.description, package.url
        );
    }
}

fn install(package_name: &str, repo: &Repository) {
    let selected_package = repo
        .packages
        .iter()
        .find(|package| package.name == package_name);

    match selected_package {
        Some(package) => {
            println!(
                "Found {} - {} - {}",
                package.name, package.version, package.description
            );

            let response = reqwest::blocking::get(&package.url);
            let bytes = response
                .expect("Failed to download package")
                .bytes()
                .expect("Failed to read package");

            let filename = format!("/tmp/{}-{}-x86_64.tar.zst", package.name, package.version);
            std::fs::write(filename, bytes).expect("failed to save package");

            println!("Downloaded!")
        }
        None => {
            println!("Package not found")
        }
    }
}
