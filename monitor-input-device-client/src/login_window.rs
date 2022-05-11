include!("register_window.rs");
include!("operate_window.rs");

fn create_login_window(application: &Application) -> ApplicationWindow {
	let login_window = ApplicationWindow::builder()
		.title("Monitor Input Device -- User login")
		.resizable(false)
		.build();

	let main_vbox = Box::builder()
		.margin_start(50)
		.margin_end(50)
		.margin_top(10)
		.margin_bottom(30)
		.orientation(Orientation::Vertical)
		.build();

	/* title */
	let title_label = Label::builder()
		.label("Monitor Input Device")
		.valign(Align::Center)
		.margin_top(30)
		.build();

	let mut font_desc = FontDescription::new();
	font_desc.set_family("Noto Serif CJK TC");
	font_desc.set_weight(pango::Weight::Heavy);
	font_desc.set_size(30 * 1024);
	let attr_font_desc = AttrFontDesc::new(&font_desc);
	let attr_list = AttrList::new();
	attr_list.insert(attr_font_desc);
	title_label.set_attributes(Some(&attr_list));

	main_vbox.append(&title_label);

	/* form-box */

	let form_vbox = Box::builder()
		.margin_start(60)
		.margin_end(60)
		.margin_top(30)
		.orientation(Orientation::Vertical)
		.build();
	/*** username ***/
	/**** label for username ****/
	let username_label = Label::builder()
		.label("User Name")
		.halign(Align::Start)
		.build();
	let mut font_desc = FontDescription::new();
	font_desc.set_family("Noto Serif CJK TC");
	font_desc.set_weight(pango::Weight::Heavy);
	font_desc.set_size(18 * 1024);
	let attr_font_desc = AttrFontDesc::new(&font_desc);
	let attr_list = AttrList::new();
	attr_list.insert(attr_font_desc);
	username_label.set_attributes(Some(&attr_list));

	form_vbox.append(&username_label);

	/**** textbox for username ****/
	let username_entry = Entry::builder()
		.placeholder_text("Please input user name.")
		.margin_top(10)
		.text("")
		.build();

	form_vbox.append(&username_entry);

	/*** password ***/
	/**** label for password ****/
	let password_label = Label::builder()
		.label("Password")
		.halign(Align::Start)
		.margin_top(15)
		.build();
	let mut font_desc = FontDescription::new();
	font_desc.set_family("Noto Serif CJK TC");
	font_desc.set_weight(pango::Weight::Heavy);
	font_desc.set_size(18 * 1024);
	let attr_font_desc = AttrFontDesc::new(&font_desc);
	let attr_list = AttrList::new();
	attr_list.insert(attr_font_desc);
	password_label.set_attributes(Some(&attr_list));

	form_vbox.append(&password_label);

	/**** textbox for password ****/
	let password_entry = PasswordEntry::builder()
		.placeholder_text("Please input password.")
		.margin_top(10)
		.text("")
		.build();

	form_vbox.append(&password_entry);

	/*** Buttons Box ***/
	let buttons_hbox = Box::builder().margin_top(30).homogeneous(true).build();

	/*** Login Button ***/
	let login_button = Button::builder()
		.label("Login")
		.margin_start(50)
		.margin_end(50)
		.build();

	login_button.connect_clicked(clone!(@weak login_window, @weak application => move |_| {
		let username = String::from(username_entry.text().as_str());
		let password = String::from(password_entry.text().as_str());

		if username == ""{
			let warning_dialog = MessageDialog::builder()
				.transient_for(&login_window)
				.modal(true)
				.buttons(ButtonsType::Ok)
				.text("Warning")
				.secondary_text("You should input the user name.")
				.build();

			warning_dialog.run_async(move |obj, _|{
				obj.close();
			});

			username_entry.grab_focus();

			return;
		}

		if password == ""{
			let warning_dialog = MessageDialog::builder()
				.transient_for(&login_window)
				.modal(true)
				.buttons(ButtonsType::Ok)
				.text("Warning")
				.secondary_text("You should input the password.")
				.build();

			warning_dialog.run_async(move |obj, _|{
				obj.close();
			});

			password_entry.grab_focus();

			return;
		}

		let response_jobject = login(&username, &password);
		println!("{}", response_jobject);
		let response = &response_jobject["response"];
		let is_success = response["is_success"].as_bool().unwrap();
		let message = response["message"].as_str().unwrap();
		let returned_value = &response["returned_value"];
		if is_success{
			let token_id = returned_value["token_id"].as_str().unwrap();
			let token_string = String::from(token_id);

			let mut user_id_list:Vec<String> = Vec::new();
			let db_file = String::from(DB_FILE);
			let connection = sqlite::open(&*db_file).unwrap();
			let sql = "SELECT ID FROM User;";
			connection.iterate(sql, |pairs|{
				for &(column, value) in pairs.iter(){
					if column == "ID"{
						let id = String::from(value.unwrap());
						user_id_list.push(id);
					}
				}
				true
			}).unwrap();

			let user_id = user_id_list.get(0).unwrap().clone();

			unsafe{
				TOKEN_ID = Some(token_string);
				println!("token_id : {}", TOKEN_ID.clone().unwrap());

				USER_ID = Some(user_id);
				PASSWORD = Some(password);
				NET_IS_ENABLED = true;
			}
		}

		let info_dialog = MessageDialog::builder()
			.transient_for(&login_window)
			.modal(true)
			.buttons(ButtonsType::Ok)
			.text("Infomation")
			.secondary_text(message)
			.build();

		info_dialog.run_async(move |obj, _|{
			obj.close();

			if is_success{
				login_window.close();

				let operate_window = create_operate_window(&application);
				operate_window.present();
			}
		});
	}));

	buttons_hbox.append(&login_button);

	/*** Reset Button ***/
	let register_button = Button::builder()
		.label("Register")
		.margin_start(50)
		.margin_end(50)
		.build();

	register_button.connect_clicked(clone!(@weak application, @weak login_window => move |_| {
		login_window.close();

		let register_window = create_register_window(&application);
		register_window.present();
	}));

	buttons_hbox.append(&register_button);

	form_vbox.append(&buttons_hbox);

	main_vbox.append(&form_vbox);

	login_window.set_child(Some(&main_vbox));

	application.add_window(&login_window);

	login_window
}
