static mut MOUSE_MOVING_VELOCITY_LIST: Vec<f64> = Vec::new();
static mut MOUSE_MOVING_VELOCITY_MAX: f64 = 0.0;
static mut MOUSE_MOVING_ACCELERATION_LIST: Vec<f64> = Vec::new();
static mut MOUSE_MOVING_ACCELERATION_MAX: f64 = 0.0;

static mut KEYBOARD_PRESSING_VELOCITY_LIST: Vec<f64> = Vec::new();
static mut KEYBOARD_PRESSING_VELOCITY_MAX: f64 = 0.0;
static mut KEYBOARD_PRESSING_ACCELERATION_LIST: Vec<f64> = Vec::new();
static mut KEYBOARD_PRESSING_ACCELERATION_MAX: f64 = 0.0;

static mut MOUSE_PRESSING_VELOCITY_LIST: Vec<f64> = Vec::new();
static mut MOUSE_PRESSING_VELOCITY_MAX: f64 = 0.0;
static mut MOUSE_PRESSING_ACCELERATION_LIST: Vec<f64> = Vec::new();
static mut MOUSE_PRESSING_ACCELERATION_MAX: f64 = 0.0;

static mut KEYBOARD_PRESSED_TIME_LIST: Vec<f64> = Vec::new();
static mut KEYBOARD_PRESSED_TIME_MAX: f64 = 0.0;

static mut MOUSE_PRESSED_TIME_LIST: Vec<f64> = Vec::new();
static mut MOUSE_PRESSED_TIME_MAX: f64 = 0.0;

fn create_event_analyse_window(application: &Application) -> ApplicationWindow {
	let event_analyse_window = ApplicationWindow::builder()
		.title("Monitor Input Device -- Event analyse")
		.default_width(1280)
		.default_height(800)
		.build();

	/* Mouse Moving */
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
			.transient_for(&event_analyse_window)
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

		unsafe {
			MOUSE_MOVING_VELOCITY_LIST.clear();
			MOUSE_MOVING_ACCELERATION_LIST.clear();
		}
		let mut prev_time = Local
			.datetime_from_str(
				&mouse_event_data_list[0]["time"].as_str().unwrap(),
				"%Y-%m-%d %H:%M:%S%.f %z",
			)
			.unwrap();
		let mut j = -1;
		for i in 1..mouse_event_data_list.len() {
			let mouse_event_data = &mouse_event_data_list[i];

			let event_time_str = mouse_event_data["time"].as_str().unwrap();
			let event_original_position_str =
				mouse_event_data["original_position"].as_str().unwrap();
			let time = Local
				.datetime_from_str(event_time_str, "%Y-%m-%d %H:%M:%S%.f %z")
				.unwrap();
			let event_new_position_str = mouse_event_data["new_position"].as_str().unwrap();

			let position: Vec<&str> = event_original_position_str
				.trim_matches('(')
				.trim_matches(')')
				.split(',')
				.collect();
			let x1 = String::from(position[0].trim()).parse::<i32>().unwrap();
			let y1 = String::from(position[1].trim()).parse::<i32>().unwrap();
			let position: Vec<&str> = event_new_position_str
				.trim_matches('(')
				.trim_matches(')')
				.split(',')
				.collect();
			let x2 = String::from(position[0].trim()).parse::<i32>().unwrap();
			let y2 = String::from(position[1].trim()).parse::<i32>().unwrap();
			let dx = (x2 - x1).abs();
			let dy = (y2 - y1).abs();
			let distance = ((dx * dx + dy * dy) as f64).sqrt();
			let nanosecond_count_of_timespan = (time - prev_time).num_nanoseconds().unwrap();
			let velocity_max_theoretical_value = 100000.0;
			let mut velocity = distance * 1000_000_000f64 / (nanosecond_count_of_timespan as f64);
			velocity = if velocity < velocity_max_theoretical_value {
				velocity
			} else {
				// 100000.0
				match unsafe { MOUSE_MOVING_VELOCITY_LIST.last() } {
					Some(v) => *v,
					None => velocity_max_theoretical_value,
				}
			};
			unsafe {
				if velocity < velocity_max_theoretical_value {
					MOUSE_MOVING_VELOCITY_MAX = if velocity > MOUSE_MOVING_VELOCITY_MAX {
						velocity
					} else {
						MOUSE_MOVING_VELOCITY_MAX
					};
					MOUSE_MOVING_VELOCITY_LIST.push(velocity);
				}
			}

			j += 1;
			if j > 0 {
				unsafe {
					let prev_velocity =
						MOUSE_MOVING_VELOCITY_LIST[MOUSE_MOVING_VELOCITY_LIST.len() - 2];
					let mut acceleration = (velocity - prev_velocity) * 1000_000_000f64
						/ (nanosecond_count_of_timespan as f64);

					acceleration = acceleration.abs();
					let acceleration_max_theoretical_value = 10000000.0;
					acceleration = if acceleration < acceleration_max_theoretical_value {
						acceleration
					} else {
						// 10000000.0
						match MOUSE_MOVING_ACCELERATION_LIST.last() {
							Some(v) => *v,
							None => acceleration_max_theoretical_value,
						}
					};

					if acceleration < acceleration_max_theoretical_value {
						MOUSE_MOVING_ACCELERATION_MAX =
							if acceleration > MOUSE_MOVING_ACCELERATION_MAX {
								acceleration
							} else {
								MOUSE_MOVING_ACCELERATION_MAX
							};

						MOUSE_MOVING_ACCELERATION_LIST.push(acceleration);
					}
				}
			}

			prev_time = time;
		}
	}

	/* Key Pressing */
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
			.transient_for(&event_analyse_window)
			.modal(true)
			.buttons(ButtonsType::Ok)
			.text("Infomation")
			.secondary_text(key_message)
			.build();

		info_dialog.run_async(move |obj, _| {
			obj.close();
		});
	} else {
		unsafe {
			KEYBOARD_PRESSING_VELOCITY_LIST.clear();
			KEYBOARD_PRESSING_ACCELERATION_LIST.clear();
			MOUSE_PRESSING_VELOCITY_LIST.clear();
			MOUSE_PRESSING_ACCELERATION_LIST.clear();
			KEYBOARD_PRESSED_TIME_LIST.clear();
			MOUSE_PRESSED_TIME_LIST.clear();
		}

		let key_returned_value = &key_response_jobject["response"]["returned_value"];
		let _key_event_data_list_count = &key_returned_value["event_data_list_count"];
		let key_event_data_list = &key_returned_value["event_data_list"];

		let mut pressed_key_hashmap: HashMap<String, chrono::DateTime<chrono::Local>> =
			HashMap::new();
		let task_start_time = unsafe { TASK_START_TIME.unwrap() };
		let mut j_keyboard = -1;
		let mut j_mouse = -1;
		for i in 0..key_event_data_list.len() {
			let key_event_data = &key_event_data_list[i];
			let key_time = key_event_data["time"].as_str().unwrap();
			let key_name = key_event_data["key_name"].as_str().unwrap();
			let status = if key_event_data["status"].as_u8().unwrap() == 1 {
				"Pressed"
			} else {
				"Released"
			};

			if status == "Pressed" {
				let start_time = Local
					.datetime_from_str(key_time, "%Y-%m-%d %H:%M:%S%.f %z")
					.unwrap();
				pressed_key_hashmap.insert(String::from(key_name), start_time);

				if String::from(key_name).contains("Keyboard") {
					let current_count = unsafe { KEYBOARD_PRESSING_VELOCITY_LIST.len() };
					let nanosecond_count_of_timespan =
						(start_time - task_start_time).num_nanoseconds().unwrap();
					let velocity = ((current_count + 1) as f64) * 1000_000_000f64 * 60.0
						/ (nanosecond_count_of_timespan as f64);
					unsafe {
						KEYBOARD_PRESSING_VELOCITY_MAX =
							if velocity > KEYBOARD_PRESSING_VELOCITY_MAX {
								velocity
							} else {
								KEYBOARD_PRESSING_VELOCITY_MAX
							};
					}

					unsafe { KEYBOARD_PRESSING_VELOCITY_LIST.push(velocity) }

					j_keyboard += 1;
					if j_keyboard > 0 {
						unsafe {
							let prev_velocity = KEYBOARD_PRESSING_VELOCITY_LIST
								[KEYBOARD_PRESSING_VELOCITY_LIST.len() - 2];
							let mut acceleration = (velocity - prev_velocity) * 1000_000_000f64
								/ (nanosecond_count_of_timespan as f64);

							acceleration = acceleration.abs();
							let acceleration_max_theoretical_value = 100.0;
							acceleration = if acceleration < acceleration_max_theoretical_value {
								acceleration
							} else {
								// 100.0
								match KEYBOARD_PRESSING_ACCELERATION_LIST.last() {
									Some(v) => *v,
									None => acceleration_max_theoretical_value,
								}
							};

							if acceleration < acceleration_max_theoretical_value {
								KEYBOARD_PRESSING_ACCELERATION_MAX =
									if acceleration > KEYBOARD_PRESSING_ACCELERATION_MAX {
										acceleration
									} else {
										KEYBOARD_PRESSING_ACCELERATION_MAX
									};

								KEYBOARD_PRESSING_ACCELERATION_LIST.push(acceleration);
							}
						}
					}
				} else {
					// MOUSE_PRESSING_VELOCITY_LIST
					let current_count = unsafe { MOUSE_PRESSING_VELOCITY_LIST.len() };
					let nanosecond_count_of_timespan =
						(start_time - task_start_time).num_nanoseconds().unwrap();
					let velocity = ((current_count + 1) as f64) * 1000_000_000f64 * 60.0
						/ (nanosecond_count_of_timespan as f64);
					unsafe {
						MOUSE_PRESSING_VELOCITY_MAX = if velocity > MOUSE_PRESSING_VELOCITY_MAX {
							velocity
						} else {
							MOUSE_PRESSING_VELOCITY_MAX
						};
					}

					unsafe { MOUSE_PRESSING_VELOCITY_LIST.push(velocity) }

					j_mouse += 1;
					if j_mouse > 0 {
						unsafe {
							let prev_velocity = MOUSE_PRESSING_VELOCITY_LIST
								[MOUSE_PRESSING_VELOCITY_LIST.len() - 2];
							let mut acceleration = (velocity - prev_velocity) * 1000_000_000f64
								/ (nanosecond_count_of_timespan as f64);

							acceleration = acceleration.abs();
							let acceleration_max_theoretical_value = 100.0;
							acceleration = if acceleration < acceleration_max_theoretical_value {
								acceleration
							} else {
								// 100.0
								match MOUSE_PRESSING_ACCELERATION_LIST.last() {
									Some(v) => *v,
									None => acceleration_max_theoretical_value,
								}
							};

							if acceleration < acceleration_max_theoretical_value {
								MOUSE_PRESSING_ACCELERATION_MAX =
									if acceleration > MOUSE_PRESSING_ACCELERATION_MAX {
										acceleration
									} else {
										MOUSE_PRESSING_ACCELERATION_MAX
									};

								MOUSE_PRESSING_ACCELERATION_LIST.push(acceleration);
							}
						}
					}
				}
			} else {
				let key = String::from(key_name);

				if pressed_key_hashmap.contains_key(&key) {
					let start_time = pressed_key_hashmap.remove(&String::from(key_name)).unwrap();
					let end_time = Local
						.datetime_from_str(key_time, "%Y-%m-%d %H:%M:%S%.f %z")
						.unwrap();
					let duration = end_time - start_time;

					let pressed_time =
						(duration.num_nanoseconds().unwrap() as f64) / 1000_000_000f64;
					unsafe {
						if String::from(key_name).contains("Keyboard") {
							KEYBOARD_PRESSED_TIME_MAX = if pressed_time > KEYBOARD_PRESSED_TIME_MAX
							{
								pressed_time
							} else {
								KEYBOARD_PRESSED_TIME_MAX
							};
							KEYBOARD_PRESSED_TIME_LIST.push(pressed_time);
						} else {
							MOUSE_PRESSED_TIME_MAX = if pressed_time > MOUSE_PRESSED_TIME_MAX {
								pressed_time
							} else {
								MOUSE_PRESSED_TIME_MAX
							};
							MOUSE_PRESSED_TIME_LIST.push(pressed_time);
						}
					}
				}
			}
		}
	}

	let notebook = Notebook::builder().tab_pos(PositionType::Top).build();

	let drawing_area_of_mouse_moving_velocity = DrawingArea::builder()
		.margin_start(50)
		.margin_end(50)
		.margin_top(50)
		.margin_bottom(50)
		.content_width(100)
		.content_height(100)
		.build();
	drawing_area_of_mouse_moving_velocity.set_draw_func(draw_function_of_mouse_moving_velocity);
	let label = Label::builder().label("Mouse Moving Velocity").build();
	notebook.append_page(&drawing_area_of_mouse_moving_velocity, Some(&label));

	let drawing_area_of_mouse_moving_acceleration = DrawingArea::builder()
		.margin_start(50)
		.margin_end(50)
		.margin_top(50)
		.margin_bottom(50)
		.content_width(100)
		.content_height(100)
		.build();
	drawing_area_of_mouse_moving_acceleration
		.set_draw_func(draw_function_of_mouse_moving_acceleration);
	let label = Label::builder().label("Mouse Moving Acceleration").build();
	notebook.append_page(&drawing_area_of_mouse_moving_acceleration, Some(&label));

	let drawing_area_of_mouse_click_velocity = DrawingArea::builder()
		.margin_start(50)
		.margin_end(50)
		.margin_top(50)
		.margin_bottom(50)
		.content_width(100)
		.content_height(100)
		.build();
	drawing_area_of_mouse_click_velocity.set_draw_func(draw_function_of_mouse_click_velocity);
	let label = Label::builder().label("Mouse Click Velocity").build();
	notebook.append_page(&drawing_area_of_mouse_click_velocity, Some(&label));

	let drawing_area_of_mouse_click_acceleration = DrawingArea::builder()
		.margin_start(50)
		.margin_end(50)
		.margin_top(50)
		.margin_bottom(50)
		.content_width(100)
		.content_height(100)
		.build();
	drawing_area_of_mouse_click_acceleration
		.set_draw_func(draw_function_of_mouse_click_acceleration);
	let label = Label::builder().label("Mouse Click Acceleration").build();
	notebook.append_page(&drawing_area_of_mouse_click_acceleration, Some(&label));

	let drawing_area_of_keyboard_pressing_velocity = DrawingArea::builder()
		.margin_start(50)
		.margin_end(50)
		.margin_top(50)
		.margin_bottom(50)
		.content_width(100)
		.content_height(100)
		.build();
	drawing_area_of_keyboard_pressing_velocity
		.set_draw_func(draw_function_of_keyboard_pressing_velocity);
	let label = Label::builder().label("Keyboard Pressing Velocity").build();
	notebook.append_page(&drawing_area_of_keyboard_pressing_velocity, Some(&label));

	let drawing_area_of_keyboard_pressing_acceleration = DrawingArea::builder()
		.margin_start(50)
		.margin_end(50)
		.margin_top(50)
		.margin_bottom(50)
		.content_width(100)
		.content_height(100)
		.build();
	drawing_area_of_keyboard_pressing_acceleration
		.set_draw_func(draw_function_of_keyboard_pressing_acceleration);
	let label = Label::builder()
		.label("Keyboard Pressing Acceleration")
		.build();
	notebook.append_page(
		&drawing_area_of_keyboard_pressing_acceleration,
		Some(&label),
	);

	let drawing_area_of_keyboard_pressed_time = DrawingArea::builder()
		.margin_start(50)
		.margin_end(50)
		.margin_top(50)
		.margin_bottom(50)
		.content_width(100)
		.content_height(100)
		.build();
	drawing_area_of_keyboard_pressed_time.set_draw_func(draw_function_of_keyboard_pressed_time);
	let label = Label::builder().label("Keyboard Pressed Time").build();
	notebook.append_page(&drawing_area_of_keyboard_pressed_time, Some(&label));

	let drawing_area_of_mouse_pressed_time = DrawingArea::builder()
		.margin_start(50)
		.margin_end(50)
		.margin_top(50)
		.margin_bottom(50)
		.content_width(100)
		.content_height(100)
		.build();
	drawing_area_of_mouse_pressed_time.set_draw_func(draw_function_of_mouse_pressed_time);
	let label = Label::builder().label("Mouse Pressed Time").build();
	notebook.append_page(&drawing_area_of_mouse_pressed_time, Some(&label));

	event_analyse_window.set_child(Some(&notebook));

	application.add_window(&event_analyse_window);

	event_analyse_window
}

fn draw_function_of_mouse_moving_velocity(
	drawing_area: &DrawingArea,
	content: &Context,
	width: i32,
	height: i32,
) {
	content.set_line_width(1.0);
	content.set_source_rgb(255.0, 0.0, 0.0);

	content.move_to(0.0, 0.0);
	content.line_to(0.0, height as f64);
	content.line_to(width as f64, height as f64);

	let count = unsafe { MOUSE_MOVING_VELOCITY_LIST.len() };
	if count > 0 {
		let step_of_horizontal = (count as f64) / (width as f64);
		let unit_of_vertical = unsafe { (height as f64) / MOUSE_MOVING_VELOCITY_MAX };
		content.move_to(0.0, unsafe { MOUSE_MOVING_VELOCITY_LIST[0] });

		let mut i = 0;
		while i < width {
			let index = ((i as f64) * step_of_horizontal) as usize;
			if index >= unsafe { MOUSE_MOVING_VELOCITY_LIST.len() } {
				break;
			}
			let velocity = unsafe { MOUSE_MOVING_VELOCITY_LIST[index] };
			content.line_to(i as f64, height as f64 - velocity * unit_of_vertical);

			if step_of_horizontal <= 1.0 {
				i += (1.0 / step_of_horizontal) as i32;
			} else {
				i += 1;
			}
		}
	}
	content.stroke().unwrap();
}

fn draw_function_of_mouse_moving_acceleration(
	drawing_area: &DrawingArea,
	content: &Context,
	width: i32,
	height: i32,
) {
	content.set_line_width(1.0);
	content.set_source_rgb(255.0, 0.0, 0.0);

	content.move_to(0.0, 0.0);
	content.line_to(0.0, height as f64);
	content.line_to(width as f64, height as f64);

	let count = unsafe { MOUSE_MOVING_ACCELERATION_LIST.len() };
	if count > 0 {
		let step_of_horizontal = (count as f64) / (width as f64);
		let unit_of_vertical = unsafe { (height as f64) / MOUSE_MOVING_ACCELERATION_MAX };
		content.move_to(0.0, unsafe { MOUSE_MOVING_ACCELERATION_LIST[0] });

		let mut i = 0;
		while i < width {
			let index = ((i as f64) * step_of_horizontal) as usize;
			if index >= unsafe { MOUSE_MOVING_ACCELERATION_LIST.len() } {
				break;
			}
			let acceleration = unsafe { MOUSE_MOVING_ACCELERATION_LIST[index] };
			content.line_to(i as f64, height as f64 - acceleration * unit_of_vertical);

			if step_of_horizontal <= 1.0 {
				i += (1.0 / step_of_horizontal) as i32;
			} else {
				i += 1;
			}
		}
	}
	content.stroke().unwrap();
}

fn draw_function_of_mouse_click_velocity(
	drawing_area: &DrawingArea,
	content: &Context,
	width: i32,
	height: i32,
) {
	content.set_line_width(1.0);
	content.set_source_rgb(255.0, 0.0, 0.0);

	content.move_to(0.0, 0.0);
	content.line_to(0.0, height as f64);
	content.line_to(width as f64, height as f64);

	let count = unsafe { MOUSE_PRESSING_VELOCITY_LIST.len() };
	if count > 0 {
		let step_of_horizontal = (count as f64) / (width as f64);
		let unit_of_vertical = unsafe { (height as f64) / MOUSE_PRESSING_VELOCITY_MAX };
		content.move_to(0.0, unsafe { MOUSE_PRESSING_VELOCITY_LIST[0] });

		let mut i = 0;
		while i < width {
			let index = ((i as f64) * step_of_horizontal) as usize;
			if index >= unsafe { MOUSE_PRESSING_VELOCITY_LIST.len() } {
				break;
			}
			let velocity = unsafe { MOUSE_PRESSING_VELOCITY_LIST[index] };
			content.line_to(i as f64, height as f64 - velocity * unit_of_vertical);

			if step_of_horizontal <= 1.0 {
				i += (1.0 / step_of_horizontal) as i32;
			} else {
				i += 1;
			}
		}
	}

	content.stroke().unwrap();
}

fn draw_function_of_mouse_click_acceleration(
	drawing_area: &DrawingArea,
	content: &Context,
	width: i32,
	height: i32,
) {
	content.set_line_width(1.0);
	content.set_source_rgb(255.0, 0.0, 0.0);

	content.move_to(0.0, 0.0);
	content.line_to(0.0, height as f64);
	content.line_to(width as f64, height as f64);

	content.set_line_width(1.0);
	content.set_source_rgb(255.0, 0.0, 0.0);

	content.move_to(0.0, 0.0);
	content.line_to(0.0, height as f64);
	content.line_to(width as f64, height as f64);

	let count = unsafe { MOUSE_PRESSING_ACCELERATION_LIST.len() };
	if count > 0 {
		let step_of_horizontal = (count as f64) / (width as f64);
		let unit_of_vertical = unsafe { (height as f64) / MOUSE_PRESSING_ACCELERATION_MAX };
		content.move_to(0.0, unsafe { MOUSE_PRESSING_ACCELERATION_LIST[0] });

		let mut i = 0;
		while i < width {
			let index = ((i as f64) * step_of_horizontal) as usize;
			if index >= unsafe { MOUSE_PRESSING_ACCELERATION_LIST.len() } {
				break;
			}
			let acceleration = unsafe { MOUSE_PRESSING_ACCELERATION_LIST[index] };
			content.line_to(i as f64, height as f64 - acceleration * unit_of_vertical);

			if step_of_horizontal <= 1.0 {
				i += (1.0 / step_of_horizontal) as i32;
			} else {
				i += 1;
			}
		}
	}

	content.stroke().unwrap();
}

fn draw_function_of_keyboard_pressing_velocity(
	drawing_area: &DrawingArea,
	content: &Context,
	width: i32,
	height: i32,
) {
	content.set_line_width(1.0);
	content.set_source_rgb(255.0, 0.0, 0.0);

	content.move_to(0.0, 0.0);
	content.line_to(0.0, height as f64);
	content.line_to(width as f64, height as f64);

	let count = unsafe { KEYBOARD_PRESSING_VELOCITY_LIST.len() };
	if count > 0 {
		let step_of_horizontal = (count as f64) / (width as f64);
		let unit_of_vertical = unsafe { (height as f64) / KEYBOARD_PRESSING_VELOCITY_MAX };
		content.move_to(0.0, unsafe { KEYBOARD_PRESSING_VELOCITY_LIST[0] });

		let mut i = 0;
		while i < width {
			let index = ((i as f64) * step_of_horizontal) as usize;
			if index >= unsafe { KEYBOARD_PRESSING_VELOCITY_LIST.len() } {
				break;
			}
			let velocity = unsafe { KEYBOARD_PRESSING_VELOCITY_LIST[index] };
			content.line_to(i as f64, height as f64 - velocity * unit_of_vertical);

			if step_of_horizontal <= 1.0 {
				i += (1.0 / step_of_horizontal) as i32;
			} else {
				i += 1;
			}
		}
	}

	content.stroke().unwrap();
}

fn draw_function_of_keyboard_pressing_acceleration(
	drawing_area: &DrawingArea,
	content: &Context,
	width: i32,
	height: i32,
) {
	content.set_line_width(1.0);
	content.set_source_rgb(255.0, 0.0, 0.0);

	content.move_to(0.0, 0.0);
	content.line_to(0.0, height as f64);
	content.line_to(width as f64, height as f64);

	let count = unsafe { KEYBOARD_PRESSING_ACCELERATION_LIST.len() };
	if count > 0 {
		let step_of_horizontal = (count as f64) / (width as f64);
		let unit_of_vertical = unsafe { (height as f64) / KEYBOARD_PRESSING_ACCELERATION_MAX };
		content.move_to(0.0, unsafe { KEYBOARD_PRESSING_ACCELERATION_LIST[0] });

		let mut i = 0;
		while i < width {
			let index = ((i as f64) * step_of_horizontal) as usize;
			if index >= unsafe { KEYBOARD_PRESSING_ACCELERATION_LIST.len() } {
				break;
			}
			let acceleration = unsafe { KEYBOARD_PRESSING_ACCELERATION_LIST[index] };
			content.line_to(i as f64, height as f64 - acceleration * unit_of_vertical);

			if step_of_horizontal <= 1.0 {
				i += (1.0 / step_of_horizontal) as i32;
			} else {
				i += 1;
			}
		}
	}

	content.stroke().unwrap();
}

fn draw_function_of_keyboard_pressed_time(
	drawing_area: &DrawingArea,
	content: &Context,
	width: i32,
	height: i32,
) {
	content.set_line_width(1.0);
	content.set_source_rgb(255.0, 0.0, 0.0);

	content.move_to(0.0, 0.0);
	content.line_to(0.0, height as f64);
	content.line_to(width as f64, height as f64);

	let count = unsafe { KEYBOARD_PRESSING_VELOCITY_LIST.len() };
	if count > 0 {
		let step_of_horizontal = (count as f64) / (width as f64);
		let unit_of_vertical = unsafe { (height as f64) / KEYBOARD_PRESSED_TIME_MAX };
		content.move_to(0.0, unsafe { KEYBOARD_PRESSED_TIME_LIST[0] });

		let mut prev_real_height = 0.0;
		let mut i = 0;
		while i < width {
			let index = ((i as f64) * step_of_horizontal) as usize;
			if index >= unsafe { KEYBOARD_PRESSED_TIME_LIST.len() } {
				break;
			}
			let pressed_time = unsafe { KEYBOARD_PRESSED_TIME_LIST[index] };
			let current_real_height = height as f64 - pressed_time * unit_of_vertical;
			if current_real_height == prev_real_height {
				content.line_to(i as f64, current_real_height);
			} else {
				content.line_to(i as f64, height as f64);
				content.line_to(i as f64, current_real_height);
				prev_real_height = current_real_height;
			}

			i += 1;
		}
		content.line_to(i as f64, height as f64);
	}

	content.stroke().unwrap();
}

fn draw_function_of_mouse_pressed_time(
	drawing_area: &DrawingArea,
	content: &Context,
	width: i32,
	height: i32,
) {
	content.set_line_width(1.0);
	content.set_source_rgb(255.0, 0.0, 0.0);

	content.move_to(0.0, 0.0);
	content.line_to(0.0, height as f64);
	content.line_to(width as f64, height as f64);

	let count = unsafe { MOUSE_PRESSING_VELOCITY_LIST.len() };
	if count > 0 {
		let step_of_horizontal = (count as f64) / (width as f64);
		let unit_of_vertical = unsafe { (height as f64) / MOUSE_PRESSED_TIME_MAX };
		content.move_to(0.0, unsafe { MOUSE_PRESSED_TIME_LIST[0] });

		let mut prev_real_height = 0.0;
		let mut i = 0;
		while i < width {
			let index = ((i as f64) * step_of_horizontal) as usize;
			if index >= unsafe { MOUSE_PRESSED_TIME_LIST.len() } {
				break;
			}
			let pressed_time = unsafe { MOUSE_PRESSED_TIME_LIST[index] };
			let current_real_height = height as f64 - pressed_time * unit_of_vertical;
			if current_real_height == prev_real_height {
				content.line_to(i as f64, current_real_height);
			} else {
				content.line_to(i as f64, height as f64);
				content.line_to(i as f64, current_real_height);
				prev_real_height = current_real_height;
			}

			i += 1;
		}
		content.line_to(i as f64, height as f64);
	}

	content.stroke().unwrap();
}
