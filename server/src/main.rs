#[macro_use] extern crate rocket;

use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use rocket_contrib::json::Json;
use std::sync::Mutex;
use rocket::State;

use rocket::fs::NamedFile;


#[derive(Deserialize, Serialize, Debug)]
struct User{
    name: String,
    password: String,
    favorite_fruit: String,
    favorite_language: String,
    profile_picture: String,
}

struct AppState{
    users: Mutex<HashMap<String, User>>
}


#[get("/login")]
async fn get_login() -> Option<NamedFile>{

    NamedFile::open("static/login.html").await.ok()
}

/* 
#[post("/login", data = "<form>")]
fn login(form: Json<User>, state: &State<AppState>) -> String{
    
    let users = &state.users.lock().unwrap();
    let name = &form.name;
    let senha = &form.password;
   

    match  users.get(name) {
        Some(user) => {
            if user.password == senha{
                format!("bem vindo cliente {name}")
            }
        }
        _ =>{
            format!("falha ao logar, tente novamente")
        }
    }

}
*/
/* 
#[post("/cadastro", data = "<form>")]
fn cadastro(form: Json<User>, state: &State<AppState>) -> String{

    let users = &state.users.lock().unwrap();

    match users.get(&form.name) {
        Some(x) =>{
            format!("usuario ja cadastrado, tente fazer login")
        }
        _ =>{
            users.insert(&form.name, form);
            format!("usuario cadastrado com sucesso")
        }
    }

}
*/

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!!!!!"
}

#[launch]
fn rocket() -> _ {
    let users = HashMap::<String ,User>::new();
    let app_state = AppState{
        users: Mutex::new(users)
    };

    rocket::build().manage(app_state).mount("/", routes![hello, get_login])
}