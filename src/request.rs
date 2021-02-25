use bytes::Bytes;
//use url::Url;
/// The struct for HTTP Request
#[derive(Debug)]
pub struct HTTPRequest {
    /// HTTP method of this request. Generally, it can be GET/POST/PUT...
    ///
    /// See https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods for details.
    pub method: String,
    /// The path of content that is requested.
    pub path: String,
    /// The protocol for this request. For now, only HTTP/1.1 is supported.
    pub protocol: String,
    /// HTTP Headers, represented by key-value pairs
    pub headers: Vec<(String, String)>,
    /// Body of the request
    pub body: Bytes,
    /// Body of the request
    pub is_tunnel: bool,
}

impl HTTPRequest {
    /// Get the value of a specific key in the headers.
    ///
    /// Return `None` if the key isn't found.
    pub fn get_header_value<'a>(&'a self, key: &str) -> Option<&'a str> {
        for (k, v) in &self.headers {
            if k == key {
                return Some(v.as_ref());
            }
        }
        None
    }

    /// Parsing the message of a request to create an HTTPRequest
    pub fn parse_message(buf: &Bytes) -> Option<Self> {
        let mut pointer = 0;
        let mut last_end = 0;
        let mut method = String::new();
        let mut path = String::new();
        let mut protocol = String::new();
        let mut headers = Vec::new();
        let mut is_tunnel = false;

        while pointer + 1 < buf.len() {
            if buf.get(pointer).unwrap() == &('\r' as u8)
                && buf.get(pointer + 1).unwrap() == &('\n' as u8)
            {
                let new_line = String::from_utf8(buf[last_end..pointer].to_vec()).unwrap();
                last_end = pointer + 2;
                pointer = pointer + 2;
                //println!("{}",new_line);
                if new_line.ends_with("HTTP/1.1") {
                    let items: Vec<&str> = new_line.split(' ').collect();
                    method = String::from(*items.get(0).unwrap());
                    path = String::from(*items.get(1).unwrap());
                    protocol = String::from(*items.get(2).unwrap());
                } else if new_line.len() == 0 {
                    break;
                } else if new_line.starts_with("CONNECT") {
                    let items: Vec<&str> = new_line.split(' ').collect();
                    method = String::from(*items.get(0).unwrap());
                    path = String::from(*items.get(1).unwrap());
                    protocol = String::from(*items.get(2).unwrap());
                } else {
                    if let Some(spliter) = new_line.find(": ") {
                        let key = String::from(&new_line[..spliter]);
                        let value = String::from(&new_line[spliter + 2..]);
                        headers.push((key, value));
                    } else {
                        return None;
                    }
                }
                if new_line.starts_with("CONNECT") {
                    is_tunnel = true;
                } 
            } else {
                pointer = pointer + 1;
            }
        }
        if protocol.len() == 0 {
            return None;
        }
        Some(HTTPRequest {
            method: method,
            path: path,
            protocol: protocol,
            headers: headers,
            body: Bytes::copy_from_slice(&buf[last_end..]),
            is_tunnel: is_tunnel,
        })
    }
}
