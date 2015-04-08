//HYPER
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

//SOLICIT

/// A struct representing a full HTTP/2 request, along with the full body, as a
/// sequence of bytes.
#[derive(Clone)]
pub struct Request {
    pub stream_id: u32, // enum some or none
    pub headers: Vec<Header>, //conform solicit to hyper headers
    pub body: Vec<u8>, //ditto
}
