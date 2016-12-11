extern crate websocket;
extern crate openssl;
extern crate byteorder;
extern crate gst;

use std::thread;
use std::path::Path;
use websocket::{Server, Message, Receiver};
use websocket::message::Type;
use openssl::ssl::{SslContext, SslMethod};
use openssl::x509::X509FileType;
use byteorder::{BigEndian, WriteBytesExt};

fn main() {
    let mut context = SslContext::new(SslMethod::Tlsv1).unwrap();
    let _ = context.set_certificate_file(&(Path::new("/etc/letsencrypt/live/rbs.io/fullchain.pem")), X509FileType::PEM);
    let _ = context.set_private_key_file(&(Path::new("/etc/letsencrypt/live/rbs.io/privkey.pem")), X509FileType::PEM);
    let server = Server::bind_secure("0.0.0.0:8080", &context).unwrap();

    gst::init();

    println!("Server running...");
    
    for connection in server {
        thread::spawn(move || {
            let request = connection.unwrap().read_request().unwrap(); // Get the request
            let response = request.accept(); // Form a response
            let mut client = response.send().unwrap(); // Send the response
            let (mut sender, mut receiver) = client.split();

            for message in receiver.incoming_messages() {
                let message: Message = message.unwrap();

                match message.opcode {
                    Type::Binary => {
                        println!("Got binary blob.");
                    }

                    Type::Text => {
                        println!("{:?}", message.payload);
                    }

                    _ => println!("Got other message.")
                }
            }
        });
    }
}
