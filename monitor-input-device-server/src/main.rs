include!("xor_encrypt.rs");

const KEY_STRING: &str = "1357924680";
//const MYSQL_USER: &str = "hy";
//const MYSQL_PASSWORD: &str = "huo04ying11xia";
 const MYSQL_USER: &str = "lzm";
 const MYSQL_PASSWORD: &str = "1234567890";

use chrono::prelude::*;
use chrono::Duration;
use mysql::prelude::*;
use mysql::*;
use std::io::prelude::*;
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq)]
struct User {
	ID: String,
	UserName: String,
	Password: String,
	TokenID: String,
	TokenExpiryTime: String,
}

impl Clone for User {
	fn clone(&self) -> User {
		let id = self.ID.as_str();
		let username = self.UserName.as_str();
		let password = self.Password.as_str();
		let token_id = self.TokenID.as_str();
		let token_expiry_time = self.TokenExpiryTime.as_str();

		User {
			ID: String::from(id),
			UserName: String::from(username),
			Password: String::from(password),
			TokenID: String::from(token_id),
			TokenExpiryTime: String::from(token_expiry_time),
		}
	}
}

#[derive(Debug, PartialEq, Eq)]
struct KeyPressingEvent {
	ID: String,
	UserID: String,
	Time: String,
	KeyName: String,
	Status: u8,
}

impl Clone for KeyPressingEvent {
	fn clone(&self) -> KeyPressingEvent {
		let id = self.ID.as_str();
		let user_id = self.UserID.as_str();
		let time = self.Time.as_str();
		let key_name = self.KeyName.as_str();
		let status = self.Status;

		KeyPressingEvent {
			ID: String::from(id),
			UserID: String::from(user_id),
			Time: String::from(time),
			KeyName: String::from(key_name),
			Status: status,
		}
	}
}

impl From<KeyPressingEvent> for json::JsonValue {
	fn from(event: KeyPressingEvent) -> Self {
		let mut json_object_of_event = json::JsonValue::new_object();
		json_object_of_event["id"] = json::JsonValue::from(event.ID);
		json_object_of_event["time"] = json::JsonValue::from(event.Time);
		json_object_of_event["key_name"] = json::JsonValue::from(event.KeyName);
		json_object_of_event["status"] = json::JsonValue::from(event.Status);

		json_object_of_event
	}
}

#[derive(Debug, PartialEq, Eq)]
struct MouseMovingEvent {
	ID: String,
	UserID: String,
	Time: String,
	OriginalPosition: String,
	NewPosition: String,
}

impl Clone for MouseMovingEvent {
	fn clone(&self) -> MouseMovingEvent {
		let id = self.ID.as_str();
		let user_id = self.UserID.as_str();
		let time = self.Time.as_str();
		let original_position = self.OriginalPosition.as_str();
		let new_position = self.NewPosition.as_str();

		MouseMovingEvent {
			ID: String::from(id),
			UserID: String::from(user_id),
			Time: String::from(time),
			OriginalPosition: String::from(original_position),
			NewPosition: String::from(new_position),
		}
	}
}

impl From<MouseMovingEvent> for json::JsonValue {
	fn from(event: MouseMovingEvent) -> Self {
		let mut json_object_of_event = json::JsonValue::new_object();
		json_object_of_event["id"] = json::JsonValue::from(event.ID);
		json_object_of_event["time"] = json::JsonValue::from(event.Time);
		json_object_of_event["original_position"] = json::JsonValue::from(event.OriginalPosition);
		json_object_of_event["new_position"] = json::JsonValue::from(event.NewPosition);

		json_object_of_event
	}
}

fn get_db_conn_string() -> String {
	let db_user = MYSQL_USER;
	let db_password = MYSQL_PASSWORD;
	let conn_string = format!(
		"mysql://{}:{}@localhost:3306/MonitorInputDevice",
		db_user, db_password
	);

	conn_string
}

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

fn response(stream: &mut TcpStream, response_jobject: &mut json::JsonValue) {
	let response_json = response_jobject.dump();
	let response_json_len = response_json.len() as i32;
	response_jobject
		.insert("length", accurate_len(response_json_len))
		.unwrap();
	let response_json = response_jobject.dump();

	let key_str = String::from(KEY_STRING);
	let key = key_str.into_bytes();
	let encrypt_data = encrypt(&response_json.into_bytes(), &key);
	stream.write(&encrypt_data.as_slice()).unwrap();
}

fn write_response_to_log(request_time: DateTime<Local>, response_jobject: &json::JsonValue) {
	let response_jobject = response_jobject.clone();
	let sql = format!(
		"UPDATE Log SET Response = '{}', ResponseTime = '{}' WHERE Time = '{}';",
		json::stringify(response_jobject).replace("'", "\\'"),
		&*Local::now().to_string(),
		&*request_time.to_string()
	);
	mysql_execute_sql(sql);
}

fn register_user(
	stream: &mut TcpStream,
	request_time: DateTime<Local>,
	json_object: &json::JsonValue,
) {
	println!("Calling register_user");

	let arguments = &json_object["request"]["arguments"];
	let username = arguments["username"].as_str().unwrap();
	let password = arguments["password"].as_str().unwrap();
	let password_md5 = md5::compute(password);

	let mut response_jobject;

	let conn_string = get_db_conn_string();
	let url = conn_string.as_str();
	let opts = Opts::from_url(url).unwrap();
	let pool = Pool::new(opts).unwrap();
	let mut conn = pool.get_conn().unwrap();
	let user_list = conn
		.query_map(
			format!(
				"SELECT ID, UserName, Password FROM User WHERE UserName = '{}';",
				username
			),
			|(id, username, password)| User {
				ID: id,
				UserName: username,
				Password: password,
				TokenID: String::from(""),
				TokenExpiryTime: String::from(""),
			},
		)
		.unwrap();
	if user_list.len() > 0 {
		response_jobject = json::object! {
			length: 0,
			response: {
				is_success: false,
				message: "Username already exists.",
				returned_value: {},
			},
		};
		response(stream, &mut response_jobject);
		write_response_to_log(request_time, &response_jobject);
		stream.shutdown(Shutdown::Both).unwrap();
		return;
	}

	let id = Uuid::new_v4();
	let sql = format!(
		"INSERT INTO User (ID, UserName, Password) VALUES ('{}', '{}', '{:x}');",
		id.to_string(),
		username.replace("'", "\\'"),
		password_md5
	);
	mysql_execute_sql(sql);

	response_jobject = json::object! {
		length: 0,
		response: {
			is_success: true,
			message: "Register user successfully.",
			returned_value: {},
		},
	};

	response(stream, &mut response_jobject);

	write_response_to_log(request_time, &response_jobject);
	stream.shutdown(Shutdown::Both).unwrap();
}

fn login(stream: &mut TcpStream, request_time: DateTime<Local>, json_object: &json::JsonValue) {
	println!("Calling login");

	let arguments = &json_object["request"]["arguments"];
	let username = arguments["username"].as_str().unwrap();
	let password = arguments["password"].as_str().unwrap();
	let password_md5 = md5::compute(password);

	let mut response_jobject;

	let conn_string = get_db_conn_string();
	let url = conn_string.as_str();
	let opts = Opts::from_url(url).unwrap();
	let pool = Pool::new(opts).unwrap();
	let mut conn = pool.get_conn().unwrap();
	let user_list = conn
		.query_map(
			format!(
				"SELECT ID, UserName, Password FROM User WHERE UserName = '{}';",
				username
			),
			|(id, username, password)| User {
				ID: id,
				UserName: username,
				Password: password,
				TokenID: String::from(""),
				TokenExpiryTime: String::from(""),
			},
		)
		.unwrap();
	if user_list.len() == 0 {
		response_jobject = json::object! {
			length: 0,
			response: {
				is_success: false,
				message: "Username doesn't exist.",
				returned_value: {},
			},
		};
		response(stream, &mut response_jobject);

		write_response_to_log(request_time, &response_jobject);
		stream.shutdown(Shutdown::Both).unwrap();
		return;
	}

	let user = user_list.get(0).unwrap().clone();

	if user.Password != format!("{:x}", password_md5) {
		response_jobject = json::object! {
			length: 0,
			response: {
				is_success: false,
				message: "User password error.",
				returned_value: {},
			},
		};
		response(stream, &mut response_jobject);

		write_response_to_log(request_time, &response_jobject);
		stream.shutdown(Shutdown::Both).unwrap();
		return;
	}

	let token_id = Uuid::new_v4().to_string();

	let sql = format!(
		"UPDATE User SET TokenID = '{}', TokenExpiryTime = '{}' WHERE ID = '{}';",
		token_id,
		(Local::now() + Duration::minutes(30)).to_string(),
		user.ID
	);
	mysql_execute_sql(sql);
	response_jobject = json::object! {
		length: 0,
		response: {
			is_success: true,
			message: "Login successfully.",
			returned_value: {
				token_id: String::from(token_id.as_str()),
				token_expiry_time: Local::now().to_string()
			},
		},
	};
	response(stream, &mut response_jobject);

	write_response_to_log(request_time, &response_jobject);
	stream.shutdown(Shutdown::Both).unwrap();
}

fn modify_password(
	stream: &mut TcpStream,
	request_time: DateTime<Local>,
	json_object: &json::JsonValue,
) {
	println!("Calling modify_password");

	let arguments = &json_object["request"]["arguments"];
	let token_id = arguments["token_id"].as_str().unwrap();
	let old_password = arguments["old_password"].as_str().unwrap();
	let new_password = arguments["new_password"].as_str().unwrap();
	let old_password_md5 = md5::compute(old_password);
	let new_password_md5 = md5::compute(new_password);

	let mut response_jobject;

	let conn_string = get_db_conn_string();
	let url = conn_string.as_str();
	let opts = Opts::from_url(url).unwrap();
	let pool = Pool::new(opts).unwrap();
	let mut conn = pool.get_conn().unwrap();
	let user_list = conn
		.query_map(
			format!(
				"SELECT ID, UserName, Password, TokenID, TokenExpiryTime FROM User WHERE TokenID = '{}';",
				token_id
			),
			|(id, username, password, token_id_in_db, token_expiry_time)| User {
				ID: id,
				UserName: username,
				Password: password,
				TokenID: token_id_in_db,
				TokenExpiryTime: token_expiry_time,
			},
		)
		.unwrap();
	if user_list.len() == 0 {
		response_jobject = json::object! {
			length: 0,
			response: {
				is_success: false,
				message: "Username doesn't exist.",
				returned_value: {},
			},
		};
		response(stream, &mut response_jobject);

		write_response_to_log(request_time, &response_jobject);
		stream.shutdown(Shutdown::Both).unwrap();
		return;
	}
	let user = user_list.get(0).unwrap().clone();

	if user.TokenID == String::from("") {
		response_jobject = json::object! {
			length: 0,
			response: {
				is_success: false,
				message: "User didn't login.",
				returned_value: {},
			},
		};
		response(stream, &mut response_jobject);

		write_response_to_log(request_time, &response_jobject);
		stream.shutdown(Shutdown::Both).unwrap();
		return;
	}

	if user.TokenID != token_id {
		response_jobject = json::object! {
			length: 0,
			response: {
				is_success: false,
				message: "Login token is error.",
				returned_value: {},
			},
		};
		response(stream, &mut response_jobject);

		write_response_to_log(request_time, &response_jobject);
		stream.shutdown(Shutdown::Both).unwrap();
		return;
	}

	if user.TokenExpiryTime < format!("{}", Local::now()) {
		let sql = format!(
			"UPDATE User SET TokenExpiryTime = '{}' WHERE ID = '{}';",
			(Local::now() + Duration::minutes(30)).to_string(),
			user.ID
		);
		mysql_execute_sql(sql);
	}

	if format!("{:x}", old_password_md5) != user.Password {
		response_jobject = json::object! {
			length: 0,
			response: {
				is_success: false,
				message: "User password error.",
				returned_value: {},
			},
		};
		response(stream, &mut response_jobject);

		write_response_to_log(request_time, &response_jobject);
		stream.shutdown(Shutdown::Both).unwrap();
		return;
	}

	let sql = format!(
		"UPDATE User SET Password = '{:x}' WHERE ID = '{}';",
		new_password_md5, user.ID,
	);
	mysql_execute_sql(sql);

	response_jobject = json::object! {
		length: 0,
		response: {
			is_success: true,
			message: "Modify user password successfully.",
			returned_value: {},
		},
	};

	response(stream, &mut response_jobject);

	write_response_to_log(request_time, &response_jobject);

	stream.shutdown(Shutdown::Both).unwrap();
}

fn logout(stream: &mut TcpStream, request_time: DateTime<Local>, json_object: &json::JsonValue) {
	println!("Calling logout");

	let arguments = &json_object["request"]["arguments"];
	let token_id = arguments["token_id"].as_str().unwrap();

	let mut response_jobject;

	let conn_string = get_db_conn_string();
	let url = conn_string.as_str();
	let opts = Opts::from_url(url).unwrap();
	let pool = Pool::new(opts).unwrap();
	let mut conn = pool.get_conn().unwrap();
	let user_list = conn
		.query_map(
			format!(
				"SELECT ID, UserName, Password, TokenID, TokenExpiryTime FROM User WHERE TokenID = '{}';",
				token_id
			),
			|(id, username, password, token_id_in_db, token_expiry_time)| User {
				ID: id,
				UserName: username,
				Password: password,
				TokenID: token_id_in_db,
				TokenExpiryTime: token_expiry_time,
			},
		)
		.unwrap();
	if user_list.len() == 0 {
		response_jobject = json::object! {
			length: 0,
			response: {
				is_success: false,
				message: "Username doesn't exist.",
				returned_value: {},
			},
		};
		response(stream, &mut response_jobject);

		write_response_to_log(request_time, &response_jobject);
		stream.shutdown(Shutdown::Both).unwrap();
		return;
	}
	let user = user_list.get(0).unwrap().clone();

	if user.TokenID == String::from("") {
		response_jobject = json::object! {
			length: 0,
			response: {
				is_success: false,
				message: "User didn't login.",
				returned_value: {},
			},
		};
		response(stream, &mut response_jobject);

		write_response_to_log(request_time, &response_jobject);
		stream.shutdown(Shutdown::Both).unwrap();
		return;
	}

	if user.TokenID != token_id {
		response_jobject = json::object! {
			length: 0,
			response: {
				is_success: false,
				message: "Login token is error.",
				returned_value: {},
			},
		};
		response(stream, &mut response_jobject);

		write_response_to_log(request_time, &response_jobject);
		stream.shutdown(Shutdown::Both).unwrap();
		return;
	}

	let sql = format!(
		"UPDATE User SET TokenID = '', TokenExpiryTime = '' WHERE ID = '{}';",
		user.ID,
	);
	mysql_execute_sql(sql);

	response_jobject = json::object! {
		length: 0,
		response: {
			is_success: true,
			message: "User logout successfully.",
			returned_value: {},
		},
	};

	response(stream, &mut response_jobject);

	write_response_to_log(request_time, &response_jobject);

	stream.shutdown(Shutdown::Both).unwrap();
}

fn insert_key_event_data(
	stream: &mut TcpStream,
	request_time: DateTime<Local>,
	json_object: &json::JsonValue,
) {
	println!("Calling insert_key_event_data");

	let arguments = &json_object["request"]["arguments"];
	let token_id = arguments["token_id"].as_str().unwrap();
	let event_data_list = &arguments["event_data_list"];

	let mut response_jobject;

	let conn_string = get_db_conn_string();
	let url = conn_string.as_str();
	let opts = Opts::from_url(url).unwrap();
	let pool = Pool::new(opts).unwrap();
	let mut conn = pool.get_conn().unwrap();
	let user_list = conn
		.query_map(
			format!(
				"SELECT ID, UserName, Password, TokenID, TokenExpiryTime FROM User WHERE TokenID = '{}';",
				token_id
			),
			|(id, username, password, token_id_in_db, token_expiry_time)| User {
				ID: id,
				UserName: username,
				Password: password,
				TokenID: token_id_in_db,
				TokenExpiryTime: token_expiry_time,
			},
		)
		.unwrap();
	if user_list.len() == 0 {
		response_jobject = json::object! {
			length: 0,
			response: {
				is_success: false,
				message: "User id doesn't exist.",
				returned_value: {},
			},
		};
		response(stream, &mut response_jobject);

		write_response_to_log(request_time, &response_jobject);
		stream.shutdown(Shutdown::Both).unwrap();
		return;
	}
	let user = user_list.get(0).unwrap().clone();

	if user.TokenID == String::from("") {
		response_jobject = json::object! {
			length: 0,
			response: {
				is_success: false,
				message: "User didn't login.",
				returned_value: {},
			},
		};
		response(stream, &mut response_jobject);

		write_response_to_log(request_time, &response_jobject);
		stream.shutdown(Shutdown::Both).unwrap();
		return;
	}

	if user.TokenID != token_id {
		response_jobject = json::object! {
			length: 0,
			response: {
				is_success: false,
				message: "Login token is error.",
				returned_value: {},
			},
		};
		response(stream, &mut response_jobject);

		write_response_to_log(request_time, &response_jobject);
		stream.shutdown(Shutdown::Both).unwrap();
		return;
	}

	if user.TokenExpiryTime < format!("{}", Local::now()) {
		let sql = format!(
			"UPDATE User SET TokenExpiryTime = '{}' WHERE ID = '{}';",
			(Local::now() + Duration::minutes(30)).to_string(),
			user.ID
		);
		mysql_execute_sql(sql);
	}

	let mut sql = String::new();
	for i in 0..event_data_list.len() {
		let event_data = &event_data_list[i];

		let id = event_data["id"].as_str().unwrap();
		let time = event_data["time"].as_str().unwrap();
		let key_name = event_data["key_name"].as_str().unwrap().replace("'", "\\'");
		let status = event_data["status"].as_i8().unwrap();

		let current_sql = format!("INSERT INTO KeyPressingEvent (ID, UserID, Time, KeyName, Status) VALUES ('{}', '{}', '{}', '{}', {});",
			id, user.ID, time, key_name, status);
		sql += current_sql.as_str();
	}
	mysql_execute_sql(sql);

	response_jobject = json::object! {
		length: 0,
		response: {
			is_success: true,
			message: "Insert key event data successfully.",
			returned_value: {},
		},
	};

	response(stream, &mut response_jobject);

	write_response_to_log(request_time, &response_jobject);

	stream.shutdown(Shutdown::Both).unwrap();
}

fn insert_mouse_moving_event_data(
	stream: &mut TcpStream,
	request_time: DateTime<Local>,
	json_object: &json::JsonValue,
) {
	println!("Calling insert_mouse_moving_event_data");

	let arguments = &json_object["request"]["arguments"];
	let token_id = arguments["token_id"].as_str().unwrap();
	let event_data_list = &arguments["event_data_list"];

	let mut response_jobject;

	let conn_string = get_db_conn_string();
	let url = conn_string.as_str();
	let opts = Opts::from_url(url).unwrap();
	let pool = Pool::new(opts).unwrap();
	let mut conn = pool.get_conn().unwrap();
	let user_list = conn
		.query_map(
			format!(
				"SELECT ID, UserName, Password, TokenID, TokenExpiryTime FROM User WHERE TokenID = '{}';",
				token_id
			),
			|(id, username, password, token_id_in_db, token_expiry_time)| User {
				ID: id,
				UserName: username,
				Password: password,
				TokenID: token_id_in_db,
				TokenExpiryTime: token_expiry_time,
			},
		)
		.unwrap();
	if user_list.len() == 0 {
		response_jobject = json::object! {
			length: 0,
			response: {
				is_success: false,
				message: "User id doesn't exist.",
				returned_value: {},
			},
		};
		response(stream, &mut response_jobject);

		write_response_to_log(request_time, &response_jobject);
		stream.shutdown(Shutdown::Both).unwrap();
		return;
	}
	let user = user_list.get(0).unwrap().clone();

	if user.TokenID == String::from("") {
		response_jobject = json::object! {
			length: 0,
			response: {
				is_success: false,
				message: "User didn't login.",
				returned_value: {},
			},
		};
		response(stream, &mut response_jobject);

		write_response_to_log(request_time, &response_jobject);
		stream.shutdown(Shutdown::Both).unwrap();
		return;
	}

	if user.TokenID != token_id {
		response_jobject = json::object! {
			length: 0,
			response: {
				is_success: false,
				message: "Login token is error.",
				returned_value: {},
			},
		};
		response(stream, &mut response_jobject);

		write_response_to_log(request_time, &response_jobject);
		stream.shutdown(Shutdown::Both).unwrap();
		return;
	}

	if user.TokenExpiryTime < format!("{}", Local::now()) {
		let sql = format!(
			"UPDATE User SET TokenExpiryTime = '{}' WHERE ID = '{}';",
			(Local::now() + Duration::minutes(30)).to_string(),
			user.ID
		);
		mysql_execute_sql(sql);
	}

	let mut sql = format!("INSERT INTO MouseMovingEvent (ID, UserID, Time, OriginalPosition, NewPosition) VALUES ('{}', '{}', '{}', '{}', '{}')",
		&event_data_list[0]["id"].as_str().unwrap(),
		user.ID,
		&event_data_list[0]["time"].as_str().unwrap(),
		&event_data_list[0]["original_position"].as_str().unwrap(),
		&event_data_list[0]["new_position"].as_str().unwrap());
	let mut prev_id: Option<String> = None;
	for i in 1..event_data_list.len() {
		let event_data = &event_data_list[i];

		let id = event_data["id"].as_str().unwrap();
		let time = event_data["time"].as_str().unwrap();
		let original_position = event_data["original_position"]
			.as_str()
			.unwrap()
			.replace("'", "\\'");
		let new_position = event_data["new_position"].as_str().unwrap();

		match prev_id.clone() {
			Some(prev_id) => {
				if prev_id == id {
					continue;
				}
			}
			None => (),
		}
		prev_id = Some(String::from(id));
		let current_sql = format!(
			" ,('{}', '{}', '{}', '{}', '{}')",
			id, user.ID, time, original_position, new_position
		);
		sql += current_sql.as_str();
	}
	sql += ";";
	mysql_execute_sql(sql);

	response_jobject = json::object! {
		length: 0,
		response: {
			is_success: true,
			message: "Insert mouse moving event data successfully.",
			returned_value: {},
		},
	};

	response(stream, &mut response_jobject);

	write_response_to_log(request_time, &response_jobject);

	stream.shutdown(Shutdown::Both).unwrap();
}

fn view_key_event_data(
	stream: &mut TcpStream,
	request_time: DateTime<Local>,
	json_object: &json::JsonValue,
) {
	println!("Calling view_key_event_data");

	let arguments = &json_object["request"]["arguments"];
	let token_id = arguments["token_id"].as_str().unwrap();
	let start_time = arguments["start_time"].as_str().unwrap();
	let end_time = arguments["end_time"].as_str().unwrap();

	let mut response_jobject;

	let conn_string = get_db_conn_string();
	let url = conn_string.as_str();
	let opts = Opts::from_url(url).unwrap();
	let pool = Pool::new(opts).unwrap();
	let mut conn = pool.get_conn().unwrap();
	let user_list = conn
		.query_map(
			format!(
				"SELECT ID, UserName, Password, TokenID, TokenExpiryTime FROM User WHERE TokenID = '{}';",
				token_id
			),
			|(id, username, password, token_id_in_db, token_expiry_time)| User {
				ID: id,
				UserName: username,
				Password: password,
				TokenID: token_id_in_db,
				TokenExpiryTime: token_expiry_time,
			},
		)
		.unwrap();
	if user_list.len() == 0 {
		response_jobject = json::object! {
			length: 0,
			response: {
				is_success: false,
				message: "Username doesn't exist.",
				returned_value: {},
			},
		};
		response(stream, &mut response_jobject);

		write_response_to_log(request_time, &response_jobject);
		stream.shutdown(Shutdown::Both).unwrap();
		return;
	}
	let user = user_list.get(0).unwrap().clone();

	if user.TokenID == String::from("") {
		response_jobject = json::object! {
			length: 0,
			response: {
				is_success: false,
				message: "User didn't login.",
				returned_value: {},
			},
		};
		response(stream, &mut response_jobject);

		write_response_to_log(request_time, &response_jobject);
		stream.shutdown(Shutdown::Both).unwrap();
		return;
	}

	if user.TokenID != token_id {
		response_jobject = json::object! {
			length: 0,
			response: {
				is_success: false,
				message: "Login token is error.",
				returned_value: {},
			},
		};
		response(stream, &mut response_jobject);

		write_response_to_log(request_time, &response_jobject);
		stream.shutdown(Shutdown::Both).unwrap();
		return;
	}

	if user.TokenExpiryTime < format!("{}", Local::now()) {
		let sql = format!(
			"UPDATE User SET TokenExpiryTime = '{}' WHERE ID = '{}';",
			(Local::now() + Duration::minutes(30)).to_string(),
			user.ID.as_str()
		);
		mysql_execute_sql(sql);
	}

	let conn_string = get_db_conn_string();
	let url = conn_string.as_str();
	let opts = Opts::from_url(url).unwrap();
	let pool = Pool::new(opts).unwrap();
	let mut conn = pool.get_conn().unwrap();
	let mut sql = String::from("SELECT ID, UserID, Time, KeyName, Status FROM KeyPressingEvent");
	if (start_time != String::new()) || (end_time != String::new()) {
		sql += " WHERE ";
		if start_time != String::from("") {
			sql += format!("Time >= '{}'", start_time).as_str();
		}
		if (start_time != String::new()) && (end_time != String::new()) {
			sql += " AND ";
		}
		if end_time != String::from("") {
			sql += format!("Time <= '{}'", end_time).as_str();
		}
	}
	sql += " ORDER BY Time;";
	let event_data_list = conn
		.query_map(sql, |(id, user_id, time, keyname, status)| {
			KeyPressingEvent {
				ID: id,
				UserID: user_id,
				Time: time,
				KeyName: keyname,
				Status: status,
			}
		})
		.unwrap();
	response_jobject = json::object! {
		length: 0,
		response: {
			is_success: true,
			message: "View key event data successfully.",
			returned_value: {
				event_data_list_count: event_data_list.len(),
				event_data_list: event_data_list
			},
		},
	};

	response(stream, &mut response_jobject);

	write_response_to_log(request_time, &response_jobject);

	stream.shutdown(Shutdown::Both).unwrap();
}

fn view_mouse_moving_event_data(
	stream: &mut TcpStream,
	request_time: DateTime<Local>,
	json_object: &json::JsonValue,
) {
	println!("Calling view_mouse_moving_event_data");

	let arguments = &json_object["request"]["arguments"];
	let token_id = arguments["token_id"].as_str().unwrap();
	let start_time = arguments["start_time"].as_str().unwrap();
	let end_time = arguments["end_time"].as_str().unwrap();

	let mut response_jobject;

	let conn_string = get_db_conn_string();
	let url = conn_string.as_str();
	let opts = Opts::from_url(url).unwrap();
	let pool = Pool::new(opts).unwrap();
	let mut conn = pool.get_conn().unwrap();
	let user_list = conn
		.query_map(
			format!(
				"SELECT ID, UserName, Password, TokenID, TokenExpiryTime FROM User WHERE TokenID = '{}';",
				token_id
			),
			|(id, username, password, token_id_in_db, token_expiry_time)| User {
				ID: id,
				UserName: username,
				Password: password,
				TokenID: token_id_in_db,
				TokenExpiryTime: token_expiry_time,
			},
		)
		.unwrap();
	if user_list.len() == 0 {
		response_jobject = json::object! {
			length: 0,
			response: {
				is_success: false,
				message: "Username doesn't exist.",
				returned_value: {},
			},
		};
		response(stream, &mut response_jobject);

		write_response_to_log(request_time, &response_jobject);
		stream.shutdown(Shutdown::Both).unwrap();
		return;
	}
	let user = user_list.get(0).unwrap().clone();

	if user.TokenID == String::from("") {
		response_jobject = json::object! {
			length: 0,
			response: {
				is_success: false,
				message: "User didn't login.",
				returned_value: {},
			},
		};
		response(stream, &mut response_jobject);

		write_response_to_log(request_time, &response_jobject);
		stream.shutdown(Shutdown::Both).unwrap();
		return;
	}

	if user.TokenID != token_id {
		response_jobject = json::object! {
			length: 0,
			response: {
				is_success: false,
				message: "Login token is error.",
				returned_value: {},
			},
		};
		response(stream, &mut response_jobject);

		write_response_to_log(request_time, &response_jobject);
		stream.shutdown(Shutdown::Both).unwrap();
		return;
	}

	if user.TokenExpiryTime < format!("{}", Local::now()) {
		let sql = format!(
			"UPDATE User SET TokenExpiryTime = '{}' WHERE ID = '{}';",
			(Local::now() + Duration::minutes(30)).to_string(),
			user.ID.as_str()
		);
		mysql_execute_sql(sql);
	}

	let conn_string = get_db_conn_string();
	let url = conn_string.as_str();
	let opts = Opts::from_url(url).unwrap();
	let pool = Pool::new(opts).unwrap();
	let mut conn = pool.get_conn().unwrap();
	let mut sql = String::from(
		"SELECT ID, UserID, Time, OriginalPosition, NewPosition FROM MouseMovingEvent",
	);
	if (start_time != String::new()) || (end_time != String::new()) {
		sql += " WHERE ";
		if start_time != String::from("") {
			sql += format!("Time >= '{}'", start_time).as_str();
		}
		if (start_time != String::new()) && (end_time != String::new()) {
			sql += " AND ";
		}
		if end_time != String::from("") {
			sql += format!("Time <= '{}'", end_time).as_str();
		}
	}
	sql += " ORDER BY Time;";
	let event_data_list = conn
		.query_map(
			sql,
			|(id, user_id, time, original_position, new_position)| MouseMovingEvent {
				ID: id,
				UserID: user_id,
				Time: time,
				OriginalPosition: original_position,
				NewPosition: new_position,
			},
		)
		.unwrap();
	response_jobject = json::object! {
		length: 0,
		response: {
			is_success: true,
			message: "View mouse moving event data successfully.",
			returned_value: {
				event_data_list_count: event_data_list.len(),
				event_data_list: event_data_list
			},
		},
	};

	response(stream, &mut response_jobject);

	write_response_to_log(request_time, &response_jobject);

	stream.shutdown(Shutdown::Both).unwrap();
}

fn mysql_execute_sql(sql: String) {
	let conn_string = get_db_conn_string();
	let url = conn_string.as_str();
	let opts = Opts::from_url(url).unwrap();
	let pool = Pool::new(opts).unwrap();
	let mut conn = pool.get_conn().unwrap();
	return conn.query_drop(sql).unwrap();
}

fn write_log(
	now: DateTime<Local>,
	ip: std::net::IpAddr,
	port: u16,
	request_json: &str,
) -> DateTime<Local> {
	// Write to log
	println!(
		"[{}] Client: {}:{}, Request: {}",
		now, ip, port, request_json
	);

	let sql = format!(
		"INSERT INTO Log (Time, IP, Port, Request) VALUES ('{}', '{}', '{}', '{}');",
		&*now.to_string(),
		&*ip.to_string(),
		&*port.to_string(),
		request_json
	);
	mysql_execute_sql(sql);

	return now;
}

fn get_request(stream: &mut TcpStream) -> (DateTime<Local>, json::JsonValue) {
	let now = Local::now();
	let ip = stream.peer_addr().unwrap().ip();
	let port = stream.peer_addr().unwrap().port();

	let mut encrypted_data: Vec<u8> = Vec::new();
	loop {
		let mut buffer = [0; 4096];
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
				return (now, json::object! {});
			}
		}
	}

	let key_str = String::from(KEY_STRING);
	let key = key_str.into_bytes();
	let original_data = decrypt(&encrypted_data, &key);
	let json_string = String::from_utf8(original_data).unwrap();

	let request_jobject = json::parse(&*json_string).unwrap();

	let now = write_log(now, ip, port, json_string.replace("'", "\\'").as_str());

	let length = &request_jobject["length"];
	if length != json_string.len() {
		eprintln!(
			"length = {}, json_string.len() = {}",
			length,
			json_string.len()
		);
		return (now, json::object! {});
	}

	return (now, request_jobject);
}

fn handle_client(stream: &mut TcpStream) {
	let request = get_request(stream);
	let request_time = request.0;
	let json_object = request.1;

	let func_name = &json_object["request"]["func_name"].as_str().unwrap();
	match func_name as &str {
		"register_user" => register_user(stream, request_time, &json_object),
		"login" => login(stream, request_time, &json_object),
		"modify_password" => modify_password(stream, request_time, &json_object),
		"logout" => logout(stream, request_time, &json_object),
		"insert_key_event_data" => insert_key_event_data(stream, request_time, &json_object),
		"insert_mouse_moving_event_data" => {
			insert_mouse_moving_event_data(stream, request_time, &json_object)
		}
		"view_key_event_data" => view_key_event_data(stream, request_time, &json_object),
		"view_mouse_moving_event_data" => {
			view_mouse_moving_event_data(stream, request_time, &json_object)
		}
		_ => (),
	}
}

fn main() {
	let listener = TcpListener::bind("127.0.0.1:13579").unwrap();

	for stream in listener.incoming() {
		let mut stream = stream.unwrap();
		thread::spawn(move || {
			handle_client(&mut stream);
		});
	}
}
