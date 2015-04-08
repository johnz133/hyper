extern crate hyper;

use std::io::Read;

use hyper::Client;
use hyper::header::Connection;
use hyper::header::ConnectionOption;

fn main() {
    // Create a client.
    let mut client = Client::new();

    // Creating an outgoing request.
    let mut response = client.get("http://www.google.com/")  //this returns a request builder
        // set a header
        // .header(Connection(vec![ConnectionOption::Close]))
        // let 'er go!
        .send().unwrap();           //asking the request to send itself, then unwrap the response

    // Read the Response as HYPER style.
    // let mut body = String::new();
    // res.read_to_string(&mut body).unwrap();

    // println!("Response: {}", body);

    assert_eq!(response.status_code().unwrap(), 200);
    assert_eq!(response.stream_id, 1);
    // Dump the headers and the response body to stdout.
    // They are returned as raw bytes for the user to do as they please.
    // (Note: in general directly decoding assuming a utf8 encoding might not
    // always work -- this is meant as a simple example that shows that the
    // response is well formed.)
    for header in response.headers.iter() {
    println!("{}: {}",
    str::from_utf8(&header.0).unwrap(),
    str::from_utf8(&header.1).unwrap());
    }
    println!("{}", str::from_utf8(&response.body).unwrap());
}
