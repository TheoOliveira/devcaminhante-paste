#[macro_use] extern crate rocket;


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}

#[get("/")]
fn index() -> &'static str {
    "
    Uso

    POST /
            aceita arquivos raw no corpo do request e responde com  uma url da página contendo o conteudo do body
    
    GET /id
            retorna o conteúdo do paste com o id
    "
}

