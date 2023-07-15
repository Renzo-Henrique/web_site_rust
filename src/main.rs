use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use std::fs;





fn read_pages(path: &str) -> Result<(), Box<dyn std::error::Error>>{

   // let names =
   Ok(())
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        tokio::spawn(async move { 
            if let Err(e) = process_request(socket).await {
                eprintln!("Erro ao processar a requisição: {}", e);
            }
        });
    }
}

async fn process_request(mut socket: TcpStream) -> Result<(), Box<dyn std::error::Error>> {

    loop{

        let mut buffer = [0u8; 1024];
        let bytes_read = socket.read(&mut buffer).await?;

        if bytes_read == 0{
            break;
        }
    
        let file_content = fs::read_to_string("pages/Home/esquecer_senha.html")?;
    
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            file_content.len(),
            file_content
        );
    
        socket.write_all(response.as_bytes()).await?;
        socket.flush().await?;
    
        Ok(())
    }
}



static HELLO_WORLD: &str = r#"
<!DOCTYPE html>
<html>
<head>
    <title>Hello, World!</title>
</head>
<body>
    <h1>Ai chavinho que gostoso!</h1>
    <div>
        <{{name}}/>
    <div/>
</body>
</html>
"#;
