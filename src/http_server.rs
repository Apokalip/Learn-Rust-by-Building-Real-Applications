use crate::http::{Request, Response, StatusCode, ParseError};
use std::convert::TryFrom;
use std::io::{Read, Write};
use std::net::TcpListener;

pub trait Handler {
	fn handle_request(&mut self, request: &Request) -> Response;

	fn handle_bad_request(&mut self, e: &ParseError) -> Response{
		println!("Failed to parse request: {}", e);
		Response::new(StatusCode::BadRequest, None)
	}
}

pub struct Server{
	address: String,
}

impl Server{
	pub fn new(addr: String) -> Self {
		Self{
			address: addr,
		}
	}

	// let run take ownership as we want to deallocate at the end of run() anywayР
	pub fn run(self, mut handler: impl Handler){
		println!("Listening on {}", self.address);

		let listener = TcpListener::bind(&self.address).unwrap();

		loop {
			match listener.accept() {
				Ok((mut stream, _)) => {
					let mut buffer = [0; 1024];
					match stream.read(&mut buffer) {
						Ok(_) => {
							println!("Received a request: {}", String::from_utf8_lossy(&buffer));
							let response = match Request::try_from(&buffer[..]){
								Ok(request) => {
									handler.handle_request((&request))
								}
								Err(error) => { 
									handler.handle_bad_request(&error)
								}
							};
							
							if let Err(error) = response.send(&mut stream) {
								println!("Failed to send response")
							}
						},
						Err(error) => println!("{}", error)

					}

				},
				Err(error) => println!("{}", error)
			}

		}
	}
}

