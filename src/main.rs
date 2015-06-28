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

    // test sending a picture
    //let pic = Path::new("favicon.ico");

    if let hyper::uri::RequestUri::AbsolutePath(uri) = req.uri {

        if uri == "/favicon.ico" {
            let mut file = std::fs::File::open("favicon.ico").unwrap();
            let mut buf = Vec::new();
            file.read_to_end(&mut buf).unwrap();

            println!("{}", buf.len());

            let mut res = res.start().unwrap();
            res.write_all(buf.as_ref()).unwrap();
            res.end().unwrap();
        }
        else {
            let mut res = res.start().unwrap();
            res.write_all("ウィキペディアへようこそ".to_string().as_bytes()).unwrap();
            //res.write_all(file.).unwrap();
            res.end().unwrap();
        }
    }
}

fn main() {

    // need to do something where i load all the website files in to a data structure for quick reference
    let p = Path::new("deps");

    if let Ok(dir_i) = std::fs::read_dir(p) {
        for file in dir_i {
            if let Ok(file) = file {
                println!("{:?}", file.path());
            }
        }
    }
    else{
        println!("deps dir does not exists");
    }

    println!("==============");

    //test success
    let web_dir = wfs::DirNode::new(Path::new("website"));

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
