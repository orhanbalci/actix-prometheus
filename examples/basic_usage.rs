extern crate actix;
extern crate actix_web;
extern crate actix_prometheus;


use actix_web::{server, App};
use actix_prometheus::PrometheusMiddleware;


fn main() {
    println!("Running prometheus example");
    let sys = actix::System::new("prometheus-example");

    let _addr = server::new(|| {
        App::new()
            .middleware(PrometheusMiddleware::default())
            .resource("/", |r| {
                r.f(|_| "Hello, middleware! Check the console where the server is run.")
            })
    }).bind("127.0.0.1:8080")
        .unwrap()
        .start();

    let _ = sys.run();
}