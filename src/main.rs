use std::{print, println};

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
    let contents = std::fs::read_to_string("/home/izac/Dev/pkgmngr/index.json")
        .expect("Failed to load index.json");
    let repo: Repository =
        serde_json::from_str(&contents).expect("Failed to Deserialize Repository");

    repo
}

fn list(repo: &Repository) {
    // print!("{:#?}", repo)
    for package in &repo.packages {
        println!(
            "{} {} - {}",
            package.name, package.version, package.description
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
            )
        }
        None => {
            println!("Package not found")
        }
    }
}
