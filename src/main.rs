// ANCHOR: main_func
use async_std::net::TcpListener;
use async_std::net::TcpStream;
use futures::stream::StreamExt;
use async_std::task::spawn;
use async_std::prelude::*;

#[async_std::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap();
    listener
        .incoming()
        .for_each_concurrent(/* limit */ None, |stream| async move {
            let stream = stream.unwrap();
            spawn(handle_connection(stream));
        })
        .await;
}


async fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).await.unwrap();

    println!("{:?}", buffer);

    const RESPONSE: &'static str = "";
    stream.write(RESPONSE.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
}
