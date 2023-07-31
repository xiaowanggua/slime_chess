use std::{net::TcpStream, io::{Read, Write}, time, thread,io};
pub fn handle_client(mut stream: TcpStream) -> io::Result<()>{
    let mut buf = [0; 512];
    //创建一个叫buf的数组，内容为0，长度为512
    loop {
        //该循环表示server端永久提供服务，因为默认服务器为永不关闭的
        let bytes_read = stream.read(&mut buf)?;
        //从流里面读内容，读到buf中
        if bytes_read == 0 {
            return Ok(());
            //如果读到的为空（即0），则说明已经结束了
        }
        stream.write(&buf[..bytes_read])?;
        //否则把它写回去
        thread::sleep(time::Duration::from_secs(1));
        //调用sleep函数实现服务的间隔，间隔1s
    }
}
