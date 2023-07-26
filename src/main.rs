use std::str::from_utf8;

use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};

const MUD_ADDRESS: &str = "batmud.bat.org:23";
//const MUD_ADDRESS: &str = "grimne.org:4000";

#[tokio::main]
async fn main() {
    println!("Client starting...");

    // server stuff
    let server_stream: TcpStream = TcpStream::connect(MUD_ADDRESS).await.unwrap();
    let (read_half, mut write_half) = tokio::io::split(server_stream);
    let mut reader = BufReader::new(read_half);
    let mut line = String::new();

    // hela tiden...
    loop {
        // ..race mellan dessa två futures
        // (läsa från server resp. skriva stdin -> server)
        tokio::select! {
            // läs varje "line" som kommer och skriv till skärmen
            result = reader.read_line(&mut line) => {
                let x = format!("line [{}::{}]: {}", result.unwrap(), line.len(), line).to_string();
                print!("{}", x);
                line.clear();
            }
            // läs från stdin och skriv till strömmen
            input = read_stdin() => {
                match input {
                    Ok(v) => {
                        // debugprint till skärm :-(
                        println!(" stdin::OK {}", v);
                        // mitt försök att skriva på strömmen
                        let _r = write_half.write_all(v.as_bytes()).await;
                    }
                    Err(e) => {
                        println!("stdin::ERR {:?}", e);
                    }
                }
            }

        }
    }
}

// läs från stdin och leverera resultat när så skett
async fn read_stdin() -> Result<String, Box<dyn std::error::Error>> {
    let mut buf: [u8; 512] = [0; 512];
    let count = tokio::io::stdin().read(&mut buf).await.unwrap();
    let input = from_utf8(&buf[..count]).unwrap().to_string();
    Ok(input)
}
