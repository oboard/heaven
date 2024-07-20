mod web;

use notify::RecommendedWatcher;
use notify::{RecursiveMode, Watcher};
use std::path::Path;
use std::process::Command;
use std::sync::mpsc::{channel, Sender};
use std::time::Duration;
use std::{fs, io};

use anyhow::{anyhow, Context, Result};
use clap::{Parser, Subcommand};
use dialoguer::Input;
// use serde_json::json;
use colored::*;
use web::start_web_server;

/// CLI for project management
#[derive(Parser)]
#[command(name = "heaven")]
#[command(version = "0.0.1")]
#[command(author = "oboard")]
#[command(about = "Heaven is a SDK for multi-platform applications with moonbit.")]
struct Opts {
    #[command(subcommand)]
    subcmd: SubCommand,
}

#[derive(Subcommand)]
enum SubCommand {
    New,
    Run(Build),
    Clean,
    Upgrade,
    Build(Build),
}

#[derive(Parser)]
struct Build {
    #[arg(short, long)]
    target: Option<String>,
    #[arg(long, default_value = "src")]
    source_dir: String,
    #[arg(long, default_value = "target/app/{target}")]
    target_dir: String,
}

#[actix_web::main]
async fn main() -> Result<()> {
    let opts: Opts = Opts::parse();

    match opts.subcmd {
        SubCommand::New => new_project()?,
        SubCommand::Run(build) => run_project(build).await?,
        SubCommand::Clean => println!("..."),
        SubCommand::Upgrade => upgrade_heaven()?,
        SubCommand::Build(build) => build_project(build)?,
    }

    Ok(())
}

async fn run_project(build: Build) -> Result<()> {
    build_project(build).context("Failed to build project")?;
    // 调用 start 函数来启动服务器

    std::thread::spawn(move || {
        let _ = monitor_changes();
    });

    if let Err(e) = start_web_server().await {
        eprintln!("{} {}", "Error starting server:".red().bold(), e);
    }

    fn monitor_changes() -> Result<()> {
        let (_tx, _rx): (Sender<notify::Event>, _) = channel();

        // let (tx, _rx) = channel();
        let mut watcher = RecommendedWatcher::new(
            move |res: Result<notify::Event, notify::Error>| {
                match res {
                    Ok(event) => {
                        // 这里可以添加重新启动服务器的逻辑
                        // 显示有多少个文件改变， 如果只有一个文件的话，打印文件名

                        if event.paths.len() == 1 {
                            println!(
                                "File changed: {}",
                                event.paths[0].display().to_string().blue()
                            );
                        } else {
                            println!("{} files changed", event.paths.len().to_string().blue());
                        }

                        // 重新编译wasm到/target/app/web/assets/app.wasm

                        Command::new("moon")
                            .args(&[
                                "build",
                                "--target",
                                "wasm",
                                "--target-dir",
                                "target/app/web/assets/",
                            ])
                            .status()
                            .unwrap();
                    }
                    Err(e) => eprintln!("Error: {:?}", e),
                }
            },
            notify::Config::default(),
        )?;
        watcher.watch(Path::new("main"), RecursiveMode::Recursive)?;

        println!(
            "{}",
            "Monitoring for changes in the main directory..."
                .magenta()
                .bold()
        );

        loop {
            // 在这里可以做一些有用的事情，比如检查队列，避免 CPU 占用过高
            std::thread::sleep(Duration::from_secs(1));
        }
    }

    Ok(())
}

fn new_project() -> Result<()> {
    let name: String = Input::new()
        .with_prompt("Project name")
        .default("heaven-app".into())
        .validate_with(|input: &String| -> Result<(), &str> {
            if !input.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
                return Err("Invalid project name, only letters, numbers, underscores and dashes are allowed.");
            }

            if Path::new(input).exists() {
                return Err("Project already exists.");
            }

            Ok(())
        })
        .interact_text()?;

    let username: String = Input::new()
        .with_prompt("Your username")
        .default("username".into())
        .interact_text()?;

    Command::new("moon")
        .args(&["new", "--name", &name, "--path", &name, "--user", &username])
        .status()
        .context("Failed to execute moon command")?;

    let web_template_path = Path::new("resources/templates/web");

    copy_dir_all(web_template_path, Path::new(&name).join("web").as_path())
        .context("Failed to copy web template")?;

    replace_template_variables(&format!("{}/web/index.html", name), &name)?;
    replace_template_variables(&format!("{}/web/manifest.json", name), &name)?;

    println!("{}", format!("Project {} created!", name).green());

    Ok(())
}

fn copy_dir_all(src: &Path, dst: &Path) -> Result<(), io::Error> {
    // 如果目标目录不存在，则创建它
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }

    // 遍历源目录中的所有条目
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        // 如果是目录，则递归调用复制函数
        if src_path.is_dir() {
            copy_dir_all(&src_path, &dst_path)?;
        } else {
            // 如果是文件，则复制文件
            fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}

fn upgrade_heaven() -> Result<()> {
    Command::new("npm")
        .args(&["install", "-g", "moonbit-heaven"])
        .status()
        .context("Failed to execute npm command")?;
    Ok(())
}

fn build_project(build: Build) -> Result<()> {
    let target = if let Some(target) = build.target {
        target
    } else {
        let available_targets = scan_targets()?;
        if available_targets.is_empty() {
            return Err(anyhow!("No available targets found."));
        }
        if available_targets.len() == 1 {
            available_targets[0].clone()
        } else {
            let target: String = Input::new()
                .with_prompt("Select output target")
                .default(available_targets[0].clone())
                .interact_text()?;
            target
        }
    };

    match target.as_str() {
        "web" => {
            println!("Building for web...");
            Command::new("moon")
                .args(&["build", "--target", "wasm"])
                .status()
                .context("Failed to execute moon build command")?;

            let target_dir = build.target_dir.replace("{target}", "web");

            fs::create_dir_all(&format!("{}/assets/lib", target_dir))
                .context("Failed to create target directory")?;

            copy_dir_all(Path::new("web"), Path::new(&format!("{}/", target_dir)))
                .context("Failed to copy web files")?;

            copy_dir_all(
                Path::new("resources/templates/web/assets/lib"),
                Path::new("web/assets/lib"),
            )
            .context("Failed to copy lib files")?;

            fs::copy(
                "target/wasm/release/build/main/main.wasm",
                format!("{}/assets/app.wasm", target_dir),
            )
            .context("Failed to copy wasm file")?;

            println!("{}", "Build completed!".green());
        }
        _ => {
            println!("Unsupported target.");
        }
    }

    Ok(())
}

fn replace_template_variables(file_path: &str, project_name: &str) -> Result<()> {
    let file_content = fs::read_to_string(file_path)
        .with_context(|| format!("Failed to read file {}", file_path))?;
    let replaced_content = file_content.replace("{{name}}", project_name);
    fs::write(file_path, replaced_content)
        .with_context(|| format!("Failed to write file {}", file_path))?;
    Ok(())
}

fn scan_targets() -> Result<Vec<String>> {
    let targets = fs::read_dir("./")?
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                let path = e.path();
                if path.is_dir() {
                    path.file_name()
                        .and_then(|name| name.to_str().map(|s| s.to_string()))
                } else {
                    None
                }
            })
        })
        .collect::<Vec<String>>();

    let supported_targets = vec!["web".to_string()];

    Ok(targets
        .into_iter()
        .filter(|t| supported_targets.contains(t))
        .collect())
}
