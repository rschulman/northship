extern crate websocket;
extern crate openssl;
extern crate byteorder;
//extern crate gst;

use std::thread;
use std::path::Path;
use websocket::{Server, Message, Receiver};
use websocket::message::Type;
use openssl::ssl::{SslContext, SslMethod};
use openssl::x509::X509FileType;
use byteorder::{BigEndian, WriteBytesExt};

// struct Connection {
//     sdp: String,
//     gst_pipeline: String,
//     pipeline: gst::Pipeline,
// }

fn main() {
    let mut context = SslContext::new(SslMethod::Tlsv1).unwrap();
    let _ = context.set_certificate_file(&(Path::new("/etc/letsencrypt/live/rbs.io/fullchain.pem")), X509FileType::PEM);
    let _ = context.set_private_key_file(&(Path::new("/etc/letsencrypt/live/rbs.io/privkey.pem")), X509FileType::PEM);
    let server = Server::bind_secure("0.0.0.0:8080", &context).unwrap();

    //gst::init();

    println!("Server running...");

    for connection in server {
        thread::spawn(move || {
            println!("New connection, processing...");
            let request = connection.unwrap().read_request().unwrap(); // Get the request
            request.validate().unwrap();
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
                        println!("{}", String::from_utf8_lossy(&message.payload));
                        // // Decode JSON into something usable.
                        // // If type: sdp -> parse it into something gstreamer can use, send answer
                        // // If type: candidate -> Add candidate to list, try until connection?
                        // let deserialized_msg: Value = serde_json::from_str(&message.payload).unwrap();
                        // //TODO: Do we need to error check the above?
                        // match deserialized_msg.pointer("/is") {
                        //     Some(type_val) => {
                        //         if type_val.is_string() {
                        //             let type_val = type_val.as_str().unwrap();
                        //             if type_val == "candidate" {
                        //                 process_candidate(deserialized_msg.pointer("/candidate").unwrap());
                        //             } else if type_val == "sdp" {
                        //                 process_sdp(deserialized_msg.pointer("/sdp").unwrap());
                        //             }
                        //         }
                        //     },
                        //     None() => println!("Got a misformed JSON message.");
                        // }
                        // // Ok, what else?
                        // // process_sdp() has to eventually send an answer of some kind.
                        // // and process_candidate has to populate some sort of candidate object?


                    }

                    _ => println!("Got other message.")
                }
            }
        });
    }
}
