use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;
use std::net::TcpListener;
use std::thread;


fn main() {
    //通过通过TcpListener方法监听本地8080端口，通过unwrap方法来处理异常
    let l = TcpListener::bind("127.0.0.1:8080").unwrap(); 
    //通过迭代程序处理incoming的返回值
    for stream in l.incoming() {
        //创建一个线程来处理TCP接收到的请求
        thread::spawn(move || {
            //创建一个变量stream来存储接收到的对象
            let stream = stream.unwrap(); 
            //添加一个缓存存储stream的数据，存储到reader变量
            let reader = BufReader::new(&stream);
            // 读取可变变量writer，用于向缓存写入TCP返回值
            let mut writer = BufWriter::new(&stream);
            //通过迭代程序逐行处理reader中的文本
            for line in reader.lines() {
                //通过line变量覆盖原line变量
                let line = line.unwrap();
                //输出line的值
                println!("收到TCP请求，内容为：{}", line);
                //将回应TCP请求的字节串写入缓存
                writer.write_all(b"got it.\n").unwrap();
                //发出对TCP请求的回应，并处理异常
                match writer.flush() {
                    //回应成功后在屏幕打印出成功信息
                    Ok(n) => println!("已成功回应{:?}",n),
                    //异常后在屏幕打印出异常信息
                    Err(err) => println!("{:?}", err),
                };
            }
        });
    }
}