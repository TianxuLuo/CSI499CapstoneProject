include!("event_view_window.rs");
include!("event_analyse_window.rs");
include!("modify_password_window.rs");

static mut IS_MONITOR_DEVICE: bool = false;
static mut IS_TASK_RUNNING: bool = false;
static mut TASK_START_TIME: Option<DateTime<chrono::Local>> = None;
static mut TASK_END_TIME: Option<DateTime<chrono::Local>> = None;

struct KeyPressingEvent {
	id: String,
	user_id: String,
	time: chrono::DateTime<chrono::Local>,
	key_code: String,
	status: bool,
	is_synchro: bool,
}
static mut NEW_KEY_PRESSING_EVENT: Vec<KeyPressingEvent> = Vec::new();

struct MouseMovingEvent {
	id: String,
	user_id: String,
	time: chrono::DateTime<chrono::Local>,
	original_position: String,
	new_position: String,
	is_synchro: bool,
}
static mut NEW_MOUSE_MOVING_EVENT: Vec<MouseMovingEvent> = Vec::new();

struct KeyPressingEventToServer {
	ID: String,
	Time: String,
	KeyName: String,
	Status: u8,
}
static mut NEW_KEY_PRESSING_EVENT_TO_SERVER: Vec<KeyPressingEventToServer> = Vec::new();

struct MouseMovingEventToServer {
	ID: String,
	Time: String,
	OriginalPosition: String,
	NewPosition: String,
}
static mut NEW_MOUSE_MOVING_EVENT_TO_SERVER: Vec<MouseMovingEventToServer> = Vec::new();

static mut NEW_SQL: Vec<String> = Vec::new();

fn create_operate_window(application: &Application) -> ApplicationWindow {
	let operate_window = ApplicationWindow::builder()
		.title("Monitor Input Device -- Operate")
		.resizable(false)
		.deletable(false)
		.decorated(true)
		.build();

	let main_vbox = Box::builder().orientation(Orientation::Horizontal).build();

	let _thread = thread::spawn(|| unsafe {
		let db_file = String::from(DB_FILE);

		loop {
			if NEW_SQL.len() > 0 {
				let count = if NEW_SQL.len() <= 10000 {
					NEW_SQL.len()
				} else {
					10000
				};

				let connection = sqlite::open(&*db_file).unwrap();
				connection.execute("PRAGMA synchronous = OFF;").unwrap();
				connection.execute("BEGIN;").unwrap();
				for _i in 0..count {
					let sql = NEW_SQL.remove(0);
					// println!("---SQL: {}", sql);
					connection.execute(sql).unwrap();
				}
				connection.execute("COMMIT;").unwrap();
			} else {
				if !IS_MONITOR_DEVICE {
					thread::sleep(Duration::from_millis(100));
				}
			}
		}
	});

	let _thread = thread::spawn(|| unsafe {
		loop {
			if NEW_KEY_PRESSING_EVENT_TO_SERVER.len() > 0 {
				let count = if NEW_KEY_PRESSING_EVENT_TO_SERVER.len() <= 100 {
					NEW_KEY_PRESSING_EVENT_TO_SERVER.len()
				} else {
					100
				};

				let mut request_jobject = json::JsonValue::new_object();
				request_jobject["length"] = 0.into();

				let mut request = json::JsonValue::new_object();
				request["func_name"] = "insert_key_event_data".into();
				let mut arguments = json::JsonValue::new_object();
				arguments["token_id"] = TOKEN_ID.clone().unwrap().into();

				let mut start_time: Option<String> = None;
				let mut end_time: Option<String> = None;
				let mut event_data_list = json::JsonValue::new_array();
				for i in 0..count {
					let event = NEW_KEY_PRESSING_EVENT_TO_SERVER.remove(0);

					if i == 0 {
						start_time = Some(event.Time.clone());
					}
					end_time = Some(event.Time.clone());

					let mut event_jobject = json::JsonValue::new_object();
					event_jobject["id"] = event.ID.into();
					event_jobject["time"] = event.Time.into();
					event_jobject["key_name"] = event.KeyName.into();
					event_jobject["status"] = event.Status.into();

					event_data_list.push(event_jobject);
				}
				arguments["event_data_list"] = event_data_list;

				request["arguments"] = arguments;

				request_jobject["request"] = request;

				let response_jobject = insert_key_event_data(&mut request_jobject);
				println!("{}", response_jobject);

				let response = &response_jobject["response"];
				let is_success = response["is_success"].as_bool().unwrap();
				if is_success {
					/* Write to SQLite Begin */
					let sql = format!(
						"UPDATE KeyPressingEvent SET IsSynchro = '1' WHERE Time >= '{}' AND Time <= '{}';",
						start_time.unwrap(),
						end_time.unwrap()
					);
					NEW_SQL.push(sql);
					/* Write to SQLite End */
				}
			} else {
				if !IS_MONITOR_DEVICE {
					thread::sleep(Duration::from_millis(100));
				}
			}
		}
	});

	let _thread = thread::spawn(|| unsafe {
		loop {
			if NEW_MOUSE_MOVING_EVENT_TO_SERVER.len() > 0 {
				let count = if NEW_MOUSE_MOVING_EVENT_TO_SERVER.len() <= 300 {
					NEW_MOUSE_MOVING_EVENT_TO_SERVER.len()
				} else {
					300
				};

				let mut request_jobject = json::JsonValue::new_object();
				request_jobject["length"] = 0.into();

				let mut request = json::JsonValue::new_object();
				request["func_name"] = "insert_mouse_moving_event_data".into();
				let mut arguments = json::JsonValue::new_object();
				arguments["token_id"] = TOKEN_ID.clone().unwrap().into();

				let mut start_time: Option<String> = None;
				let mut end_time: Option<String> = None;
				let mut event_data_list = json::JsonValue::new_array();
				for i in 0..count {
					let event = NEW_MOUSE_MOVING_EVENT_TO_SERVER.remove(0);

					if i == 0 {
						start_time = Some(event.Time.clone());
					}
					end_time = Some(event.Time.clone());

					let mut event_jobject = json::JsonValue::new_object();
					event_jobject["id"] = event.ID.into();
					event_jobject["time"] = event.Time.into();
					event_jobject["original_position"] = event.OriginalPosition.into();
					event_jobject["new_position"] = event.NewPosition.into();

					event_data_list.push(event_jobject);
				}
				arguments["event_data_list"] = event_data_list;

				request["arguments"] = arguments;

				request_jobject["request"] = request;

				let response_jobject = insert_mouse_moving_event_data(&mut request_jobject);
				println!("{}", response_jobject);

				let response = &response_jobject["response"];
				let is_success = response["is_success"].as_bool().unwrap();
				if is_success {
					/* Write to SQLite Begin */
					let sql = format!(
						"UPDATE MouseMovingEvent SET IsSynchro = '1' WHERE Time >= '{}' AND Time <= '{}';",
						start_time.unwrap(),
						end_time.unwrap()
					);
					NEW_SQL.push(sql);
					/* Write to SQLite End */
				}
			} else {
				if !IS_MONITOR_DEVICE {
					thread::sleep(Duration::from_millis(100));
				}
			}
		}
	});

	let _thread = thread::spawn(|| unsafe {
		loop {
			if NEW_KEY_PRESSING_EVENT.len() > 0 {
				let event_info = NEW_KEY_PRESSING_EVENT.remove(0);

				let sql = format!(
					"INSERT INTO KeyPressingEvent VALUES ('{}', '{}', '{}', '{}', {}, {});",
					event_info.id,
					&*event_info.user_id,
					event_info.time,
					event_info.key_code.replace("'", "''"),
					event_info.status,
					event_info.is_synchro
				);
				NEW_SQL.push(sql);

				if NET_IS_ENABLED {
					NEW_KEY_PRESSING_EVENT_TO_SERVER.push(KeyPressingEventToServer {
						ID: event_info.id,
						Time: event_info.time.to_string(),
						KeyName: event_info.key_code,
						Status: event_info.status as u8,
					});
				}
			} else {
				if IS_MONITOR_DEVICE == false {
					thread::sleep(Duration::from_millis(100));
				}
			}
		}
	});

	let _thread = thread::spawn(|| unsafe {
		loop {
			if NEW_MOUSE_MOVING_EVENT.len() > 0 {
				let event_info = NEW_MOUSE_MOVING_EVENT.remove(0);

				let sql = format!(
					"INSERT INTO MouseMovingEvent VALUES ('{}', '{}', '{}', '{}', '{}', {});",
					event_info.id,
					&*event_info.user_id,
					event_info.time,
					event_info.original_position,
					event_info.new_position,
					event_info.is_synchro
				);
				NEW_SQL.push(sql);

				if NET_IS_ENABLED {
					NEW_MOUSE_MOVING_EVENT_TO_SERVER.push(MouseMovingEventToServer {
						ID: event_info.id,
						Time: event_info.time.to_string(),
						OriginalPosition: event_info.original_position,
						NewPosition: event_info.new_position,
					});
				}
			} else {
				if IS_MONITOR_DEVICE == false {
					thread::sleep(Duration::from_millis(100));
				}
			}
		}
	});

	/* View Event Button */
	let file = gio::File::for_path("images/View_Enabled.png");
	let asset_paintable = gdk::Texture::from_file(&file).unwrap();
	let image_from = gtk4::Image::builder()
		.pixel_size(48)
		.paintable(&asset_paintable)
		.build();
	let event_view_button = Button::builder()
		.child(&image_from)
		.sensitive(false)
		.build();

	event_view_button.connect_clicked(clone!(@weak application => move |_| {
		let event_view_window = create_event_view_window(&application);
		event_view_window.set_modal(true);
		event_view_window.present();
	}));

	/* Analyse Event Button */
	let file = gio::File::for_path("images/Analyse_Enabled.png");
	let asset_paintable = gdk::Texture::from_file(&file).unwrap();
	let image_from = gtk4::Image::builder()
		.pixel_size(48)
		.paintable(&asset_paintable)
		.build();
	let event_analyse_button = Button::builder()
		.child(&image_from)
		.sensitive(false)
		.build();

	event_analyse_button.connect_clicked(clone!(@weak application => move |_| {
		let event_analyse_window = create_event_analyse_window(&application);
		event_analyse_window.set_modal(true);
		event_analyse_window.present();
	}));

	/* Stop Button */
	let file = gio::File::for_path("images/Stop_Enabled.png");
	let asset_paintable = gdk::Texture::from_file(&file).unwrap();
	let image_from = gtk4::Image::builder()
		.pixel_size(48)
		.paintable(&asset_paintable)
		.build();
	let stop_button = Button::builder()
		.child(&image_from)
		.sensitive(false)
		.build();

	/* Start Button */
	let file = gio::File::for_path("images/Start_Enabled.png");
	let asset_paintable = gdk::Texture::from_file(&file).unwrap();
	let image_from = gtk4::Image::builder()
		.pixel_size(48)
		.paintable(&asset_paintable)
		.build();
	let start_button = Button::builder().child(&image_from).build();

	start_button.connect_clicked(
		clone!(@weak event_view_button, @weak event_analyse_button, @weak stop_button, @weak application => move |button| {
			unsafe{
				if !IS_MONITOR_DEVICE {
					IS_MONITOR_DEVICE = true;

					let _thread = thread::spawn(|| {
						monitor_device();
					});
					stop_button.set_sensitive(true);

					let file = gio::File::for_path("images/Pause_Enabled.png");
					let asset_paintable = gdk::Texture::from_file(&file).unwrap();
					let image_from = gtk4::Image::builder()
						.pixel_size(48)
						.paintable(&asset_paintable)
						.build();
					button.set_child(Some(&image_from));
				} else{
					IS_MONITOR_DEVICE = false;

					let file = gio::File::for_path("images/Start_Enabled.png");
					let asset_paintable = gdk::Texture::from_file(&file).unwrap();
					let image_from = gtk4::Image::builder()
						.pixel_size(48)
						.paintable(&asset_paintable)
						.build();
					button.set_child(Some(&image_from));
				}

				if !IS_TASK_RUNNING{
					TASK_START_TIME = Some(Local::now());
					TASK_END_TIME = None;
					IS_TASK_RUNNING = true;
				}
			}

			event_view_button.set_sensitive(false);
			event_analyse_button.set_sensitive(false);
		}),
	);

	stop_button.connect_clicked(
		clone!(@weak event_view_button, @weak event_analyse_button, @weak start_button, @weak application => move |button| {
			unsafe{
				if IS_TASK_RUNNING{
					IS_MONITOR_DEVICE = false;
					IS_TASK_RUNNING = false;
					TASK_END_TIME = Some(Local::now());
				}
			}

			let file = gio::File::for_path("images/Start_Enabled.png");
				let asset_paintable = gdk::Texture::from_file(&file).unwrap();
				let image_from = gtk4::Image::builder()
					.pixel_size(48)
					.paintable(&asset_paintable)
					.build();
			start_button.set_child(Some(&image_from));

			button.set_sensitive(false);
			event_view_button.set_sensitive(true);
			event_analyse_button.set_sensitive(true);
		}),
	);

	main_vbox.append(&start_button);

	main_vbox.append(&stop_button);

	main_vbox.append(&event_view_button);

	main_vbox.append(&event_analyse_button);

	/* Modify Password Button */
	let file = gio::File::for_path("images/Modify_Password_Enabled.png");
	let asset_paintable = gdk::Texture::from_file(&file).unwrap();
	let image_from = gtk4::Image::builder()
		.pixel_size(48)
		.paintable(&asset_paintable)
		.build();
	let modify_password_button = Button::builder().child(&image_from).build();

	modify_password_button.connect_clicked(clone!(@weak application => move |_| {
		let modify_password_window = create_modify_password_window(&application);
		modify_password_window.set_modal(true);
		modify_password_window.present();
	}));
	main_vbox.append(&modify_password_button);

	/* Quit Button */
	let file = gio::File::for_path("images/Quit_Enabled.png");
	let asset_paintable = gdk::Texture::from_file(&file).unwrap();
	let image_from = gtk4::Image::builder()
		.pixel_size(48)
		.paintable(&asset_paintable)
		.build();
	let quit_button = Button::builder().child(&image_from).build();

	quit_button.connect_clicked(clone!(@weak application => move |_| {
		let token_id = unsafe { TOKEN_ID.clone().unwrap() };
		let response_jobject = logout(token_id.as_str());
		println!("{}", response_jobject);
		// let response = &response_jobject["response"];
		// let is_success = response["is_success"].as_bool().unwrap();
		// let message = response["message"].as_str().unwrap();

		application.quit();
	}));
	main_vbox.append(&quit_button);

	operate_window.set_child(Some(&main_vbox));

	application.add_window(&operate_window);

	operate_window.connect_destroy(move |_| {
		// Do something before closing.
	});

	operate_window
}

fn monitor_device() {
	let device_state = DeviceState::new();
	let last_mouse_position = device_state.query_pointer().coords;
	let arc_of_last_mouse_position = Arc::new(Mutex::new(last_mouse_position));
	let pressed_keys = HashMap::new();
	let arc_of_pressed_keys = Arc::new(Mutex::new(pressed_keys));
	let is_caps_lock_key_pressed = false;
	let arc_of_is_caps_lock_key_pressed = Arc::new(Mutex::new(is_caps_lock_key_pressed));

	let last_mouse_position = Arc::clone(&arc_of_last_mouse_position);
	let _guard = device_state.on_mouse_move(move |position| {
		unsafe {
			if IS_MONITOR_DEVICE == false {
				return;
			}
		}

		let now = Local::now();
		let mut last_mouse_position = last_mouse_position.lock().unwrap();

		unsafe {
			let guid = GUID::rand();
			let guid_str = guid.to_string();
			NEW_MOUSE_MOVING_EVENT.push(MouseMovingEvent {
				id: guid_str,
				user_id: USER_ID.clone().unwrap(),
				time: now,
				original_position: format!(
					"({}, {})",
					last_mouse_position.0, last_mouse_position.1
				),
				new_position: format!("({}, {})", position.0, position.1),
				is_synchro: false,
			});
		}

		*last_mouse_position = *position;
	});

	let pressed_keys = Arc::clone(&arc_of_pressed_keys);
	let _guard = device_state.on_mouse_down(move |button| {
		unsafe {
			if IS_MONITOR_DEVICE == false {
				return;
			}
		}

		let now = Local::now();
		let mut pressed_keys = pressed_keys.lock().unwrap();

		unsafe {
			let guid = GUID::rand();
			let guid_str = guid.to_string();
			NEW_KEY_PRESSING_EVENT.push(KeyPressingEvent {
				id: guid_str,
				user_id: USER_ID.clone().unwrap(),
				time: now,
				key_code: format!(
					"Mouse {} Button",
					match button {
						1 => "Left".to_string(),
						2 => "Middle".to_string(),
						3 => "Right".to_string(),
						_ => "Extended Key".to_string(),
					}
				),
				status: true,
				is_synchro: false,
			});
		}

		(*pressed_keys).insert(format!("MOUSE_{}", button), now);
	});

	let pressed_keys = Arc::clone(&arc_of_pressed_keys);
	let _guard = device_state.on_mouse_up(move |button| {
		unsafe {
			if IS_MONITOR_DEVICE == false {
				return;
			}
		}

		let now = Local::now();
		let mut pressed_keys = pressed_keys.lock().unwrap();

		unsafe {
			let guid = GUID::rand();
			let guid_str = guid.to_string();
			NEW_KEY_PRESSING_EVENT.push(KeyPressingEvent {
				id: guid_str,
				user_id: USER_ID.clone().unwrap(),
				time: now,
				key_code: format!(
					"Mouse {} Button",
					match button {
						1 => "Left".to_string(),
						2 => "Middle".to_string(),
						3 => "Right".to_string(),
						_ => "Extended Key".to_string(),
					}
				),
				status: false,
				is_synchro: false,
			});
		}
	});

	let pressed_keys = Arc::clone(&arc_of_pressed_keys);
	let _guard = device_state.on_key_down(move |key| {
		unsafe {
			if IS_MONITOR_DEVICE == false {
				return;
			}
		}

		let now = Local::now();
		let mut pressed_keys = pressed_keys.lock().unwrap();

		unsafe {
			let guid = GUID::rand();
			let guid_str = guid.to_string();
			NEW_KEY_PRESSING_EVENT.push(KeyPressingEvent {
				id: guid_str,
				user_id: USER_ID.clone().unwrap(),
				time: now,
				key_code: format!("Keyboard '{}' Key", key),
				status: true,
				is_synchro: false,
			});
		}

		(*pressed_keys).insert(format!("KEYBOARD_{}", key), now);
	});

	let pressed_keys = Arc::clone(&arc_of_pressed_keys);
	let is_caps_lock_key_pressed = Arc::clone(&arc_of_is_caps_lock_key_pressed);
	let _guard = device_state.on_key_up(move |key| {
		unsafe {
			if IS_MONITOR_DEVICE == false {
				return;
			}
		}

		let now = Local::now();
		let mut pressed_keys = pressed_keys.lock().unwrap();
		let mut is_caps_lock_key_pressed = is_caps_lock_key_pressed.lock().unwrap();

		let mut character = format!("{}", key);
		let shift_characters = [
			Keycode::Grave,
			Keycode::Key1,
			Keycode::Key2,
			Keycode::Key3,
			Keycode::Key4,
			Keycode::Key5,
			Keycode::Key6,
			Keycode::Key7,
			Keycode::Key8,
			Keycode::Key9,
			Keycode::Key0,
			Keycode::Minus,
			Keycode::Equal,
			Keycode::LeftBracket,
			Keycode::RightBracket,
			Keycode::BackSlash,
			Keycode::Semicolon,
			Keycode::Apostrophe,
			Keycode::Comma,
			Keycode::Dot,
			Keycode::Slash,
		];
		let letters = [
			Keycode::A,
			Keycode::B,
			Keycode::C,
			Keycode::D,
			Keycode::E,
			Keycode::F,
			Keycode::G,
			Keycode::H,
			Keycode::I,
			Keycode::J,
			Keycode::K,
			Keycode::L,
			Keycode::M,
			Keycode::N,
			Keycode::O,
			Keycode::P,
			Keycode::Q,
			Keycode::R,
			Keycode::S,
			Keycode::T,
			Keycode::U,
			Keycode::V,
			Keycode::W,
			Keycode::X,
			Keycode::Y,
			Keycode::Z,
		];
		if shift_characters.contains(key) {
			if (pressed_keys.contains_key(&"KEYBOARD_LShift".to_string())
				|| pressed_keys.contains_key(&"KEYBOARD_RShift".to_string()))
				&& pressed_keys.len() == 2
			{
				match key {
					Keycode::Grave => character = "~".to_string(),
					Keycode::Key1 => character = "!".to_string(),
					Keycode::Key2 => character = "@".to_string(),
					Keycode::Key3 => character = "#".to_string(),
					Keycode::Key4 => character = "$".to_string(),
					Keycode::Key5 => character = "%".to_string(),
					Keycode::Key6 => character = "^".to_string(),
					Keycode::Key7 => character = "&".to_string(),
					Keycode::Key8 => character = "*".to_string(),
					Keycode::Key9 => character = "(".to_string(),
					Keycode::Key0 => character = ")".to_string(),
					Keycode::Minus => character = "_".to_string(),
					Keycode::Equal => character = "+".to_string(),
					Keycode::LeftBracket => character = "{".to_string(),
					Keycode::RightBracket => character = "}".to_string(),
					Keycode::BackSlash => character = "|".to_string(),
					Keycode::Semicolon => character = ":".to_string(),
					Keycode::Apostrophe => character = "\"".to_string(),
					Keycode::Comma => character = "<".to_string(),
					Keycode::Dot => character = ">".to_string(),
					Keycode::Slash => character = "?".to_string(),
					_ => character = character,
				}
			} else {
				match key {
					Keycode::Grave => character = "`".to_string(),
					Keycode::Key1 => character = "1".to_string(),
					Keycode::Key2 => character = "2".to_string(),
					Keycode::Key3 => character = "3".to_string(),
					Keycode::Key4 => character = "4".to_string(),
					Keycode::Key5 => character = "5".to_string(),
					Keycode::Key6 => character = "6".to_string(),
					Keycode::Key7 => character = "7".to_string(),
					Keycode::Key8 => character = "8".to_string(),
					Keycode::Key9 => character = "9".to_string(),
					Keycode::Key0 => character = "0".to_string(),
					Keycode::Minus => character = "-".to_string(),
					Keycode::Equal => character = "=".to_string(),
					Keycode::LeftBracket => character = "[".to_string(),
					Keycode::RightBracket => character = "]".to_string(),
					Keycode::BackSlash => character = "\\".to_string(),
					Keycode::Semicolon => character = ";".to_string(),
					Keycode::Apostrophe => character = "'".to_string(),
					Keycode::Comma => character = ",".to_string(),
					Keycode::Dot => character = ".".to_string(),
					Keycode::Slash => character = "/".to_string(),
					_ => character = character,
				}
			}
		} else if letters.contains(key) {
			character = if *is_caps_lock_key_pressed {
				if (pressed_keys.contains_key(&"KEYBOARD_LShift".to_string())
					|| pressed_keys.contains_key(&"KEYBOARD_RShift".to_string()))
					&& pressed_keys.len() == 2
				{
					character.to_lowercase()
				} else {
					character.to_uppercase()
				}
			} else {
				if (pressed_keys.contains_key(&"KEYBOARD_LShift".to_string())
					|| pressed_keys.contains_key(&"KEYBOARD_RShift".to_string()))
					&& pressed_keys.len() == 2
				{
					character.to_uppercase()
				} else {
					character.to_lowercase()
				}
			};
		}

		unsafe {
			let guid = GUID::rand();
			let guid_str = guid.to_string();
			NEW_KEY_PRESSING_EVENT.push(KeyPressingEvent {
				id: guid_str,
				user_id: USER_ID.clone().unwrap(),
				time: now,
				key_code: format!("Keyboard '{}' Key", key),
				status: false,
				is_synchro: false,
			});
		}

		if *key == Keycode::CapsLock {
			*is_caps_lock_key_pressed = !*is_caps_lock_key_pressed;
		}
	});

	//loop {}
	unsafe { while IS_MONITOR_DEVICE {} }
}
