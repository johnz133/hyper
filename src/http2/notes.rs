pub enum HttpFrame { //3 frames

pub struct HttpConnection<S> where S: TransportStream {
        pub fn with_stream(stream: S) -> HttpConnection<S> {
        pub fn send_frame<F: Frame>(&mut self, frame: F) -> HttpResult<()> {
        pub fn recv_frame(&mut self) -> HttpResult<HttpFrame> {

pub struct ClientConnection<TS, S>
        where TS: TransportStream, S: Session {
    pub fn new(stream: TS, session: S) -> ClientConnection<TS, S> {
    pub fn with_connection(conn: HttpConnection<TS>, session: S) -> ClientConnection<TS, S> {
    pub fn init(&mut self) -> HttpResult<()> {
    pub fn send_request(&mut self, req: Request) -> HttpResult<()> {
    pub fn handle_next_frame(&mut self) -> HttpResut<()> {

pub type Header // tuple of 2 Vec<u8>

pub struct Response {
    pub fn new(stream_id: StreamId, headers: Vec<Header>, body: Vec<u8>)
            -> Response {
    pub fn status_code(&self) -> HttpResult<u16> {

pub struct Request {
    //StreamId
    //headers
    //body

// from http2/mod.rs
pub type HttpResult<T> = Result<T, HttpError>;

SimpleClient
    //conn ClientConnection/HttpConnection
    //next stream id << Fix?
    //host
    pub fn connect(host: &str, port: u16) -> HttpResult<SimpleClient> {
    pub fn request(&mut self, method: &[u8], path: &[u8], extras: &[Header])
            -> HttpResult<StreamId> {
    pub fn get_response(&mut self, stream_id: StreamId) -> HttpResult<Response> {

// net.rs
pub trait NetworkConnector {
    /// Type of Stream to create
    type Stream: NetworkStream + Send;
    /// Connect to a remote address.
    fn connect(&mut self, host: &str, port: u16, scheme: &str) -> io::Result<Self::Stream>;
}
