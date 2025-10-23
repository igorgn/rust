use tokio::{
    io,
    net::{TcpListener, TcpStream},
};

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("localhost:1234").await?;

    loop {
        let (socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            echo(socket).await.unwrap();
        });
    }
}

async fn echo(mut socket: TcpStream) -> io::Result<()> {
    let (mut reader, mut writer) = socket.split();
    let n = io::copy(&mut reader, &mut writer).await?;
    // socket.read_to_end(&mut buffer).await?;
    println!("Sent back {:?} bytes", &n);
    // socket.write_all(&buffer).await?;
    Ok(())
}
