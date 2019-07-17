extern crate actix;
extern crate actix_prometheus;
extern crate actix_web;

use actix_prometheus::register_default_counters;
use actix_prometheus::PrometheusTransform;
use actix_web::{http, web, HttpResponse};
use actix_web::{App, HttpServer};

fn main() {
    println!("Running prometheus example");

    register_default_counters();
    HttpServer::new(|| {
        App::new().wrap(PrometheusTransform).service(
            web::resource("/test").route(web::get().to(|| HttpResponse::Ok().body("Well done!"))),
            
        ).route("/metrics", web::get().to(actix_prometheus::metric_export))
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap()
}
