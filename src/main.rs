use actix_web::{get, web, App, HttpServer, Responder};
use tera::{Tera, Context};

#[get("/")]
async fn index() -> impl Responder {
	let terra = match Tera::new("templates/**/*"){
		Ok(t) => t,
		Err(e) => {
			eprintln!("Parsing error(s): {}", e);
			::std::process::exit(1);
		}
	};

	// Context for the data to put in
	let mut context = Context::new();
	context.insert("hello", "HelloWorld!");

	// Renders the template
	let rendered = tera.render("index.html", &context).unwrap();

	web::HttpResponse::Ok()
		.content_type("text/html")
		.body(rendered)

}

#[actix_web::main]
fn main() -> std::io::Resutl<()> {
	HttpServer::new(|| App::new().service(index))
		.bind("127.0.0.1:8080")?
		.run()
		.await
}