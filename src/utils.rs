use crate::request::HTTPRequest;
use bytes::{Buf, BufMut, BytesMut};
use tokio::io::{AsyncReadExt};

type ChunkedBuffer = [u8; 4096];


/// Read HTTP request from the socket.
pub async fn read_http_request<Stream>(socket: &mut Stream) -> Option<HTTPRequest>
where
    Stream: tokio::io::AsyncRead + std::marker::Unpin,
{
    let mut buffer = BytesMut::new();
    loop {
        let mut chuncked_buffer: ChunkedBuffer = [0; 4096];
        if let Ok(c_size) = socket.read(&mut chuncked_buffer).await {
            buffer.put(&chuncked_buffer[..c_size]);
            let req_parsed = HTTPRequest::parse_message(&buffer.clone().to_bytes());
            if let Some(req) = req_parsed {
                if let Some(size_str) = &req.get_header_value("Content-Length") {
                    if size_str.parse::<usize>().unwrap() == req.body.len() {
                        return Some(req);
                    }
                } else {
                    // no content length found, directly return the valid request
                    break;
                }
            }
            if c_size == 0 {
                break;
            }
        } else {
            break;
        }
    }

    HTTPRequest::parse_message(&buffer.to_bytes())
}