use actix_files as fs;
use actix_web::{App, HttpServer};
use colored::*;
use ipnetwork::IpNetwork;
use pnet::datalink::interfaces;
use pnet::ipnetwork;
// use std::env;

async fn print_available_urls(port: u16) {
    let local_ip = "127.0.0.1".to_string();
    let mut network_ips = vec![];

    // 获取网络接口的 IP 地址
    for interface in interfaces() {
        for ip in interface.ips {
            if let IpNetwork::V4(ipv4) = ip {
                if ipv4.ip().is_private() {
                    network_ips.push(ipv4.ip().to_string());
                }
            }
        }
    }

    // 打印本地地址
    println!(
        "{}",
        format!("  > Local:    http://{}:{}/", local_ip, port).yellow()
    );

    // 打印网络地址
    for ip in network_ips {
        println!(
            "{}",
            format!("  > Network:  http://{}:{}/", ip, port).cyan()
        );
    }
}

pub async fn start_web_server() -> std::io::Result<()> {
    let host = "127.0.0.1";
    let port = 4000;

    // 获取当前工作目录
    // let current_dir = env::current_dir().unwrap();
    // let web_dir = current_dir.join("target/app/web");

    // 打印启动信息
    println!("{}", "Server is running!".green());
    println!("{}", "Running at:".cyan());

    // 打印可访问的 URL
    print_available_urls(port).await;

    HttpServer::new(move || {
        App::new().service(fs::Files::new("/", "./target/app/web").index_file("index.html"))
    })
    .bind((host, port))?
    .run()
    .await
}
