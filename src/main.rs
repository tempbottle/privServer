extern crate hyper;
extern crate openssl;

use std::io::Write;
use std::path::Path;

use hyper::Server;
use hyper::server::Request;
use hyper::server::Response;
use hyper::net::Fresh;

//use openssl::ssl::{Ssl, SslStream, SslContext, SSL_VERIFY_NONE};
use openssl::ssl::{SslContext, SSL_VERIFY_NONE};
use openssl::ssl::SslMethod::Sslv23;
//use openssl::ssl::SslMethod::Tlsv1_2;
//use openssl::ssl::error::StreamError as SslIoError;
use openssl::x509::X509FileType;


fn hello(_: Request, res: Response<Fresh>) {
    let mut res = res.start().unwrap();
    res.write_all(b"Hello World!").unwrap();
    res.end().unwrap();
}

fn main() {
    //Server::http(hello).listen("0.0.0.0:4433").unwrap();

    let cert = Path::new("server.crt");
    let key = Path::new("server.key");

    let mut ssl_context = SslContext::new(Sslv23).unwrap();
    //try!(ssl_context.set_cipher_list("DEFAULT"));
    //try!(ssl_context.set_certificate_file(cert, X509FileType::PEM));
    //try!(ssl_context.set_private_key_file(key, X509FileType::PEM));
    //ssl_context.set_verify(SSL_VERIFY_NONE, None);
    //if let Ok(mut ssl_context) = ssl_context {
        ssl_context.set_cipher_list("HIGH").unwrap();
        ssl_context.set_certificate_file(cert, X509FileType::PEM).unwrap();
        ssl_context.set_private_key_file(key, X509FileType::PEM).unwrap();
        ssl_context.set_verify(SSL_VERIFY_NONE, None);
    //}

    //Server::https(hello, Path::new("server.crt"), Path::new("server.key"))
    //    .listen("127.0.0.1:3000")
    //    .unwrap();
    Server::https_with_context(hello, ssl_context)
        .listen("127.0.0.1:4433")
        //.listen("192.168.0.19:80")
        .unwrap();
}
