include!("xor_encrypt.rs");

const KEY_STRING: &str = "1357924680";

use std::io::prelude::*;
use std::net::{Shutdown, TcpStream};

fn bit_of_num(num: i32) -> i32 {
	let mut len: i32 = 0;
	let mut num = num;
	while num != 0 {
		num /= 10;
		len += 1;
	}

	return len;
}

fn accurate_len(len: i32) -> i32 {
	let mut len_of_content = len - 1;

	if len_of_content == 0 {
		return 1;
	}
	// Special number, next power of 10
	let special_num = 10.0f64.powi(bit_of_num(len_of_content)) as i32;
	// bits of special number
	let bits_of_special_num = bit_of_num(special_num);
	let min_number_changed_bits = special_num - bits_of_special_num + (bits_of_special_num % 2);
	let mut actual_bit = bit_of_num(len_of_content);
	if len_of_content >= min_number_changed_bits && len_of_content < special_num {
		actual_bit += 1;
	}
	len_of_content += actual_bit;

	return len_of_content;
}

fn request(stream: &mut TcpStream, request_jobject: &mut json::JsonValue) {
	let request_json = request_jobject.dump();
	let request_json_len = request_json.len() as i32;
	request_jobject
		.insert("length", accurate_len(request_json_len))
		.unwrap();
	let request_json = request_jobject.dump();

	let key_str = String::from(KEY_STRING);
	let key = key_str.into_bytes();
	let encrypt_data = encrypt(&request_json.into_bytes(), &key);
	stream.write(&encrypt_data.as_slice()).unwrap();
}

fn get_response(stream: &mut TcpStream) -> json::JsonValue {
	let mut encrypted_data: Vec<u8> = Vec::new();
	let is_end = false;
	let mut buffer = [0; 4096];
	while !is_end {
		match stream.read(&mut buffer) {
			Ok(count) => {
				if count == 0 {
					break;
				} else {
					for i in 0..count {
						encrypted_data.push(buffer[i]);
					}
				}
			}
			Err(err) => {
				eprintln!("{:?}", err);
				return json::object! {};
			}
		}
	}

	let key_str = String::from(KEY_STRING);
	let key = key_str.into_bytes();
	let original_data = decrypt(&encrypted_data, &key);
	let json_string = String::from_utf8(original_data).unwrap();

	return json::parse(&*json_string).unwrap();
}

fn call_remote_server(
	stream: &mut TcpStream,
	request_jobject: &mut json::JsonValue,
) -> json::JsonValue {
	request(stream, request_jobject);
	stream.shutdown(Shutdown::Write).unwrap();

	return get_response(stream);
}

fn register_user(arg_username: &str, arg_password: &str) -> json::JsonValue {
	let mut stream = TcpStream::connect("127.0.0.1:13579").unwrap();

	let mut request_jobject = json::object! {
		length: 0,
		request: {
			func_name: "register_user",
			arguments: {
				username: arg_username,
				password: arg_password,
			}
		},
	};

	return call_remote_server(&mut stream, &mut request_jobject);
}

fn login(arg_username: &str, arg_password: &str) -> json::JsonValue {
	let mut stream = TcpStream::connect("127.0.0.1:13579").unwrap();

	let mut request_jobject = json::object! {
		length: 0,
		request: {
			func_name: "login",
			arguments: {
				username: arg_username,
				password: arg_password,
			}
		},
	};

	return call_remote_server(&mut stream, &mut request_jobject);
}

fn modify_password(
	arg_token_id: &str,
	arg_old_password: &str,
	arg_new_password: &str,
) -> json::JsonValue {
	let mut stream = TcpStream::connect("127.0.0.1:13579").unwrap();

	let mut request_jobject = json::object! {
		length: 0,
		request: {
			func_name: "modify_password",
			arguments: {
				token_id: arg_token_id,
				old_password: arg_old_password,
				new_password: arg_new_password,
			}
		},
	};

	return call_remote_server(&mut stream, &mut request_jobject);
}

fn logout(arg_token_id: &str) -> json::JsonValue {
	let mut stream = TcpStream::connect("127.0.0.1:13579").unwrap();
	let mut request_jobject = json::object! {
		length: 0,
		request: {
			func_name: "logout",
			arguments: {
				token_id: arg_token_id,
			}
		},
	};

	return call_remote_server(&mut stream, &mut request_jobject);
}

fn insert_key_event_data(request_jobject: &mut json::JsonValue) -> json::JsonValue {
	let mut stream = TcpStream::connect("127.0.0.1:13579").unwrap();

	return call_remote_server(&mut stream, request_jobject);
}

fn insert_mouse_moving_event_data(request_jobject: &mut json::JsonValue) -> json::JsonValue {
	let mut stream = TcpStream::connect("127.0.0.1:13579").unwrap();
	return call_remote_server(&mut stream, request_jobject);
}

fn view_key_event_data(
	arg_token_id: &str,
	arg_start_time: DateTime<Local>,
	arg_end_time: DateTime<Local>,
) -> json::JsonValue {
	let mut stream = TcpStream::connect("127.0.0.1:13579").unwrap();
	let mut request_jobject = json::object! {
		length: 0,
		request: {
			func_name: "view_key_event_data",
			arguments: {
				token_id: arg_token_id,
				start_time: arg_start_time.to_string(),
				end_time: arg_end_time.to_string()
			}
		},
	};

	return call_remote_server(&mut stream, &mut request_jobject);
}

fn view_mouse_moving_event_data(
	arg_token_id: &str,
	arg_start_time: DateTime<Local>,
	arg_end_time: DateTime<Local>,
) -> json::JsonValue {
	let mut stream = TcpStream::connect("127.0.0.1:13579").unwrap();
	let mut request_jobject = json::object! {
		length: 0,
		request: {
			func_name: "view_mouse_moving_event_data",
			arguments: {
				token_id: arg_token_id,
				start_time: arg_start_time.to_string(),
				end_time: arg_end_time.to_string()
			}
		},
	};

	return call_remote_server(&mut stream, &mut request_jobject);
}

fn call_remote_server_demo() {
	let response_jobject = register_user("cloudblaze", "111");
	println!("{}", response_jobject);
	let _returned_value = &response_jobject["response"]["returned_value"];

	let response_jobject = login("cloudblaze", "111");
	println!("{}", response_jobject);
	let returned_value = &response_jobject["response"]["returned_value"];
	let token_id = returned_value["token_id"].as_str().unwrap();

	// let response_jobject = modify_password(token_id, "111", "222");
	// println!("{}", response_jobject);

	let mut request_jobject = json::object! {
		length: 0,
		request: {
			func_name: "insert_key_event_data",
			arguments: {
				token_id: token_id,
				event_data_list: [{
					id: "290C6962-C05D-4551-2F73-1EF256511444",
					time: "2022-04-25 10:54:41.610301870 +08:00",
					key_name: "Keyboard 'A' Key",
					status: 1
				}, {
					id: "290C6962-C05D-4551-2F73-1EF256511445",
					time: "2022-04-25 11:54:41.610301870 +08:00",
					key_name: "Keyboard 'B' Key",
					status: 1
				}, {
					id: "290C6962-C05D-4551-2F73-1EF256511446",
					time: "2022-04-25 12:54:41.610301870 +08:00",
					key_name: "Keyboard 'C' Key",
					status: 1
				}]
			}
		},
	};
	let response_jobject = insert_key_event_data(&mut request_jobject);
	println!("{}", response_jobject);

	let mut request_jobject = json::object! {
		length: 0,
		request: {
			func_name: "insert_mouse_moving_event_data",
			arguments: {
				token_id: token_id,
				event_data_list: [{
					id: "290C6962-C05D-4551-2F73-1EF256511444",
					time: "2022-04-25 10:54:41.610301870 +08:00",
					original_position: "(124, 182)",
					new_position: "(125, 183)"
				}, {
					id: "290C6962-C05D-4551-2F73-1EF256511445",
					time: "2022-04-25 11:54:41.610301870 +08:00",
					original_position: "(126, 184)",
					new_position: "(127, 185)"
				}, {
					id: "290C6962-C05D-4551-2F73-1EF256511446",
					time: "2022-04-25 12:54:41.610301870 +08:00",
					original_position: "(128, 186)",
					new_position: "(129, 187)"
				}]
			}
		},
	};
	let response_jobject = insert_mouse_moving_event_data(&mut request_jobject);
	println!("{}", response_jobject);

	let response_jobject = view_key_event_data(
		token_id,
		Local::now() - chrono::Duration::hours(12),
		Local::now(),
	);
	println!("{}", response_jobject);

	let response_jobject = view_mouse_moving_event_data(
		token_id,
		Local::now() - chrono::Duration::hours(12),
		Local::now(),
	);
	println!("{}", response_jobject);

	let response_jobject = logout(token_id);
	println!("{}", response_jobject);
}
