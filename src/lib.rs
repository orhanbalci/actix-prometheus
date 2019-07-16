extern crate actix_web;
#[macro_use]
extern crate prometheus;
#[macro_use]
extern crate lazy_static;

extern crate futures;

use actix_web::{dev::Service, dev::Transform};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error};
use futures::future::{ok, FutureResult};
use futures::{Future, Poll};

use prometheus::{Counter, IntCounter, IntCounterVec};

use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref INT_COUNTERS: Mutex<HashMap<&'static str, Box<IntCounter>>> = {
        let m = HashMap::new();
        Mutex::from(m)
    };
    static ref INT_VEC_COUNTERS: Mutex<HashMap<&'static str, Box<IntCounterVec>>> = {
        let m = HashMap::new();
        Mutex::from(m)
    };
}

pub fn register_default_counters() {
    let opts = opts!(
        "request_count",
        "Number of HTTP requests processed. Partitioned by status code and HTTP method"
    );
    INT_VEC_COUNTERS.lock().unwrap().insert(
        "request_count",
        Box::from(register_int_counter_vec!(opts, &["status_code", "http_method"]).unwrap()),
    );
}

pub struct PrometheusTransform;

// Middleware factory is `Transform` trait from actix-service crate
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S> for PrometheusTransform
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = PrometheusMiddleware<S>;
    type Future = FutureResult<Self::Transform, Self::InitError>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(PrometheusMiddleware { service })
    }
}

pub struct PrometheusMiddleware<S> {
    service: S,
}

impl<S> PrometheusMiddleware<S> {
    pub fn register_int_counter(&self) {
        INT_COUNTERS.lock().unwrap().insert(
            "hit_count",
            Box::from(register_int_counter!("hit_count", "hit count help").unwrap()),
        );
    }
}

impl<S, B> Service for PrometheusMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        self.service.poll_ready()
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        println!("Prometheus middleware started");

        Box::new(self.service.call(req).and_then(|res| {
            println!("Prometheus middleware response");
            INT_VEC_COUNTERS
                .lock()
                .unwrap()
                .get("request_count")
                .unwrap()
                .get_metric_with_label_values(&[
                    res.status().as_str(),
                    res.request().method().as_str(),
                ]);
            Ok(res)
        }))
    }
}
