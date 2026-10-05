use ::std::process::Command;
use serde::Deserialize;
use std::os::unix::fs::symlink;
use std::{format, println};

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
            std::fs::write(&filename, bytes).expect("failed to save package");

            println!("Downloaded!");

            let home = std::env::var("HOME").expect("Could not determine home directory");

            let install_dir = format!(
                "{}/.local/share/izac/packages/{}/{}",
                home, package.name, package.version
            );

            std::fs::create_dir_all(&install_dir).expect("Failed to create install directory");

            let status = Command::new("tar")
                .args([
                    "--zstd",
                    "-xf",
                    filename.as_str(),
                    "-C",
                    install_dir.as_str(),
                ])
                .status()
                .expect("Failed to run tar");

            if !status.success() {
                println!("Failed to extract package");
                return;
            }

            let binary = format!("{}/bin/{}", install_dir, package.name);
            let link = format!("{}/.local/bin/{}", home, package.name);

            std::fs::create_dir_all(format!("{}/.local/bin", home))
                .expect("Failed to create ~/.local/bin");

            println!("Binary: {}", binary);
            println!("Link: {}", link);

            symlink(&binary, &link).expect("Failed to create symlink");
        }
        None => {
            println!("Package not found")
        }
    }
}
