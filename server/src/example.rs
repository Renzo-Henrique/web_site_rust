#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::{Request, response::Redirect};
use rocket::request::{Form, FromForm};

#[derive(FromForm)]
struct User {
    nome: String,
    senha: String,
    favorite_fruit: String,
    favorite_language: String,
    profile_picture: String,
}

#[get("/")]
fn login() -> &'static str {
    "
<!DOCTYPE html>
<html>
<head>
    <title>Exemplo de POST</title>
    <style>
        body {
            background-color: #f2f2f2;
            font-family: Arial, sans-serif;
            display: flex;
            justify-content: center;
            align-items: center;
            height: 100vh;
            margin: 0;
            padding: 0;
            background-image: linear-gradient(to right, rgba(0, 0, 0, 0.7), #FF8C00, #FFA500, #C76100, #8B5200);
        }

        .container {
            background-color: #fff;
            border-radius: 8px;
            box-shadow: 0 4px 10px rgba(0, 0, 0, 0.1);
            padding: 40px;
            width: 360px;
            text-align: center;
        }

        h1 {
            color: #333;
            font-size: 24px;
            margin-bottom: 30px;
        }

        form {
            display: flex;
            flex-direction: column;
            align-items: center;
        }

        label {
            font-weight: bold;
            color: #555;
            margin-bottom: 8px;
            display: block;
        }

        input[type=\"text\"],
        input[type=\"password\"] {
            padding: 12px;
            border: 1px solid #ccc;
            border-radius: 4px;
            width: 100%;
            margin-bottom: 16px;
        }

        button {
            background-color: #4caf50;
            color: #fff;
            border: none;
            border-radius: 4px;
            padding: 12px 20px;
            font-size: 16px;
            cursor: pointer;
            transition: background-color 0.3s ease;
            width: 100%;
        }

        button:hover {
            background-color: #45a049;
        }
    </style>
</head>
<body>
    <div class=\"container\">
        <h1>Login</h1>
        <form id=\"meuFormulario\" action=\"/authenticate\" method=\"post\">
            <label for=\"nome\">Nome:</label>
            <input type=\"text\" id=\"nome\" name=\"nome\" required>

            <label for=\"senha\">Senha:</label>
            <input type=\"password\" id=\"senha\" name=\"senha\" required>

            <button type=\"submit\">Logar</button>
        </form>
    </div>
</body>
</html>
"
}

#[post("/authenticate", data = "<user>")]
fn authenticate(user: Form<User>) -> Redirect {
    if user.nome == "manga" && user.senha == "zedamanga" {
        Redirect::to(format!("/home?{}", user.nome))
    } else {
        Redirect::to("/")
    }
}

#[get("/home?<user>")]
fn home(user: User) -> String {
    format!(
        "
<!DOCTYPE html>
<html>
<head>
    <title>Página Home</title>
    <style>
        body {{
            background-color: #000;
            font-family: Arial, sans-serif;
            display: flex;
            justify-content: center;
            align-items: center;
            height: 100vh;
            margin: 0;
            padding: 0;
            background-image: linear-gradient(to right, rgba(0, 0, 0, 0.7), #FF8C00, #FFA500, #C76100, #8B5200);
            background-repeat: no-repeat;
            background-size: cover;
            position: relative;
        }}

        .container {{
            background-color: rgba(255, 255, 255, 0.8);
            border-radius: 8px;
            box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
            padding: 20px;
            width: 320px;
            text-align: center;
            color: #333;
        }}

        h1 {{
            font-size: 24px;
            margin-bottom: 30px;
        }}

        .profile-container {{
            display: flex;
            justify-content: center;
            margin-bottom: 20px;
        }}

        .profile-pic {{
            width: 150px;
            height: 150px;
            border-radius: 50%;
        }}

        .info {{
            margin-bottom: 10px;
        }}

        label {{
            font-weight: bold;
        }}
    </style>
</head>
<body>
    <div class=\"container\">
        <h1>Página Home</h1>
        <div class=\"profile-container\">
            <img src=\"{}\" alt=\"Foto de Perfil\" class=\"profile-pic\">
        </div>
        <div class=\"info\">
            <label for=\"username\">Nome de Usuário:</label>
            <span id=\"username\">{}</span>
        </div>
        <div class=\"info\">
            <label for=\"favorite-fruit\">Fruta Favorita:</label>
            <span id=\"favorite-fruit\">{}</span>
        </div>
        <div class=\"info\">
            <label for=\"favorite-language\">Linguagem de Programação Favorita:</label>
            <span id=\"favorite-language\">{}</span>
        </div>
    </div>
</body>
</html>
",
        user.profile_picture, user.nome, user.favorite_fruit, user.favorite_language
    )
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![login, authenticate, home])
}
