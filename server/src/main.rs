use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

use std::thread;
use std::time::Duration;

use std::fs;

fn main() {
    
    // Criação de um listener TCP que escuta em localhost na porta 7878
    let listener = TcpListener::bind("localhost:7878").unwrap();
    
    // Loop que lida com cada conexão recebida pelo listener
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // Chama a função para lidar com a conexão
        handle_connection(stream);
       
    }
}


fn get_handler(server_path: &str) -> Option<fs::File>{
    
    
}

fn handle_connection(mut stream: TcpStream) {
    // Buffer para armazenar os dados recebidos pela conexão TCP
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();


    // Verifica o tipo de requisição recebida
    //let get = b"GET / HTTP/1.1\r\n";
    let get = b"GET";
    let post = b"POST";
    

    if buffer.starts_with(get){
        get_handler(&[4..]);
    }
    // --snip--
    let (status_line, filename) = if buffer.starts_with(get) {
        // Se a requisição for GET "/"
        // Retorna a página inicial e o status 200 OK
        ("HTTP/1.1 200 OK\r\n\r\n", "./public/tela_inicial.html")
    }
    else if buffer.starts_with(post){
        // Se a requisição for POST
        // Retorna a página de login ??? e o status 200 OK
        ("HTTP/1.1 200 OK\r\n\r\n", "./public/tela_login.html")
    }
    else {
        // Para qualquer outra requisição
        // Retorna a página de erro 404 e o status 404 NOT FOUND
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "./public/404.html")
    };

    // Lê o conteúdo do arquivo com base no nome do arquivo recebido
    let contents = fs::read_to_string(filename).unwrap();

    // Cria a resposta HTTP a ser enviada para o cliente
    let response = format!("{}{}", status_line, contents);

    // Envia a resposta para o cliente através do stream TCP
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
