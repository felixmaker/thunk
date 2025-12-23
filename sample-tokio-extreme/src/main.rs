use derive_more::derive::Debug;
use derive_more::derive::Display;
use derive_more::derive::Error;
use derive_more::derive::From;
use libc_alloc::LibcAlloc;
use rustls::{ClientConfig, RootCertStore};
use rustls_pki_types::ServerName;
use std::sync::Arc;
use std::time::Duration;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::runtime::Builder;
use tokio::time::sleep;
use tokio_rustls::TlsConnector;

#[global_allocator]
static ALLOCATOR: LibcAlloc = LibcAlloc;

#[derive(Error, From, Display, Debug)]
enum Error {
    Io(std::io::Error),
    Tls(rustls::Error),
    Utf8(std::str::Utf8Error),
    Dns(rustls_pki_types::InvalidDnsNameError),
    Reqwest(reqwest::Error),
}

async fn exec() -> Result<(), Error> {
    let domain = "www.rust-lang.org";
    let content = format!("GET / HTTP/1.0\r\nHost: {}\r\n\r\n", domain);
    let dnsname = ServerName::try_from(domain)?;

    let connector = TlsConnector::from(Arc::new(
        ClientConfig::builder()
            .with_root_certificates(RootCertStore {
                roots: webpki_roots::TLS_SERVER_ROOTS.into(),
            })
            .with_no_client_auth(),
    ));

    let stream = TcpStream::connect(format!("{domain}:443")).await?;
    let mut stream = connector.connect(dnsname, stream).await?;
    stream.write_all(content.as_bytes()).await?;

    let mut buffer = Vec::new();
    stream.read_to_end(&mut buffer).await?;
    println!("{}", std::str::from_utf8(&buffer)?);
    Ok(())
}

async fn exec2() -> Result<(), Error> {
    println!(
        "{}",
        reqwest::get("https://www.rust-lang.org")
            .await?
            .text()
            .await?
    );
    Ok(())
}

async fn server() -> Result<(), Error> {
    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    loop {
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            let mut buf = [0; 1024];
            loop {
                let n = match socket.read(&mut buf).await {
                    // socket closed
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };
                if let Err(e) = socket.write_all(&buf[0..n]).await {
                    eprintln!("failed to write to socket; err = {:?}", e);
                    return;
                }
            }
        });
    }
}

fn main() -> Result<(), Error> {
    async fn main() -> Result<(), Error> {
        let _ = rustls_rustcrypto::provider().install_default();
        let _thread = tokio::spawn(async move {
            loop {
                let exec = async {
                    Ok::<_, Error>(
                        reqwest::get("https://www.rust-lang.org")
                            .await?
                            .text()
                            .await?,
                    )
                }
                .await;
                match exec {
                    Ok(_) => println!("ok"),
                    Err(e) => println!("{e:?}"),
                }
                sleep(Duration::from_millis(500)).await;
            }
        });

        press_btn_continue::wait("Press any key to continue...")?;
        Ok(())
    }

    let mut builder = {
        #[cfg(windows)]
        {
            let peb = unsafe { ntapi::ntpsapi::NtCurrentPeb() };
            if (unsafe { ((*peb).OSMajorVersion, (*peb).OSMinorVersion) } < (6, 0)) {
                Builder::new_current_thread()
            } else {
                Builder::new_multi_thread()
            }
        }

        #[cfg(not(windows))]
        {
            Builder::new_multi_thread()
        }
    };

    builder.enable_all().build()?.block_on(main())
}
