extern crate actix_web;
#[macro_use]
extern crate prometheus;
#[macro_use]
extern crate lazy_static;

use actix_web::middleware::Middleware;
use actix_web::middleware::Response;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::middleware::Started;
use actix_web::middleware::Finished;
use actix_web::Result;

use prometheus::{IntCounter, Counter};
use prometheus::core::Collector;

use std::sync::Mutex;
use std::sync::Arc;
use std::collections::HashMap;


lazy_static!{
static ref Collectors : Mutex<HashMap<&'static str, Box<IntCounter>>> = {
        let mut m = HashMap::new();
        //m.insert("test_metric", Arc::from(register_int_counter!("hit_count", "hit count help").unwrap()));
        Mutex::from(m)
    };
}

pub struct PrometheusMiddleware;

impl Default for PrometheusMiddleware{
    fn default() -> Self {
        PrometheusMiddleware{}
    }
}

impl PrometheusMiddleware{
    pub fn register_int_counter(&self ){
        Collectors.lock().unwrap().insert("hit_count",Box::from(register_int_counter!("hit_count", "hit count help").unwrap()));
    }
}


impl<S> Middleware<S> for PrometheusMiddleware{

    fn start(&self, req: &HttpRequest<S>) -> Result<Started>{
        println!("protheus middleware started");
        Ok(Started::Done)
    }

    fn response(&self, req: &HttpRequest<S>, resp: HttpResponse) -> Result<Response>{
        println!("protheus middleware response");
        Ok(Response::Done(resp))
    }

    fn finish(&self, req: &HttpRequest<S>, resp: &HttpResponse) -> Finished{
        println!("protheus middleware finished");
        Finished::Done
    }
}