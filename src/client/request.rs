//! Client Requests
use std::marker::PhantomData;
use std::io::{self, Write, BufWriter};

use url::Url;

use method::{self, Method};
use header::Headers;
use header::{self, Host};
use net::{NetworkStream, NetworkConnector, HttpConnector, Fresh, Streaming};
use http::{HttpWriter, LINE_ENDING};
use http::HttpWriter::{ThroughWriter, ChunkedWriter, SizedWriter, EmptyWriter};
use version;
use HttpResult;
use client::{Response, get_host_and_port};

///////////////////////SOLICIT////////////
use std::net::TcpStream;

use super::super::http2::connection::ClientConnection;
use super::super::http2::session::{DefaultSession, Stream};
use super::super::http2;
// use super::super::http2::{HttpError};
// use super::super::http2::{StreamId, HttpResult, HttpError, Response, Header, Request};

/////////////////////////////////////////////


/// A client request to a remote server.
pub struct Request<W> {
    /// The target URI for this request.
    pub url: Url,

    /// The HTTP version of this request.
    pub version: version::HttpVersion,

    body: HttpWriter<BufWriter<Box<NetworkStream + Send>>>,
    headers: Headers,
    method: method::Method,

    _marker: PhantomData<W>,
}

impl<W> Request<W> {
    /// Read the Request headers.
    #[inline]
    pub fn headers(&self) -> &Headers { &self.headers }

    /// Read the Request method.
    #[inline]
    pub fn method(&self) -> method::Method { self.method.clone() }
}

pub struct SimpleClient {
    /// The underlying `ClientConnection` that the client uses
    pub conn: ClientConnection<TcpStream, DefaultSession>,
    /// Holds the ID that can be assigned to the next stream to be opened by the
    /// client.
    pub next_stream_id: u32,
    /// Holds the domain name of the host to which the client is connected to.
    pub host: Vec<u8>,
}


impl Request<Fresh> {
    /// Create a new client request.                HttpResult<Request<Fresh>>
    pub fn new(method: method::Method, url: Url) -> HttpResult<http2::Response> {
        // GOAL: create a http/2
        // Input: method and URL
        // If first request (request that starts connection)
            // If true need to do http/2 protocol handshake --> connection preface, in connection.rs ClientConnection init
            // Set up TCP connction and stream identifiers
            // If not, don't worry about it
        //Output: Fresh HTTPResults

        //Pseudo code:
        //first we create a new connection
        //then we ask the connection to initialize with preface
        //then we store the connection somewhere
        //
        //let mut client = SimpleClient::connect("nghttp2.org", 80).ok().unwrap();

        let mut client = SimpleClient {
            conn: ClientConnection::new(
                TcpStream::connect(&("nghttp2.org", 80)).unwrap(),
                DefaultSession::new()),
            next_stream_id: 1,
            host: "nghttp2.org".as_bytes().to_vec(),
        };

        // let mut conn = ClientConnection::new(TcpStream::connect(&("nghttp2.org", 80)).unwrap(), DefaultSession::new());
        // let mut next_stream_id = 1;
        // let mut host = "nghttp2.org".as_bytes().to_vec();
        // let mut conn = SimpleClient.conn;
        // let mut next_stream_id = SimpleClient.next_stream_id;
        // let mut host = SimpleClient.host;
        // try!(conn.init());
        client.conn.init();

        let mut stream_id = client.next_stream_id;
        client.next_stream_id += 2;

        client.conn.session.new_stream(stream_id);

        //creating the get request
        let scheme = b"http".to_vec();
        let mut headers: Vec<http2::Header> = vec![
            (b":method".to_vec(), b"GET".to_vec()),
            (b":path".to_vec(), b"/".to_vec()),
            (b":authority".to_vec(), client.host.clone()),
            (b":scheme".to_vec(), scheme),
        ];

        let request = http2::Request {
                        stream_id: stream_id,
                        headers: headers,
                        body: Vec::new(),
                    };

        //no need to add extra headers for now
        // headers.extend(extras.to_vec().into_iter());
        //
        // try!(conn.send_request(request));
        client.conn.send_request(request);
                // let response:HttpResult<Response> = client.get_response(stream_id);

                // pub fn get_response(&mut self, stream_id: StreamId) -> HttpResult<Response> {
                //     match self.conn.session.get_stream(stream_id) {
                //         None => return Err(HttpError::UnknownStreamId),
                //         Some(_) => {},
                //     };
                //     loop {
                //         if let Some(stream) = self.conn.session.get_stream(stream_id) {
                //             if stream.is_closed() {
                //                 return Ok(Response {
                //                     stream_id: stream.id(),
                //                     headers: stream.headers.clone().unwrap(),
                //                     body: stream.body.clone(),
                //                 });
                //             }
                //         }
                //         try!(self.handle_next_frame());
                //     }
                // }
        let mut response:HttpResult<http2::Response>;
        loop {
            if let Some(stream) = client.conn.session.get_stream(stream_id) {
                if stream.is_closed(){
                    response = Ok(http2::Response{
                        stream_id: stream.id(),
                        headers: stream.headers.clone().unwrap(),
                        body: stream.body.clone(),
                    });
                    break;
                }
            }
            // try!(client.conn.handle_next_frame());
            client.conn.handle_next_frame();
        }
        // println!("123");
        return response;
// //SOLICIT STYLE REQUEST
        // Request {
        //     stream_id: stream_id,
        //     headers: headers,
        //     body: Vec::new(),
        // }

        // //HYPER STYLE REQUEST
        // Request {
        //     method: method,
        //    headers: headers,
        //     url: url,
        //     streamid?
        //     version: version::HttpVersion::Http11,
        //     body: stream,
        //     _marker: PhantomData,
        // }


        //GET REQUEST
            //let response = client.get(b"/", &[]).unwrap();
            // pub fn get(&mut self, path: &[u8], extra_headers: &[Header])
            //     -> HttpResult<Response> {
            //     let stream_id = try!(self.request(b"GET", path, extra_headers));
            //     self.get_response(stream_id)
            //
        // let stream_id = try!(client.request(b"GET", b"/", &[]));
        // //Defining client.request:
        //                 pub fn request(&mut self, method: &[u8], path: &[u8], extras: &[Header])
        //                     -> HttpResult<StreamId> {
        //                     let stream_id = self.new_stream(); //set stream id
        //                     // Only http supported for now...
        //                     let scheme = b"http".to_vec();  // set the scheme
        //                     let host = self.host.clone();   //set the host
        //                     let mut headers: Vec<Header> = vec![  //set the header
        //                         (b":method".to_vec(), method.to_vec()),
        //                         (b":path".to_vec(), path.to_vec()),
        //                         (b":authority".to_vec(), host),
        //                         (b":scheme".to_vec(), scheme),
        //                     ];
        //                     headers.extend(extras.to_vec().into_iter());  //add some more headers

        //                     try!(self.conn.send_request(Request {
        //                         stream_id: stream_id,
        //                         headers: headers,
        //                         body: Vec::new(),
        //                     }));

        //                     Ok(stream_id)
        //                 }
            //the meat of client.request
            // let request = Request {
            //             stream_id: stream_id,
            //             headers: headers,
            //             body: Vec::new(),
            //         };

        //this is gonna be hyper's client.send, referring to hyperClient.rs
        //     try!(client.conn.send_request(request));
        // let response:HttpResult<Response> = client.get_response(stream_id);

        //TADA


        // Ok(client)


        //Parse the request and if its http2, use with connecter for http2

        //return this.
        //this will be casted into a HttpResult<Request<Fresh>
        // Ok(Request {
        //     method: method,
        //     // headers: headers,
        //     url: url,
        //     // streamid?
        //     // version: version::HttpVersion::Http11,
        //     // body: stream,
        //     // _marker: PhantomData,
        // })
        // HYPER ORIGINAL
        // TODO: Implement Router for Http1 > 2

        // import connect.rs into this file
        // parse the outgoing request
        // if it's of type http2. we use solict streams
        // pass it off to http2_connector

        // let mut conn = HttpConnector(None);
        // Request::with_connector(method, url, &mut conn)
    }

    /// Create a new client request with a specific underlying NetworkStream.
    pub fn with_connector<C, S>(method: method::Method, url: Url, connector: &mut C)
        -> HttpResult<Request<Fresh>> where
        C: NetworkConnector<Stream=S>,
        S: NetworkStream + Send {
        debug!("{} {}", method, url);
        let (host, port) = try!(get_host_and_port(&url));

        let stream = try!(connector.connect(&*host, port, &*url.scheme));//destructures into http/https
        // FIXME: Use Type ascription
        let stream: Box<NetworkStream + Send> = Box::new(stream);
        let stream = ThroughWriter(BufWriter::new(stream));

        let mut headers = Headers::new();
        headers.set(Host {
            hostname: host,
            port: Some(port),
        });

        Ok(Request {
            method: method,
            headers: headers,
            url: url,
            version: version::HttpVersion::Http20,
            body: stream,
            _marker: PhantomData,
        })
    }

    /// Consume a Fresh Request, writing the headers and method,
    /// returning a Streaming Request.
    pub fn start(mut self) -> HttpResult<Request<Streaming>> {
        let mut uri = self.url.serialize_path().unwrap();
        //TODO: this needs a test
        if let Some(ref q) = self.url.query {
            uri.push('?');
            uri.push_str(&q[..]);
        }

        debug!("writing head: {:?} {:?} {:?}", self.method, uri, self.version);
        try!(write!(&mut self.body, "{} {} {}{}",
                    self.method, uri, self.version, LINE_ENDING));


        let stream = match self.method {
            Method::Get | Method::Head => {
                debug!("headers [\n{:?}]", self.headers);
                try!(write!(&mut self.body, "{}{}", self.headers, LINE_ENDING));
                EmptyWriter(self.body.into_inner())
            },
            _ => {
                let mut chunked = true;
                let mut len = 0;

                match self.headers.get::<header::ContentLength>() {
                    Some(cl) => {
                        chunked = false;
                        len = **cl;
                    },
                    None => ()
                };

                // cant do in match above, thanks borrowck
                if chunked {
                    let encodings = match self.headers.get_mut::<header::TransferEncoding>() {
                        Some(&mut header::TransferEncoding(ref mut encodings)) => {
                            //TODO: check if chunked is already in encodings. use HashSet?
                            encodings.push(header::Encoding::Chunked);
                            false
                        },
                        None => true
                    };

                    if encodings {
                        self.headers.set::<header::TransferEncoding>(
                            header::TransferEncoding(vec![header::Encoding::Chunked]))
                    }
                }

                debug!("headers [\n{:?}]", self.headers);
                try!(write!(&mut self.body, "{}{}", self.headers, LINE_ENDING));

                if chunked {
                    ChunkedWriter(self.body.into_inner())
                } else {
                    SizedWriter(self.body.into_inner(), len)
                }
            }
        };

        Ok(Request {
            method: self.method,
            headers: self.headers,
            url: self.url,
            version: self.version,
            body: stream,
            _marker: PhantomData,
        })
    }

    /// Get a mutable reference to the Request headers.
    #[inline]
    pub fn headers_mut(&mut self) -> &mut Headers { &mut self.headers }
}

impl Request<Streaming> {
    /// Completes writing the request, and returns a response to read from.
    ///
    /// Consumes the Request.
    pub fn send(self) -> HttpResult<Response> {
        let raw = try!(self.body.end()).into_inner().unwrap(); // end() already flushes
        Response::new(raw)
    }
}

impl Write for Request<Streaming> {
    #[inline]
    fn write(&mut self, msg: &[u8]) -> io::Result<usize> {
        self.body.write(msg)
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        self.body.flush()
    }
}

#[cfg(test)]
mod tests {
    use std::str::from_utf8;
    use url::Url;
    use method::Method::{Get, Head};
    use mock::{MockStream, MockConnector};
    use super::Request;

    #[test]
    fn test_get_empty_body() {
        let req = Request::with_connector(
            Get, Url::parse("http://example.dom").unwrap(), &mut MockConnector
        ).unwrap();
        let req = req.start().unwrap();
        let stream = *req.body.end().unwrap()
            .into_inner().unwrap().downcast::<MockStream>().ok().unwrap();
        let bytes = stream.write;
        let s = from_utf8(&bytes[..]).unwrap();
        assert!(!s.contains("Content-Length:"));
        assert!(!s.contains("Transfer-Encoding:"));
    }

    #[test]
    fn test_head_empty_body() {
        let req = Request::with_connector(
            Head, Url::parse("http://example.dom").unwrap(), &mut MockConnector
        ).unwrap();
        let req = req.start().unwrap();
        let stream = *req.body.end().unwrap()
            .into_inner().unwrap().downcast::<MockStream>().ok().unwrap();
        let bytes = stream.write;
        let s = from_utf8(&bytes[..]).unwrap();
        assert!(!s.contains("Content-Length:"));
        assert!(!s.contains("Transfer-Encoding:"));
    }
}
