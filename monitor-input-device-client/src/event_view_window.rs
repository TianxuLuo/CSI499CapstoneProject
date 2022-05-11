use regex::Regex;

fn create_event_view_window(application: &Application) -> ApplicationWindow {
	let event_view_window = ApplicationWindow::builder()
		.title("Monitor Input Device -- Event view")
		.default_width(1300)
		.default_height(640)
		.build();

	let main_vbox = Box::builder()
		.margin_start(50)
		.margin_end(50)
		.margin_top(10)
		.margin_bottom(30)
		.orientation(Orientation::Horizontal)
		.build();

	/* Mouse Moving */
	let list_box_of_mouse_moving = ListBox::builder().build();

	let mouse_response_jobject = view_mouse_moving_event_data(
		unsafe { TOKEN_ID.clone().unwrap().as_str() },
		unsafe { TASK_START_TIME.unwrap() },
		unsafe { TASK_END_TIME.unwrap() },
	);
	println!("{}", mouse_response_jobject);

	let mouse_is_success = &mouse_response_jobject["response"]["is_success"]
		.as_bool()
		.unwrap();
	let mouse_message = &mouse_response_jobject["response"]["message"]
		.as_str()
		.unwrap();

	if !mouse_is_success {
		let info_dialog = MessageDialog::builder()
			.transient_for(&event_view_window)
			.modal(true)
			.buttons(ButtonsType::Ok)
			.text("Infomation")
			.secondary_text(mouse_message)
			.build();

		info_dialog.run_async(move |obj, _| {
			obj.close();
		});
	} else {
		let mouse_returned_value = &mouse_response_jobject["response"]["returned_value"];
		let _mouse_event_data_list_count = &mouse_returned_value["event_data_list_count"];
		let mouse_event_data_list = &mouse_returned_value["event_data_list"];

		for i in 0..mouse_event_data_list.len() {
			let mouse_event_data = &mouse_event_data_list[i];
			let event_info = format!(
				"[{}] Mouse move {} => {}",
				mouse_event_data["time"],
				mouse_event_data["original_position"],
				mouse_event_data["new_position"]
			);
			let label = Label::builder()
				.label(event_info.as_str())
				.halign(Align::Start)
				.build();
			list_box_of_mouse_moving.insert(&label, -1);
		}
	}

	main_vbox.append(&list_box_of_mouse_moving);

	/* Key Pressing */
	let list_box_of_key_pressing = ListBox::builder().margin_start(150).build();

	let key_response_jobject = view_key_event_data(
		unsafe { TOKEN_ID.clone().unwrap().as_str() },
		unsafe { TASK_START_TIME.unwrap() },
		unsafe { TASK_END_TIME.unwrap() },
	);
	println!("{}", key_response_jobject);

	let key_is_success = &key_response_jobject["response"]["is_success"]
		.as_bool()
		.unwrap();
	let key_message = &key_response_jobject["response"]["message"]
		.as_str()
		.unwrap();

	if !key_is_success {
		let info_dialog = MessageDialog::builder()
			.transient_for(&event_view_window)
			.modal(true)
			.buttons(ButtonsType::Ok)
			.text("Infomation")
			.secondary_text(key_message)
			.build();

		info_dialog.run_async(move |obj, _| {
			obj.close();
		});
	} else {
		let key_returned_value = &key_response_jobject["response"]["returned_value"];
		let _key_event_data_list_count = &key_returned_value["event_data_list_count"];
		let key_event_data_list = &key_returned_value["event_data_list"];

		let mut pressed_key_hashmap: HashMap<String, chrono::DateTime<chrono::Local>> =
			HashMap::new();
		let mut is_left_shift_pressed = false;
		let mut is_right_shift_pressed = false;
		let mut caps_lock = false;
		for i in 0..key_event_data_list.len() {
			let key_event_data = &key_event_data_list[i];
			let key_time = key_event_data["time"].as_str().unwrap();
			let key_name = key_event_data["key_name"].as_str().unwrap();
			let status = if key_event_data["status"].as_u8().unwrap() == 1 {
				"Pressed"
			} else {
				"Released"
			};

			let event_info = format!("[{}] {} {}", key_time, key_name, status);
			let label = Label::builder()
				.label(event_info.as_str())
				.halign(Align::Start)
				.build();
			list_box_of_key_pressing.insert(&label, -1);

			if status == "Pressed" {
				let start_time = Local
					.datetime_from_str(key_time, "%Y-%m-%d %H:%M:%S%.f %z")
					.unwrap();
				pressed_key_hashmap.insert(String::from(key_name), start_time);
			} else {
				let key = String::from(key_name);

				if pressed_key_hashmap.contains_key(&key) {
					let start_time = pressed_key_hashmap.remove(&String::from(key_name)).unwrap();
					let end_time = Local
						.datetime_from_str(key_time, "%Y-%m-%d %H:%M:%S%.f %z")
						.unwrap();
					let duration = end_time - start_time;

					let event_info = format!(
						"[{}] {} pressed {}.{} s.",
						end_time,
						key_name,
						duration.num_seconds(),
						duration.num_nanoseconds().unwrap() % 1000000000
					);
					let label = Label::builder()
						.label(event_info.as_str())
						.halign(Align::Start)
						.build();
					list_box_of_key_pressing.insert(&label, -1);
				}
			}

			if String::from(key_name).contains("Keyboard") {
				let regex = Regex::new(r"'\S+'").unwrap();
				let capture = regex.captures(key_name).unwrap();
				let key_code = &capture[0];
				let key_code = key_code.trim_matches('\'');
				if status == "Pressed" {
					if key_code == "LShift" {
						is_left_shift_pressed = true;
					} else if key_code == "RShift" {
						is_right_shift_pressed = true;
					}
				} else {
					if key_code == "LShift" {
						is_left_shift_pressed = false;
					} else if key_code == "RShift" {
						is_right_shift_pressed = false;
					} else if key_code == "CapsLock" {
						caps_lock = !caps_lock;
					} else {
						let is_upper = (!caps_lock
							&& (is_left_shift_pressed || is_right_shift_pressed))
							|| (caps_lock && (!is_left_shift_pressed) && (!is_right_shift_pressed));
						let is_shift_pressed = is_left_shift_pressed || is_right_shift_pressed;
						let real_character = match key_code {
							"Grave" => {
								if is_shift_pressed {
									"~"
								} else {
									"`"
								}
							}
							"Key1" => {
								if is_shift_pressed {
									"!"
								} else {
									"1"
								}
							}
							"Key2" => {
								if is_shift_pressed {
									"@"
								} else {
									"2"
								}
							}
							"Key3" => {
								if is_shift_pressed {
									"#"
								} else {
									"3"
								}
							}
							"Key4" => {
								if is_shift_pressed {
									"$"
								} else {
									"4"
								}
							}
							"Key5" => {
								if is_shift_pressed {
									"%"
								} else {
									"5"
								}
							}
							"Key6" => {
								if is_shift_pressed {
									"^"
								} else {
									"6"
								}
							}
							"Key7" => {
								if is_shift_pressed {
									"&"
								} else {
									"7"
								}
							}
							"Key8" => {
								if is_shift_pressed {
									"*"
								} else {
									"8"
								}
							}
							"Key9" => {
								if is_shift_pressed {
									"("
								} else {
									"9"
								}
							}
							"Key0" => {
								if is_shift_pressed {
									")"
								} else {
									"0"
								}
							}
							"Minus" => {
								if is_shift_pressed {
									"_"
								} else {
									"-"
								}
							}
							"Equal" => {
								if is_shift_pressed {
									"+"
								} else {
									"="
								}
							}
							"LeftBracket" => {
								if is_shift_pressed {
									"{"
								} else {
									"["
								}
							}
							"RightBracket" => {
								if is_shift_pressed {
									"}"
								} else {
									"]"
								}
							}
							"BackSlash" => {
								if is_shift_pressed {
									"|"
								} else {
									"\\"
								}
							}
							"Semicolon" => {
								if is_shift_pressed {
									":"
								} else {
									";"
								}
							}
							"Apostrophe" => {
								if is_shift_pressed {
									"\""
								} else {
									"'"
								}
							}
							"Comma" => {
								if is_shift_pressed {
									"<"
								} else {
									","
								}
							}
							"Dot" => {
								if is_shift_pressed {
									">"
								} else {
									"."
								}
							}
							"Slash" => {
								if is_shift_pressed {
									"?"
								} else {
									"/"
								}
							}
							"A" => {
								if is_upper {
									"A"
								} else {
									"a"
								}
							}
							"B" => {
								if is_upper {
									"B"
								} else {
									"b"
								}
							}
							"C" => {
								if is_upper {
									"C"
								} else {
									"c"
								}
							}
							"D" => {
								if is_upper {
									"D"
								} else {
									"d"
								}
							}
							"E" => {
								if is_upper {
									"E"
								} else {
									"e"
								}
							}
							"F" => {
								if is_upper {
									"F"
								} else {
									"f"
								}
							}
							"G" => {
								if is_upper {
									"G"
								} else {
									"g"
								}
							}
							"H" => {
								if is_upper {
									"H"
								} else {
									"h"
								}
							}
							"I" => {
								if is_upper {
									"I"
								} else {
									"i"
								}
							}
							"J" => {
								if is_upper {
									"J"
								} else {
									"j"
								}
							}
							"K" => {
								if is_upper {
									"K"
								} else {
									"k"
								}
							}
							"L" => {
								if is_upper {
									"L"
								} else {
									"l"
								}
							}
							"M" => {
								if is_upper {
									"M"
								} else {
									"m"
								}
							}
							"N" => {
								if is_upper {
									"N"
								} else {
									"n"
								}
							}
							"O" => {
								if is_upper {
									"O"
								} else {
									"o"
								}
							}
							"P" => {
								if is_upper {
									"P"
								} else {
									"p"
								}
							}
							"Q" => {
								if is_upper {
									"Q"
								} else {
									"q"
								}
							}
							"R" => {
								if is_upper {
									"R"
								} else {
									"r"
								}
							}
							"S" => {
								if is_upper {
									"S"
								} else {
									"s"
								}
							}
							"T" => {
								if is_upper {
									"T"
								} else {
									"t"
								}
							}
							"U" => {
								if is_upper {
									"U"
								} else {
									"u"
								}
							}
							"V" => {
								if is_upper {
									"V"
								} else {
									"v"
								}
							}
							"W" => {
								if is_upper {
									"W"
								} else {
									"w"
								}
							}
							"X" => {
								if is_upper {
									"X"
								} else {
									"x"
								}
							}
							"Y" => {
								if is_upper {
									"Y"
								} else {
									"y"
								}
							}
							"Z" => {
								if is_upper {
									"Z"
								} else {
									"z"
								}
							}
							"Enter" => "\\n",
							"Tab" => "\\t",
							&_ => "Unknown",
						};

						let event_info =
							format!("[{}] Input Character '{}'", key_time, real_character);
						let label = Label::builder()
							.label(event_info.as_str())
							.halign(Align::Start)
							.build();
						list_box_of_key_pressing.insert(&label, -1);
					}
				}
			}
		}
	}

	main_vbox.append(&list_box_of_key_pressing);

	let scrolled_window = ScrolledWindow::new();
	scrolled_window.set_child(Some(&main_vbox));

	event_view_window.set_child(Some(&scrolled_window));

	application.add_window(&event_view_window);

	event_view_window
}
