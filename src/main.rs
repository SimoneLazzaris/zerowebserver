extern crate tiny_http;
use tiny_http::{Server, Response};

fn main() {
    let server = Server::http("0.0.0.0:8000").unwrap();

    for request in server.incoming_requests() {
        let ff = request.headers().iter().find(|&i| i.field=="Host".parse().unwrap());
        let host = &ff.unwrap().value;
        let resp_str=format!("Hello {}\n", host.as_str());
        let response = Response::from_string(resp_str);
        let _result = request.respond(response);
    }
}
