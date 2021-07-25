//rust用use来引用外部模块；使用枚举值；为某个作用域下的方法或作用域创建别名。可参考https://doc.rust-lang.org/1.0.0/std/net/index.html
use std::io::{Error, Read, Write}; //为了处理错误
use std::net::{TcpListener, TcpStream};
use std::thread; //创建线程
use std::time;

fn handle_client(mut stream: TcpStream) -> Result<(), Error>{ //mut-可变的意思；返回io的result
    let mut buf = [0; 512]; //创建buffer（对应的处理函数）
    for _ in 0..1000 {
        let bytes_read = stream.read(&mut buf)?;//读buffer里的内容
        if bytes_read == 0 {
            return Ok(()); //如果是0，说明已结束，返回ok
        }

        stream.write(&buf[..bytes_read])?;//否则逆序写回去
        thread::sleep(time::Duration::from_secs(1 as u64));//用time包的原因
    }

    Ok(())
}

fn main() -> std::io::Result<()> { //io，返回错误
    let listener = TcpListener::bind("127.0.0.1:8080")?; //创建listener，用bind的函数；？是简写，也可以用except等
    let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new(); //创建容器放置线程

    for stream in listener.incoming() { //输出内容，用的incoming函数
        let stream = stream.expect("failed!"); //如果流本身有问题则打failed
        let handle = thread::spawn(move || {
            handle_client(stream) //handle client返回的是result类型，所以可用unwrap
		.unwrap_or_else(|error| eprintln!("{:?}", error)); //eprintln打印error
        });

        thread_vec.push(handle); //把handle加到容器里去
    }

    for handle in thread_vec {
        handle.join().unwrap(); //等待每个线程的结束
    }

    Ok(())
}