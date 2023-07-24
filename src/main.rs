use tokio::{
    io::{AsyncBufReadExt, BufReader},
    net::TcpStream,
};

const MUD_ADDRESS: &str = "batmud.bat.org:23";
//const MUD_ADDRESS: &str = "grimne.org:4000";

#[tokio::main]
async fn main() {
    println!("Hello, async world!");

    let stream: TcpStream = TcpStream::connect(MUD_ADDRESS).await.unwrap();
    let (r, mut _w) = tokio::io::split(stream);
    let mut reader = BufReader::new(r);
    let mut line = String::new();
    loop {
        tokio::select! {
            result = reader.read_line(&mut line) => {
                let x = format!("lelle [{}::{}]: {}", result.unwrap(), line.len(), line).to_string();
                print!("{}", x);
                line.clear();
            }

        }
    }
}
