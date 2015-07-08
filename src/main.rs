#![allow(unused)]

extern crate hyper;
extern crate openssl;

mod wfs; // web file system

use std::io::Write;
use std::path::Path;
use std::fs::File;
use std::io::Read;

use hyper::Server;
use hyper::server::Request;
use hyper::server::Response;
use hyper::net::Fresh;

use openssl::ssl::{SslContext, SSL_VERIFY_NONE};
use openssl::ssl::SslMethod::Sslv23;
use openssl::x509::X509FileType;


fn accept_callback(req: Request, res: Response<Fresh>) {

    use hyper::method::Method::*;

    println!("{:?}\n{:?}\n{:?}\n{:?}\n{:?}", req.remote_addr, req.method, req.headers, req.uri, req.version);

    match req.method {

        Get => {

        }
        Post => {

        }
        _ => {}

    }

    let ws = wfs::WebFs::new(Path::new("website"));

    let mut res = res.start().unwrap();

    if let hyper::uri::RequestUri::AbsolutePath(uri) = req.uri {

        if uri == "/" {

            match ws.map.get("/index.html") {
                Some(&wfs::WebFile::Bin(ref buf)) => res.write_all(buf.as_ref()).unwrap(),
                _   =>  {}
            }

        }
        else {

            match ws.map.get(&uri) {
                Some(&wfs::WebFile::Bin(ref buf)) => res.write_all(buf.as_ref()).unwrap(),
                _   =>  {}
            }

        }

    }

    res.end().unwrap();
}

fn main() {

    println!("==============");

    let cert = Path::new("server.crt");
    let key = Path::new("server.key");

    let mut ssl_context = SslContext::new(Sslv23).unwrap();
    //ssl_context.set_cipher_list("ECDHE-RSA-AES128-GCM-SHA256:!SHA1:!MD5:!aNULL:!EDH").unwrap();
    ssl_context.set_cipher_list("HIGH").unwrap();
    ssl_context.set_certificate_file(cert, X509FileType::PEM).unwrap();
    ssl_context.set_private_key_file(key, X509FileType::PEM).unwrap();
    ssl_context.set_verify(SSL_VERIFY_NONE, None);
    //ssl_context.set_options(openssl::ssl::SSL_OP_CIPHER_SERVER_PREFERENCE);

    Server::https_with_context(accept_callback, ssl_context)
        .listen("127.0.0.1:4433")
        .unwrap();

}
