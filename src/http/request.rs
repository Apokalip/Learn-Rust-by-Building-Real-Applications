use super::method::{Method,MethodError};
use super::QueryString;

use std::convert::TryFrom;
use std::error::Error;
use std::str::Utf8Error;
use std::fmt::{Display, Debug, Formatter, Result as FmtResult};
use std::str;

#[derive(Debug)]
pub struct Request<'buf> {
	path: &'buf str,
	query_string: Option<QueryString<'buf>>,
	method: Method,
}

impl<'buf> Request<'buf> {
	pub fn path(&self) -> &str {
		&self.path
	}

	pub fn method(&self) -> &Method {
		&self.method
	}

	pub fn query_string(&self) -> Option<&QueryString>{
		self.query_string.as_ref()
	}
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf>{
	type Error = ParseError;
	
	// GET /search?name=abc&sort=1 HTTP/1.1\r\n...HEADERS...
	fn try_from(buf: &'buf [u8]) -> Result<Self, Self::Error>{
		let request = str::from_utf8(buf)?;
		
		match get_next_word(request) {
			Some((method, request)) =>{},
			None => return Err(ParseError::InvalidRequest),
		}

		let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
		let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
		//ignore rest of request as we are not interested in it
		let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

		//protocol version check
		if protocol != "HTTP/1.1" {
			return Err(ParseError::InvalidProtocol);
		}

		let method: Method = method.parse()?;

		let mut query_string = None;
		if let Some(i) = path.find('?'){
			query_string = Some(QueryString::from(&path[i + 1.. ]));
			//remove everything after the query
			path = &path[..i];
		}

		Ok(Self {
			path,
			query_string,
			method
		})
	}
}

// returns a tuple(word, leftover string)
fn get_next_word(request: &str) -> Option<(&str, &str)> {

	for (i, c) in request.chars().enumerate(){
		if c == ' ' || c == '\r' {
			return Some((&request[..i],&request[i+1..]));
		}
	}

	return None;
}

pub enum ParseError {
	InvalidRequest,
	InvalidEncoding,
	InvalidProtocol,
	InvalidMethod,
}

impl ParseError {
	fn message(&self) -> &str{
		match self{
			Self::InvalidRequest => "Invalid Request",
			Self::InvalidEncoding => "Invalid Encoding",
			Self::InvalidProtocol => "Invalid Protocol",
			Self::InvalidMethod => "Invalid Method",
		}
	}
}

impl From<MethodError> for ParseError{
	fn from(_: MethodError) -> Self{
		Self::InvalidMethod
	}
}

impl From<Utf8Error> for ParseError{
	fn from(_: Utf8Error) -> Self{
		Self::InvalidEncoding
	}
}

impl Display for ParseError{
	fn fmt(&self, f: &mut Formatter) -> FmtResult{
		write!(f, "{}", self.message())
	}
}

impl Debug for ParseError{
	fn fmt(&self, f: &mut Formatter) -> FmtResult{
		write!(f, "Debug : {}", self.message())
	}
}

impl Error for ParseError{}


