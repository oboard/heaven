mod config;
mod nodejs;
mod web;

use config::Build;

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
use nodejs::run_node_js;
use web::start_web_server;

use std::env;

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

fn build_wasm() -> Result<()> {
    Command::new("moon")
        .args(&["build", "--target", "wasm"])
        .status()
        .context("Failed to execute moon build command")?;

    Ok(())
}

fn web_sync_project() -> Result<()> {
    // 重新编译wasm到/target/app/web/assets/app.wasm
    // moon build --target wasm --target-dir target/app/web/assets/
    let exe_path = get_exe_parent_dir()?;

    // fs::copy(
    //     exe_path.join("/resources/templates/main/heaven_web.mbt"),
    //     "main/heaven_web.mbt",
    // )
    // .context("Failed to copy lib files")?;

    build_wasm().context("Failed to build wasm")?;

    fs::copy(
        "target/wasm/release/build/main/main.wasm",
        "target/app/web/assets/app.wasm",
    )
    .context("Failed to copy wasm file")?;

    // copy libs
    copy_dir_all(
        &exe_path.join("/resources/templates/web/assets/lib"),
        Path::new("target/app/web/assets/lib"),
    )
    .context("Failed to copy lib files")?;

    Ok(())
}

async fn run_project(build: Build) -> Result<()> {
    let target_dir = build.target_dir.clone();
    let target = determine_target(&build.target)?;
    build_project(build).context("Failed to build project")?;
    match target.as_str() {
        "web" => {
            std::thread::spawn(move || {
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
                                        println!(
                                            "{} files changed",
                                            event.paths.len().to_string().blue()
                                        );
                                    }

                                    let _ = web_sync_project();
                                }
                                Err(e) => eprintln!("Error: {:?}", e),
                            }
                        },
                        notify::Config::default(),
                    )?;

                    watcher.watch(Path::new("main"), RecursiveMode::Recursive)?;
                    watcher.watch(Path::new("lib"), RecursiveMode::Recursive)?;

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

                let _ = monitor_changes();
            });
            // 调用 start 函数来启动服务器
            if let Err(e) = start_web_server().await {
                eprintln!("{} {}", "Error starting server:".red().bold(), e);
            }
        }
        "node" => {
            println!("Starting node.js...");
            // 调用 node
            run_node_js(target_dir).context("Failed to run node server")?;
        }
        _ => {
            println!("Unsupported target.");
        }
    }

    Ok(())
}

fn get_exe_parent_dir() -> Result<std::path::PathBuf> {
    let exe_path = env::current_exe()?;
    Ok(exe_path.parent().unwrap().to_path_buf())
}

fn new_project() -> Result<()> {
    let _exe_path = get_exe_parent_dir()?;

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

    fn copy_template(name: &str, dir: &str) -> Result<()> {
        let _exe_path = get_exe_parent_dir()?;
        let template_path = _exe_path.join(format!("resources/templates/{dir}"));
        copy_dir_all(&template_path, Path::new(&name).join(dir).as_path())
            .context("Failed to copy lib files")?;
        Ok(())
    }

    copy_template(&name, "web").context("Failed to copy web template")?;
    copy_template(&name, "node").context("Failed to copy node template")?;
    copy_template(&name, "main").context("Failed to copy main template")?;

    // 在moon.pkg.json中增加
    //   "link": {
    //     "wasm": {
    //       "exports": ["h_rs", "h_rd", "h_re"]
    //     }
    //   }

    let moon_pkg_json_path = format!("{}/main/moon.pkg.json", name);
    let moon_pkg_json_content = fs::read_to_string(&moon_pkg_json_path)
        .with_context(|| format!("Failed to read file {}", moon_pkg_json_path))?;
    let mut moon_pkg_json = serde_json::from_str::<serde_json::Value>(&moon_pkg_json_content)
        .with_context(|| format!("Failed to parse json {}", moon_pkg_json_path))?;

    let link_obj = serde_json::json!({
        "wasm": {
            "exports": ["h_rs", "h_rd", "h_re"]
        }
    });

    moon_pkg_json["link"] = link_obj;

    let moon_pkg_json_content = serde_json::to_string_pretty(&moon_pkg_json)
        .with_context(|| format!("Failed to serialize json {}", moon_pkg_json_path))?;

    fs::write(&moon_pkg_json_path, moon_pkg_json_content)
        .with_context(|| format!("Failed to write file {}", moon_pkg_json_path))?;

    // replace_template_variables(&format!("{}/main/moon.pkg.json", name), &name, &username)?;
    replace_template_variables(&format!("{}/web/index.html", name), &name, &username)?;
    replace_template_variables(&format!("{}/web/manifest.json", name), &name, &username)?;

    // println!("{}", format!("Project {} created!", name).green());

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

fn determine_target(target: &Option<String>) -> Result<String> {
    if let Some(target) = target {
        Ok(target.clone())
    } else {
        let available_targets = scan_targets()?;
        if available_targets.is_empty() {
            return Err(anyhow!("No available targets found."));
        }
        if available_targets.len() == 1 {
            Ok(available_targets[0].clone())
        } else {
            let target: String = Input::new()
                .with_prompt("Select output target")
                .default(available_targets[0].clone())
                .interact_text()?;
            Ok(target)
        }
    }
}

fn build_project(build: Build) -> Result<()> {
    let exe_path = get_exe_parent_dir()?;

    let target = determine_target(&build.target)?;

    copy_dir_all(
        &exe_path.join("resources/templates/main"),
        Path::new("main"),
    )
    .context("Failed to copy main lib files")?;

    match target.as_str() {
        "web" => {
            println!("Building for web...");
            // Command::new("moon")
            //     .args(&["build", "--target", "wasm"])
            //     .status()
            //     .context("Failed to execute moon build command")?;

            let target_dir = build.target_dir.replace("{target}", "web");

            fs::create_dir_all(&format!("{}/assets/lib", target_dir))
                .context("Failed to create target directory")?;

            copy_dir_all(Path::new("web"), Path::new(&format!("{}/", target_dir)))
                .context("Failed to copy web files")?;

            copy_dir_all(
                &exe_path.join("resources/templates/web/assets/lib"),
                Path::new("web/assets/lib"),
            )
            .context("Failed to copy web lib files")?;

            // fs::copy(
            //     "target/wasm/release/build/main/main.wasm",
            //     format!("{}/assets/app.wasm", target_dir),
            // )
            // .context("Failed to copy wasm file")?;
            let _ = web_sync_project();

            println!("{}", "Build completed!".green());
        }
        "node" => {
            println!("Building for node...");
            let target_dir = build.target_dir.replace("{target}", "node");

            fs::create_dir_all(&target_dir).context("Failed to create target directory")?;

            copy_dir_all(Path::new("node"), Path::new(&target_dir))
                .context("Failed to copy node files")?;

            build_wasm().context("Failed to build wasm")?;

            fs::copy(
                "target/wasm/release/build/main/main.wasm",
                Path::new(&format!("{}/assets/app.wasm", target_dir)),
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

fn replace_template_variables(file_path: &str, project_name: &str, author: &str) -> Result<()> {
    let file_content = fs::read_to_string(file_path)
        .with_context(|| format!("Failed to read file {}", file_path))?;
    let replaced_content = file_content
        .replace("{{name}}", project_name)
        .replace("{{author}}", author);
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

    let supported_targets = vec!["web".to_string(), "node".to_string()];

    Ok(targets
        .into_iter()
        .filter(|t| supported_targets.contains(t))
        .collect())
}
