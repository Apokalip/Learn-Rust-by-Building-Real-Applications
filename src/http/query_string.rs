use std::collections::HashMap;

// a=1&b=2&c&d=&e===&d=7&d=abc

#[derive(Debug)]
pub struct QueryString<'buf> {
	data: HashMap<&'buf str, Value<'buf>>,
} 

#[derive(Debug)]
pub enum Value<'buf> {
	Single(&'buf str),
	Multiple(Vec<&'buf str>),
}

impl<'buf> QueryString<'buf> {
	 pub fn get(&self, key: &str) -> Option<&Value>{
		 self.data.get(key)
	 }
}

impl<'buf> From<&'buf str> for QueryString<'buf>{
	fn from(s: &'buf str) -> Self{

		let mut data = HashMap::new();

		for sub_str in s.split('&'){
			let mut key = sub_str;
			let mut val = "";
			
			// if there is a value 
			if let Some(i) = sub_str.find('='){
				key = &sub_str[..i];
				val = &sub_str[i + 1..];
			}

			// if key exist, change it into multiple or just add in to the existing vector
			data.entry(key).and_modify(|existing: &mut Value| match existing {
				Value::Single(prev) => {
					*existing = Value::Multiple(vec![prev,val]);
				}
				Value::Multiple(vec) => vec.push(val),
			})
			.or_insert(Value::Single(val));
		}
		
		QueryString {data};
		unimplemented!()
	}
}