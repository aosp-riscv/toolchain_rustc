use jsonrpc_core::*;
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
use serde::Deserialize;
=======
use serde_derive::Deserialize;
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)

#[derive(Deserialize)]
struct HelloParams {
	name: String,
}

fn main() {
	let mut io = IoHandler::new();

	io.add_method("say_hello", |params: Params| {
		let parsed: HelloParams = params.parse().unwrap();
		Ok(Value::String(format!("hello, {}", parsed.name)))
	});

	let request = r#"{"jsonrpc": "2.0", "method": "say_hello", "params": { "name": "world" }, "id": 1}"#;
	let response = r#"{"jsonrpc":"2.0","result":"hello, world","id":1}"#;

	assert_eq!(io.handle_request_sync(request), Some(response.to_owned()));
}
