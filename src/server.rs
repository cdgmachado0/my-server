use std::io::Read;
use std::net::TcpListener;

pub struct Server {
    addr: String
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self) {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let a = [1,2,3,3,3,3,4]; 
                    arr(&a[1..3]);
                    
                    // stream.read();
                },
                Err(e) => println!("Connection failed: {}", e)
            }
        }

    }
}