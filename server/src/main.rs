use core::panic;
use std::collections::HashMap;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;
use std::sync::Mutex;


use serde::{Deserialize, Serialize};
use serde_json;


#[derive(Debug, Deserialize, Serialize)]
struct User{
    username: String,
    password: String,
    favorite_language: String,
}


#[derive(Debug, Deserialize, Serialize)]
struct loginData{
    username: String,
    password: String,
}

struct AppState{
    users: Mutex<HashMap<String, User>>
}




fn main() {
    
    let mut app_state = AppState{
        users: Mutex::new(HashMap::<String, User>::new())
    };

    // Criação de um listener TCP que escuta em localhost na porta 7878
    let listener = TcpListener::bind("localhost:7878").unwrap();
    
    // Loop que lida com cada conexão recebida pelo listener
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // Chama a função para lidar com a conexão
        handle_connection(stream, &mut app_state);
       
    }
}


fn get_handler(server_path: &[u8], app_state: &mut AppState) -> Option<String>{
    
    let login = b"/login";
    let cadastro = b"/cadastro";
    let inital_page = b"/";
    

    let mut str_file: Option<String> = None;
 

    if server_path.starts_with(login){
        str_file = Some(fs::read_to_string("./public/tela_login.html").unwrap());
    }
    else if server_path.starts_with(cadastro){
        str_file = Some(fs::read_to_string("./public/tela_cadastro.html").unwrap())
    }
    else if server_path.starts_with(inital_page){
        str_file = Some(fs::read_to_string("./public/tela_inicial.html").unwrap());
    }


    str_file
}

fn post_handler(html_content: &[u8], app_state: &mut AppState) -> Option<String>{
    let login =  b"/login";
    let cadastro = b"/cadastro";

    let mut file_content: Option<String> = None;

    if html_content.starts_with(cadastro){
        let html_string = std::str::from_utf8(html_content).unwrap();

        let payload_index = html_string.find("\r\n\r\n").unwrap();
        let json_str = html_string[(payload_index + 4)..].trim_matches('\0');
    
        let new_user = serde_json::from_str::<User>(json_str).unwrap();

        let mut users = &mut app_state.users.lock().unwrap();

        if let Some(user) = users.get(&new_user.username){
            return Some("Usuário já existente".to_string());
        }else {
            users.insert(new_user.username.clone(), new_user);
            return Some("cadastro feito com sucesso".to_string());
        }

        //r#"{"username":"a","password":"a","favorite_language":"a"}"#

    }

    else if html_content.starts_with(login){
        let html_string = std::str::from_utf8(html_content).unwrap();

        let payload_index = html_string.find("\r\n\r\n").unwrap();
        let json_str = html_string[(payload_index + 4)..].trim_matches('\0');
    
        let user_data = serde_json::from_str::<loginData>(json_str).unwrap();


        let mut users = &mut app_state.users.lock().unwrap();

        if let Some(user) = users.get(&user_data.username){
            if user_data.password == user.password{
                return Some("voce esta logado".to_string());
            }
        }

        return Some("senha ou usuario incorretos".to_string());
        

    }



    None
}

fn handle_connection(mut stream: TcpStream, app_state: &mut AppState) {
    // Buffer para armazenar os dados recebidos pela conexão TCP
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

  

    //let mut data = String::from_utf8(data).unwrap();

    // Verifica o tipo de requisição recebida
    //let get = b"GET / HTTP/1.1\r\n";
    let get = b"GET";
    let post = b"POST";
    
    let mut file_content: Option<String> = None;


   
    if buffer.starts_with(get){
       file_content =  get_handler(&buffer[4..], app_state);
    }
    else if buffer.starts_with(post){
        file_content = post_handler(&buffer[5..], app_state);
    }

    let response = match file_content{
        Some(x) => {
            format!("{}{}","HTTP/1.1 200 OK\r\n\r\n", x)
        }
        _ => {
            format!("{}{}", "HTTP/1.1 404 NOT FOUND\r\n\r\n", "./public/404.html")
        }
    };


    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    
}
