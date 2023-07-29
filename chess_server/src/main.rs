use std::{net::TcpListener, io::{Read, Write}};
use std::str;
fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("running on port 3000....");
    // 请求进来
    for stream in listener.incoming() {
        // 设置为可变
        let mut stream = stream.unwrap();
        println!(" Connection established");

        // 设置接收缓冲区
        let mut buffer = [0; 1024];
        // 服务端接收数据
        stream.read(&mut buffer).unwrap();
        println!(
             "Request from client:{:?}",
             str::from_utf8(&buffer).unwrap() // 转换为utf8格式
         );
        let content = String::from(str::from_utf8(&buffer).unwrap().trim())+"server";
        // 服务端返回数据
        stream.write(content.as_bytes()).unwrap();
    }
}
