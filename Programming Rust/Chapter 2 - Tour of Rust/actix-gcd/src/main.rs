// Each of the names inside the curly brackets becomes usable
// in our code. That way we dont have to type "actix_web::HttpServer"
// each time we want to use the HttpServer command.
use actix_web::{web, App, HttpResponse, HttpServer};

// Serde: Process the form data
use serde::Deserialize;

// Structure that represents the values we expect from the form
// (The attribute tells Serde to examine the type "GcdParameters" when the program 
//  is compiled and automatically generate code to parse a value of this type
//  from data in the format that HTML forms use for POST requests. In fact, this attribute
//  allows you to parse a GcdParameters value from any type of structured data: JSON, YAML
//  or TOML)
#[derive(Deserialize)]
struct GcdParameters {
    n: u64,
    m: u64,
}

// NOTE: Serde also provides a "Serialize" attribute that generates code to take
// a Rust value and writing it onto a structured format

fn main() {

    // Server that responds to requests for a single path "/". 
    // NOTE: "|| {}" consists of a closure. When we start the server, Actix starts a pool of
    // threads to handle incoming requests. Each thread calls our closure to get a fresh copy
    // of the "App" value that tells it how to route and handle requests.
    let server = HttpServer::new(|| {

        // Each closure call creates a new and empty App. Then, we add a route for the path
        // "/" by calling the route method. The handler for that route (web::get().to(get_index))
        // responds to get requests by calling the function "get_index". Since theres no semicolon
        // at the end of this line, the return value of the closure is the modified "App".
        App::new()
            .route("/", web::get().to(get_index))
            .route("/gcd", web::post().to(post_gcd))

    });

    // Instructions on how to connect to the server
    println!("Serving on http://localhost:3000...");

    // Listen on port 3000 of the local machine
    server
        .bind("127.0.0.1:3000").expect("Error binding server to address")
        .run().expect("Error running server");
}

// GET INDEX
// Builds an HttpResponse value representing the response to a GET request.
fn get_index() -> HttpResponse {

    // We indicate that the request was successful by responding with
    // an HTTP 200 Ok status
    HttpResponse::Ok()

        // We fill the details of the response
        // The return value from body serves as the return value of "get_index"
        .content_type("text/html")
        .body(

            // Text contains a lot of double quotes, so we write it using Rust's
            // "raw string" syntax:
            //   - The letter r
            //   - Zero or more hash marks "#"
            //   - A double quote
            //   - The content of the string
            //   - Another double quote followed by the same number of hash marks "#"
            r#"
                <title>GCD Calculator</title>
                <form action="/gcd" method="post">
                <input type="text" name="n"/>
                <input type="text" name="m"/>
                <button type="submit">Compute GCD</button>
                </form>
            "#,
        )

        // NOTE: Any character can occur inside a raw string without being escaped.
        // This means that no character that starts with "\" will be recognized (eg. \n). 
}

// POST PARAMETERS
// For a function to be an "Actix Request Handler" (The function called when responding
// to a request), its arguments must all have types Actix knows how to extract from an HTTP
// request. In this case the function takes one argument "form", with type GcdParameters.
// Since GcdParameters can be deserialized from POST data (because of the struct's attribute)
// Actix doesnt complain about the output type of the handler. It Actix didnt know how to
// handle the requested output, it will throw an error inmediately.
fn post_gcd(form: web::Form<GcdParameters>) -> HttpResponse {

    // Return an HTTP 401 "bad request" error if either parameter is zero
    // (Since "gcd()" will panic if either argument is zero)
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Computing the GCD with zero is boring");
    }

    // format!: Macro very similar to "println!", except that instead of writing 
    // the result to stdout, it returns it as a string
    let response = 
        format!("The greatest common divisor of the numbers {} and {} is <b>{}</b>\n",
                form.n, form.m, gcd(form.n, form.m));

    // Wraps the response in an HTTP 200 Ok response, adding its type
    HttpResponse::Ok()
        .content_type("text/html")
        .body(response)
}

// CALCULATE GREATEST COMMON DIVISOR
fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}