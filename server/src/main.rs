
/* 
use core::panic;
use std::collections::HashMap;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;
use std::sync::Mutex;


use serde::{Deserialize, Serialize};
use serde_json;


// Definição da estrutura de dados para representar um usuário
#[derive(Debug, Deserialize, Serialize)]
struct User{
    username: String,
    password: String,
    favorite_language: String,
}

// Definição da estrutura de dados para representar os dados de login
#[derive(Debug, Deserialize, Serialize)]
struct loginData{
    username: String,
    password: String,
}

// Definição da estrutura AppState que representa o estado da aplicação,
// armazenando um HashMap com usuários. O Mutex é usado para permitir que
// várias threads acessem o HashMap de forma segura.
struct AppState{
    users: Mutex<HashMap<String, User>>
}




fn main() {
    // Criação do estado da aplicação
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

// Função que trata as requisições GET
fn get_handler(server_path: &[u8], app_state: &mut AppState) -> Option<String>{

    // Caminhos de URL obtidos da conexão TCP para as páginas de login, cadastro e página inicial
    let login = b"/login";
    let cadastro = b"/cadastro";
    let inital_page = b"/";
    
    // Variável para armazenar o conteúdo do arquivo HTML
    let mut str_file: Option<String> = None;
 
    // Verifica o caminho de URL recebido e redireciona para o correto
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

// Função que trata as requisições POST
fn post_handler(html_content: &[u8], app_state: &mut AppState) -> Option<String>{
    // Caminhos de URL obtidos da conexão TCP para as páginas de login, cadastro
    let login =  b"/login";
    let cadastro = b"/cadastro";

    // Variável para armazenar a resposta da requisição
    let mut file_content: Option<String> = None;

    if html_content.starts_with(cadastro){
        // Converte o conteúdo da requisição (sequência de bytes) para uma String
        let html_string = std::str::from_utf8(html_content).unwrap();

        // Localiza o índice onde o payload da requisição começa (após as quebras de linha)
        let payload_index = html_string.find("\r\n\r\n").unwrap();

        // Extrai a parte do payload JSON da requisição e remove caracteres nulos
        let json_str = html_string[(payload_index + 4)..].trim_matches('\0');
        
        // Desserializa o JSON recebido para a estrutura User
        let new_user = serde_json::from_str::<User>(json_str).unwrap();

        // Bloqueia o Mutex para acessar o HashMap de usuários de forma segura
        let mut users = &mut app_state.users.lock().unwrap();

        // Verifica a validacao da criacao de um usuario, insere na hashmap se for novo
        if let Some(user) = users.get(&new_user.username){
            return Some("Usuário já existente".to_string());
        }else {
            users.insert(new_user.username.clone(), new_user);
            return Some("cadastro feito com sucesso".to_string());
        }

        //r#"{"username":"a","password":"a","favorite_language":"a"}"#

    }

    else if html_content.starts_with(login){

        // Converte o conteúdo da requisição (sequência de bytes) para uma String
        let html_string = std::str::from_utf8(html_content).unwrap();

        // Localiza o índice onde o payload da requisição começa (após as quebras de linha)
        let payload_index = html_string.find("\r\n\r\n").unwrap();

        // Extrai a parte do payload JSON da requisição e remove caracteres nulos
        let json_str = html_string[(payload_index + 4)..].trim_matches('\0');
        
        // Desserializa o JSON recebido para a estrutura loginData
        let user_data = serde_json::from_str::<loginData>(json_str).unwrap();

        // Bloqueia o Mutex para acessar o HashMap de usuários de forma segura
        let mut users = &mut app_state.users.lock().unwrap();

        // Verifica se os dados de login sao corretos
        if let Some(user) = users.get(&user_data.username){
            if user_data.password == user.password{
                
                // Redirecionar para o perfil do usuário
                //let profile_url = format!("/perfil/{}", user.username);

                //return Some(format!("HTTP/1.1 302 Found\r\nLocation: {}\r\n\r\n", profile_url));

                return Some("voce esta logado".to_string());
            }
        }

        return Some("senha ou usuario incorretos".to_string());
        

    }



    None
}

// Função que lida com a conexão TCP
fn handle_connection(mut stream: TcpStream, app_state: &mut AppState) {
    // Buffer para armazenar os dados recebidos pela conexão TCP
    let mut buffer = [0; 1024];
    // Lê os dados recebidos pela conexão e os armazena no buffer
    stream.read(&mut buffer).unwrap();


    // Verifica o tipo de requisição recebida
    let get = b"GET";
    let post = b"POST";
    let mut file_content: Option<String> = None;


    // Realiza a funcao para cada requisicao
    if buffer.starts_with(get){
       file_content =  get_handler(&buffer[4..], app_state);
    }
    else if buffer.starts_with(post){
        file_content = post_handler(&buffer[5..], app_state);
    }

    // Com base no conteúdo retornado pelas funções 'get_handler' ou 'post_handler',
    // constrói a resposta que será enviada de volta ao cliente
    let response = match file_content{
        Some(x) => {
            // Se 'file_content' contém algum conteúdo (Some),
            // constrói uma resposta bem-sucedida com o código 200 OK e o conteúdo retornado
            format!("{}{}","HTTP/1.1 200 OK\r\n\r\n", x)
        }
        _ => {
            // Caso contrário, se 'file_content' estiver vazio (None),
            // constrói uma resposta com o código 404 NOT FOUND e carrega a página de erro 404.html
            format!("{}{}", "HTTP/1.1 404 NOT FOUND\r\n\r\n", "./public/404.html")
        }
    };

    // Escreve a resposta no buffer de saída da conexão TCP e a envia de volta ao cliente
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    
}

*/



use serde::de::Error;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde::{Deserialize, Serialize};
use serde_json;
use tokio::fs;

use core::fmt;
use std::collections::HashMap;
use std::sync::{Arc , Mutex};


#[derive(Debug, Deserialize, Serialize)]
struct User{
    username: String,
    password: String,
    favorite_language: String,
}

// Definição da estrutura de dados para representar os dados de login
#[derive(Debug, Deserialize, Serialize)]
struct LoginData{
    username: String,
    password: String,
}

struct AppState{
    users: Mutex<HashMap<String, User>>
}

use std::io;


async fn get_handler(server_path: &[u8], mut app_state: Arc<AppState>) -> Result<String, ()>{

    // Caminhos de URL obtidos da conexão TCP para as páginas de login, cadastro e página inicial
    let login = b"/login";
    let cadastro = b"/cadastro";
    let inital_page = b"/";
    
    // Variável para armazenar o conteúdo do arquivo HTML
    

    let file_path = match server_path{
        _ if server_path.starts_with(login) => "./public/tela_login.html",
        _ if server_path.starts_with(cadastro) => "./public/tela_cadastro.html",
        _ if server_path.starts_with(inital_page) => "./public/tela_inicial.html",
        _ => return Err(()),
    };


    Ok(fs::read_to_string(file_path).await.unwrap())
}


#[derive(Debug)]
struct MyError{
    message:String,
}

impl MyError{
    fn new(message: &str) -> Self{
        MyError { message: message.to_owned() }
    }
}

impl fmt::Display for MyError{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for MyError{}


fn to_json<'a, T>(content: &'a [u8]) -> Result<T,  Box<dyn std::error::Error >>
where
    T: Deserialize<'a> + Serialize,
{
    let msg_erro: &'static str = "couldn't find";
    // Converte o conteúdo  (sequência de bytes) para uma String
    let content = std::str::from_utf8(content)?;

    // Localiza o índice onde o payload da requisição começa (após as quebras de linha)
    if let Some(i) = content.find("\r\n\r\n"){
        // Extrai a parte do payload JSON da requisição e remove caracteres nulos
        let json_str = content[(i + 4)..].trim_matches('\0');
         // Desserializa o JSON recebido para a estrutura T genérica
        let new_ = serde_json::from_str::<T>(json_str)?;

        return Ok(new_);
    }else{
        return Err(Box::new(MyError::new(msg_erro)))
    }
}



async fn post_handler(html_content: &[u8], mut app_state: Arc<AppState>) -> Result<String, ()> {
    // Caminhos de URL obtidos da conexão TCP para as páginas de login, cadastro
    let login =  b"/login";
    let cadastro = b"/cadastro";

   
    if html_content.starts_with(cadastro){
        
        let new_user: User = to_json(html_content).unwrap();

        // Bloqueia o Mutex para acessar o HashMap de usuários de forma segura
        let mut users = &mut app_state.users.lock().unwrap();

        // Verifica a validacao da criacao de um usuario, insere na hashmap se for novo
        if let Some(user) = users.get(&new_user.username){
            return Ok("Usuário já existente👺".to_string());
        }else {
            users.insert(new_user.username.clone(), new_user);
            return Ok("Cadastro feito com sucesso 🧙‍♂️☄️".to_string());
        }
    }

    else if html_content.starts_with(login){

       
        let login_data: LoginData = to_json(html_content).unwrap();

        // Bloqueia o Mutex para acessar o HashMap de usuários de forma segura
        let mut users = &mut app_state.users.lock().unwrap();

        // Verifica se os dados de login sao corretos
        if let Some(user) = users.get(&login_data.username){
            if login_data.password == user.password{
                
                // Redirecionar para o perfil do usuário
                //let profile_url = format!("/perfil/{}", user.username);

                //return Some(format!("HTTP/1.1 302 Found\r\nLocation: {}\r\n\r\n", profile_url));

                return Ok("Você esta logado💪".to_string());
            }
        }

        return Ok("Senha e/ou usuário incorretos👻".to_string());
        

    }


    Err(())
    
}

async fn handle_connection(mut stream: TcpStream, mut app_state: Arc<AppState>) -> io::Result<()>{
    // Buffer para armazenar os dados recebidos pela conexão TCP
    let mut buffer = [0; 1024];
    // Lê os dados recebidos pela conexão e os armazena no buffer

    stream.read(&mut buffer).await;

    // Verifica o tipo de requisição recebida
    let get = b"GET";
    let post = b"POST";
    let mut response_content: Result<String, ()> = Err(());


    // Realiza a funcao para cada requisicao
    
    if buffer.starts_with(get){
        response_content =  get_handler(&buffer[4..], app_state).await;
    }
    
    else if buffer.starts_with(post){
        response_content = post_handler(&buffer[5..], app_state).await;
    }

    // Com base no conteúdo retornado pelas funções 'get_handler' ou 'post_handler',
    // constrói a resposta que será enviada de volta ao cliente
    let response = match response_content{
        Ok(x) => {
            // Se 'file_content' contém algum conteúdo (Some),
            // constrói uma resposta bem-sucedida com o código 200 OK e o conteúdo retornado
            format!("{}{}","HTTP/1.1 200 OK\r\n\r\n", x)
        }
        _ => {
            // Caso contrário, se 'file_content' estiver vazio (None),
            // constrói uma resposta com o código 404 NOT FOUND e carrega a página de erro 404.html
            format!("{}{}", "HTTP/1.1 404 NOT FOUND\r\n\r\n", "./public/404.html")
        }
    };

    // Escreve a resposta no buffer de saída da conexão TCP e a envia de volta ao cliente
    stream.write(response.as_bytes()).await?;
    stream.flush().await?;
    Ok(())
}



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{


    let mut app_state = AppState{
        users: Mutex::new(HashMap::<String, User>::new())
    };

    let mut app_state = Arc::new(app_state);

    let listener = TcpListener::bind("127.0.0.1:8080").await?;


    loop{

        match listener.accept().await{

            Ok( (socket, _ ) ) =>{
                let apst_clone = app_state.clone();
                tokio::spawn( 
                    async move {
                       handle_connection(socket, apst_clone).await;
                    }
                );
            }
            Err(e) => eprintln!("Some error ocurred when listening {:?}", e ),
        }
    }



}