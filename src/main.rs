// Include Main use cases
use actix_web::{get, web, HttpResponse, Responder, App, HttpServer};
use std::collections::HashMap;
use std::sync::Mutex;
use std::env;

// Include modules
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
    let args: HashMap<&str, String> = start_config::arg_handler::sort(env::args().collect());

    let mut ip_addr: String = String::new();
    let  workers: usize;

    if args.contains_key("ip_addr") {
        ip_addr.push_str(args["ip_addr"].clone().as_str());
    } else {
        ip_addr.push_str("127.0.0.1");
    }

    ip_addr.push(':');
    
    if args.contains_key("port") {
        ip_addr.push_str(args["port"].clone().as_str());
    }   else {
        ip_addr.push_str("8080");
    }

    if args.contains_key("workers") {
        workers = args["workers"].parse().unwrap();
    } else {
        workers = 6;
    }

    println!("Server Starting.");

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
    .bind(ip_addr.as_str())?
    .workers(workers)
    .run();
    
    println!("Server built successfully on: {:?}", ip_addr);
    
    server.await
}