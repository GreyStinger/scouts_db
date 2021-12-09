use actix_web::{get, web, HttpResponse, Responder, App, HttpServer};
use std::sync::Mutex;
use std::env;


mod start_config;


struct State {
    app_name: String,
    login_counter: Mutex<i32>
}


#[get("/")]
async fn home(data: web::Data<State>) -> impl Responder {
    let mut counter = data.login_counter.lock().unwrap();

    *counter += 1;

    HttpResponse::Ok().body(format!("<h2>Hello {0}!</h2><p>Logged in {1} times.</p>", &data.app_name, counter))
}


async fn login() -> impl Responder {
    "Login Page."
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server Starting.");

    start_config::arg_to_env::run();

    let ip_addr = format!("{}:{}", env::var("IP_ADDRESS").unwrap(), env::var("PORT").unwrap()); 

    let main_state = web::Data::new(State {
        app_name: String::from("Scout-DB"),
        login_counter: Mutex::new(0)
    });

    let server = HttpServer::new(move || {
        App::new()
            .app_data(main_state.clone())
            .service(home)
            .service(
                web::scope("/app")
                    .route("/login.html", web::get().to(login))
                )
    })
    // Clone the ip_addr so that it doesn't go out of this scope.
    .bind(ip_addr.clone())?
    .workers(env::var("WORKERS").unwrap().parse::<usize>().unwrap())
    .run();
    
    println!("Server built successfully on: {:?}", ip_addr);
    
    server.await
}