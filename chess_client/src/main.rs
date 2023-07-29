use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;  // 引入字符串转换包

fn main() {
    // 转换为字节类型，向服务器发送
    let stdin = std::io::stdin();
    loop{
        let mut stream = TcpStream::connect("127.0.0.1:3000").unwrap();

        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        stream.write(input.as_bytes()).unwrap();

        let mut buffer = [0; 1024]; // 注意括号里格式
    // 读取服务器返回的消息
        stream.read(&mut buffer).unwrap();

        println!(
            "Response from server:{:?}",
            str::from_utf8(&buffer).unwrap().trim() // 转换为utf8格式
        );
    }
}