extern crate websocket;
extern crate openssl;
extern crate serde_json;
//extern crate tensorflow;
extern crate chrono;
//extern crate gst;

use std::thread;
use std::sync::mpsc::channel;
use std::path::Path;
use websocket::{Server, Message, Sender, Receiver};
use websocket::message::Type;
use openssl::ssl::{SslContext, SslMethod};
use openssl::x509::X509FileType;
use serde_json::{Map, Value};

mod logic;

// struct Connection {
//     sdp: String,
//     gst_pipeline: String,
//     pipeline: gst::Pipeline,
// }

// TODO: Change all printlns to log crate

fn main() {
    let mut context = SslContext::new(SslMethod::Tlsv1).unwrap();
    let _ = context.set_certificate_file(&(Path::new("/etc/letsencrypt/live/rbs.io/fullchain.pem")), X509FileType::PEM);
    let _ = context.set_private_key_file(&(Path::new("/etc/letsencrypt/live/rbs.io/privkey.pem")), X509FileType::PEM);
    let server = Server::bind_secure("0.0.0.0:8080", &context).unwrap();

    //gst::init();

    println!("Server running...");

    for connection in server {
        // TODO: Authentication should happen here.
        thread::spawn(move || {
            println!("New connection, processing...");
            let request = connection.unwrap().read_request().unwrap(); // Get the request
            request.validate().unwrap();
            let response = request.accept(); // Form a response
            let mut client = response.send().unwrap(); // Send the response
            let (mut sender, mut receiver) = client.split();
            let (chan_tx, chan_rx) = channel::<String>();

            thread::spawn(move || {
                loop {
                    // This thread's only job is to take from the channel,
                    // form message into JSON, then send to the client.
                    let message = chan_rx.recv();
                    match message {
                        Ok(send_string) => {
                            let mut send_map = Map::new();
                            send_map.insert("payload", send_string);
                            let response = Message::text(serde_json::to_string(&send_map).unwrap());
                            sender.send_message(&response);
                        }
                        Err(_) => {
                            // Means that the other side of the channel has dropped.
                            // So, all of the channel senders have fallen out of scope.
                            // In this case its safe to assume our job here is done.
                            break;
                        }
                    }
                }
            });

            for message in receiver.incoming_messages() {
                let message: Message = message.unwrap();
    // get offset from client, convert to minutes.
    // Then do UTC.noW().with_timezone()
    //let dt = UTC::now().with_timezone(&FixedOffset::west(offset)); 


                match message.opcode {
                    Type::Binary => {
                        println!("Got binary blob.");
                    },

                    Type::Text => {
                        println!("{}", String::from_utf8_lossy(&message.payload));
                        // Decode the JSON...
                        let deserialized_msg: Value =
                                serde_json::from_str(std::str::from_utf8(&message.payload).unwrap()).unwrap();

                        // First, get the payload of what was said on the other side.
                        match deserialized_msg.pointer("/payload") {
                            Some(speech) => {
                                // Call into some mod for sending to tensorflow...
                            },
                            None => println!("Got an unexpected JSON message. Dropping."),
                        }

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


                    },

                    _ => println!("Got other message.")
                }
            }
        });
    }
}
