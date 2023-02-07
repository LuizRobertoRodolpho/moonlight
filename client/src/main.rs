use tokio::{
    io::{AsyncWriteExt },
    net::{TcpStream}
};

#[tokio::main]
async fn main() {
    // Connect to a peer
    let mut connection = TcpStream::connect("127.0.0.1:5000").await.unwrap();

    tokio::spawn(async move {
        let (_, mut writer) = connection.split();
        let msg = &"aaaaaa\n";
        writer.write_all(msg.as_bytes()).await.unwrap();
    })
    .await
    .unwrap();
}
