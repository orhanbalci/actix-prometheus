extern crate actix_web;
#[macro_use]
extern crate prometheus;
#[macro_use]
extern crate lazy_static;

use actix_web::middleware::{Finished, Middleware, Response, Started};
use actix_web::{HttpRequest, HttpResponse, Result};

use prometheus::{Counter, IntCounter};

use std::collections::HashMap;
use std::sync::Mutex;

lazy_static!{
static ref Collectors : Mutex<HashMap<&'static str, Box<IntCounter>>> = {
        let m = HashMap::new();
        Mutex::from(m)
    };
}

pub struct PrometheusMiddleware;

impl Default for PrometheusMiddleware {
    fn default() -> Self {
        PrometheusMiddleware {}
    }
}

impl PrometheusMiddleware {
    pub fn register_int_counter(&self) {
        Collectors.lock().unwrap().insert(
            "hit_count",
            Box::from(register_int_counter!("hit_count", "hit count help").unwrap()),
        );
    }
}

impl<S> Middleware<S> for PrometheusMiddleware {
    fn start(&self, req: &HttpRequest<S>) -> Result<Started> {
        println!("Prometheus middleware started");
        Ok(Started::Done)
    }

    fn response(&self, req: &HttpRequest<S>, resp: HttpResponse) -> Result<Response> {
        println!("Prometheus middleware response");
        Ok(Response::Done(resp))
    }

    fn finish(&self, req: &HttpRequest<S>, resp: &HttpResponse) -> Finished {
        println!("Prometheus middleware finished");
        Finished::Done
    }
}
