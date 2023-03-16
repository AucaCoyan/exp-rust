use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8080").await.unwrap();

    let (mut socket, _addr) = listener.accept().await.unwrap();

    // write a buffer of 1kb length
    let mut reader = BufReader::new(socket);

    let mut line = String::new();

    loop {
        let bytes_read = reader.read_line(&mut line).await.unwrap();

        // write all the bytes
        socket.write_all(&line.as_bytes()).await.unwrap();
    }
}
