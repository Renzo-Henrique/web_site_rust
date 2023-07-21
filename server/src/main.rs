use serde::de::Error;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::json;
use tokio::fs;

use core::fmt;
use std::collections::HashMap;
use std::hash::Hash;
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


struct Token{
    value: i32,
}


use sha2::{Sha256, Digest};

impl  Token {

    fn new( name: &str) -> Self{
    
        Token { value: 1 }
    }
}
struct AppState{
    users: Mutex<HashMap<String, User>>,
    tokens: Mutex<HashMap< Token , User>>,
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
                let response = json!(
                    {
                        "message" : "Você esta logado💪",
                        "token" : "69",
                        "status" : "0"
                    }
                );
                return Ok(response.to_string());
            }
        }

        let response = json!(
            {
                "message" : "Senha e/ou usuário incorretos👻",
                "status": "1"
            }
        );
        return Ok(response.to_string());

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
        users: Mutex::new(HashMap::<String, User>::new()),
        tokens: Mutex::new(HashMap::<Token, User>::new()),
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