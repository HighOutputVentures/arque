use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio_util::codec::LengthDelimitedCodec;

pub struct ServerOptions {
    port: Option<u16>,
    hostname: Option<String>,
}

pub struct Server {}

impl Server {
    pub async fn start(options: ServerOptions) -> Result<Self, Box<dyn std::error::Error>> {
        let addr: SocketAddr = format!(
            "{}:{}",
            options.hostname.unwrap_or("[::1]".to_string()),
            options.port.unwrap_or(22714)
        )
        .parse()?;

        let listener = TcpListener::bind(addr).await?;

        tokio::spawn(async move {
            loop {
                match listener.accept().await {
                    Ok((socket, _)) => {
                        let codec = LengthDelimitedCodec::builder()
                            .length_field_offset(5)
                            .length_field_length(3)
                            .length_adjustment(0)
                            .new_read(socket);

                        tokio::spawn(async move {
                            let mut buf = [0; 4096];
                        });
                    }
                    Err(e) => println!("failed to accept connection"),
                }
            }
        })
        .await?;

        Ok(Self {})
    }
}
