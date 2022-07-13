/*
 * @Author: Gxp-Ning 77679755+Gxp-Ning@users.noreply.github.com
 * @Date: 2022-07-12 22:03:50
 * @LastEditors: Gxp-Ning 77679755+Gxp-Ning@users.noreply.github.com
 * @LastEditTime: 2022-07-13 21:48:11
 * @FilePath: \tcpsever\src\main.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time;

fn main() -> io::Result<()>{
    //定义一个listener监听本地IP端口
    let tcp_listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    //创建线程容器
    let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();
    for stream in tcp_listener.incoming() {
        //转换流  失败则报错退出
        let stream = stream.expect("connect failed!");
        //闭包处理每个输入的流
        let handle = thread::spawn(move || {
            handle_client(stream).unwrap();
        });
        //把handle加入容器内
        thread_vec.push(handle);
    }
    for handle in thread_vec {
        //等待线程结束
        handle.join().unwrap_err();
    }
    Ok(())
}

//处理输入的流
fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    //创建缓冲区
    let mut read_buff = [0; 512];
    loop {
        //读取流的内容到缓冲区
        let bytes_read = stream.read(&mut read_buff).unwrap();
        //内容为空则结束返回
        if bytes_read == 0 {
           return Ok(());
        }
        //将读取到的内容写回流中
        stream.write(&read_buff[..bytes_read])?;
        //关闭流
        stream.flush()?;
        //间隔一秒
        thread::sleep(time::Duration::from_secs(1));
    }
    Ok(())
}