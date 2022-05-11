fn create_register_window(application: &Application) -> ApplicationWindow {
	let register_window = ApplicationWindow::builder()
		.title("Monitor Input Device -- User register")
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
		.show_peek_icon(true)
		.build();

	form_vbox.append(&password_entry);

	/*** password again ***/
	/**** label for password again ****/
	let password_again_label = Label::builder()
		.label("Password Again")
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
	password_again_label.set_attributes(Some(&attr_list));

	form_vbox.append(&password_again_label);

	/**** textbox for password again ****/
	let password_again_entry = PasswordEntry::builder()
		.placeholder_text("Please input password again.")
		.margin_top(10)
		.text("")
		.show_peek_icon(true)
		.build();

	form_vbox.append(&password_again_entry);

	/*** Buttons Box ***/
	let buttons_hbox = Box::builder().margin_top(30).homogeneous(true).build();

	/*** Submit Button ***/
	let submit_button = Button::builder()
		.label("Submit")
		.margin_start(50)
		.margin_end(50)
		.build();

	submit_button.connect_clicked(clone!(@weak application, @weak register_window, @strong username_entry, @strong password_entry, @strong password_again_entry => move |_|{
		let username = String::from(username_entry.text().as_str());
		let password = String::from(password_entry.text().as_str());
		let password_again = String::from(password_again_entry.text().as_str());

		if username == ""{
			let warning_dialog = MessageDialog::builder()
				.transient_for(&register_window)
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
				.transient_for(&register_window)
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

		if password_again == ""{
			let warning_dialog = MessageDialog::builder()
				.transient_for(&register_window)
				.modal(true)
				.buttons(ButtonsType::Ok)
				.text("Warning")
				.secondary_text("You should input the password again.")
				.build();

			warning_dialog.run_async(move |obj, _|{
				obj.close();
			});

			password_again_entry.grab_focus();

			return;
		}

		if password != password_again {
			let warning_dialog = MessageDialog::builder()
				.transient_for(&register_window)
				.modal(true)
				.buttons(ButtonsType::Ok)
				.text("Warning")
				.secondary_text("The inputed password is different for the two times.")
				.build();

			warning_dialog.run_async(move |obj, _|{
				obj.close();
			});

			password_entry.grab_focus();

			return;
		}

		if password_again.len() < 8{
			let warning_dialog = MessageDialog::builder()
				.transient_for(&register_window)
				.modal(true)
				.buttons(ButtonsType::Ok)
				.text("Warning")
				.secondary_text("The length of password should be more than 8.")
				.build();

			warning_dialog.run_async(move |obj, _|{
				obj.close();
			});

			password_entry.grab_focus();

			return;
		}

		let mut has_char = false;
		let mut has_upper = false;
		let mut has_number = false;
		for c in password.chars(){
			if r#"~!@#$%^&*()_+`-={}|[]\:";',./<>?"#.contains(c){
				has_char = true;
			}
			if "ABCDEFGHIJKLMNOPQRSTUVWXYZ".contains(c){
				has_upper = true;
			}
			if "1234567890".contains(c){
				has_number = true;
			}
		}
		if has_char == false {
			let warning_dialog = MessageDialog::builder()
				.transient_for(&register_window)
				.modal(true)
				.buttons(ButtonsType::Ok)
				.text("Warning")
				.secondary_text("The password should be contains character.")
				.build();

			warning_dialog.run_async(move |obj, _|{
				obj.close();
			});

			password_entry.grab_focus();

			return;
		}
		if has_upper == false {
			let warning_dialog = MessageDialog::builder()
				.transient_for(&register_window)
				.modal(true)
				.buttons(ButtonsType::Ok)
				.text("Warning")
				.secondary_text("The password should be contains upper-case letters.")
				.build();

			warning_dialog.run_async(move |obj, _|{
				obj.close();
			});

			password_entry.grab_focus();

			return;
		}
		if has_number == false {
			let warning_dialog = MessageDialog::builder()
				.transient_for(&register_window)
				.modal(true)
				.buttons(ButtonsType::Ok)
				.text("Warning")
				.secondary_text("The password should be contains number.")
				.build();

			warning_dialog.run_async(move |obj, _|{
				obj.close();
			});

			password_entry.grab_focus();

			return;
		}

		let response_jobject = register_user(username.as_str(), password.as_str());
		println!("{}", response_jobject);
		let response = &response_jobject["response"];
		let is_success = response["is_success"].as_bool().unwrap();
		let message = response["message"].as_str().unwrap();

		let info_dialog = MessageDialog::builder()
			.transient_for(&register_window)
			.modal(true)
			.buttons(ButtonsType::Ok)
			.text("Infomation")
			.secondary_text(message)
			.build();

		if !is_success {
			info_dialog.run_async(move |obj, _|{
				obj.close();
			});
		}
		else {
			info_dialog.run_async(move |obj, _|{
				/* Write to SQLite Begin */
				let db_file = String::from(DB_FILE);

				if !Path::new(&*db_file).exists() {
					let db_sql_file = String::from(DB_INIT_SQL_FILE);
					let text = fs::read_to_string(&*db_sql_file).unwrap();
					let connection = sqlite::open(&*db_file).unwrap();
					connection.execute(text).unwrap();
				}

				let connection = sqlite::open(&*db_file).unwrap();
				let sql = format!(
					"INSERT INTO User (ID, UserName, Password) VALUES ('{}', '{}', '{}');",
					GUID::rand().to_string(),
					username,
					password);
				connection.execute(sql).unwrap();
				/* Write to SQLite End */

				obj.close();

				register_window.close();

				let login_window = create_login_window(&application);
				application.add_window(&login_window);
				login_window.present();
			});
		}
	}));

	buttons_hbox.append(&submit_button);

	/*** Reset Button ***/
	let reset_button = Button::builder()
		.label("Reset")
		.margin_start(50)
		.margin_end(50)
		.build();

	reset_button.connect_clicked(clone!(@strong username_entry, @strong password_again_entry, @strong password_again_entry => move |_| {
		username_entry.set_text("");
		password_entry.set_text("");
		password_again_entry.set_text("");
	}));

	buttons_hbox.append(&reset_button);

	form_vbox.append(&buttons_hbox);

	main_vbox.append(&form_vbox);

	register_window.set_child(Some(&main_vbox));

	application.add_window(&register_window);

	register_window
}
