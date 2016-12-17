mod connection;

struct Connection {
    offer: String,
    mycandidates: Vec<Candidate>,
    clientcandidates: Vec<Candidate>,
}

impl Connection {
    pub fn new () {
        Connection { offer: "".to_string(), mycandidates: Vec::new(), clientcandidates: Vec::new() }
    }

    pub fn run (&self, connection: Connection) {
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
                    // Decode JSON into something usable.
                    // If type: sdp -> parse it into something gstreamer can use, send answer
                    // If type: candidate -> Add candidate to list, try until connection?
                    let deserialized_msg: Value = serde_json::from_str(&message.payload).unwrap();
                    //TODO: Do we need to error check the above?
                    match deserialized_msg.pointer("/is") {
                        Some(type_val) => {
                            if type_val.is_string() {
                                let type_val = type_val.as_str().unwrap();
                                if type_val == "candidate" {
                                    self.process_candidate(deserialized_msg.pointer("/candidate").unwrap());
                                } else if type_val == "sdp" {
                                    self.process_sdp(deserialized_msg.pointer("/sdp").unwrap());
                                }
                            }
                        },
                        None() => println!("Got a misformed JSON message.");
                    }
                    // Ok, what else?
                    // process_sdp() has to eventually send an answer of some kind.
                    // and process_candidate has to populate some sort of candidate object?
                    

                }

                _ => println!("Got other message.")
            }
        }

        fn process_candidate(candidate_string: String) {

    }
}

struct Candidate {
    foundation: String, // Is it, or can it be an int?
    compID: u8,
    protocol: NetworkType,
    priority: u32,
    address: SocketAddr,
    conn_type: ICEMethod,
    related_address: Option<SocketAddr>
}


