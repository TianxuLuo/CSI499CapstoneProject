include!("login_window.rs");
include!("call_remote_server.rs");

extern crate guid_create;

use chrono::prelude::*;
use device_query::{DeviceEvents, DeviceState, Keycode};
use glib::clone;
use gtk4::cairo::Context;
use gtk4::prelude::*;
use gtk4::Application;
use gtk4::{Align, ButtonsType, Orientation};
use gtk4::{
	ApplicationWindow, Box, Button, DrawingArea, Entry, Label, ListBox, MessageDialog, Notebook,
	PasswordEntry, PositionType, ScrolledWindow,
};
use guid_create::GUID;
use pango::{AttrFontDesc, AttrList, FontDescription};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

//const DB_INIT_SQL_FILE: &str = "/tmp/DBInit.sql";
//const DB_FILE: &str = "/tmp/monitor_input_device.db";
 const DB_INIT_SQL_FILE: &str = "/home/lzm/桌面/DBInit.sql";
 const DB_FILE: &str = "/home/lzm/桌面/monitor_input_device.db";

static mut TOKEN_ID: Option<String> = None;
static mut USER_ID: Option<String> = None;
static mut PASSWORD: Option<String> = None;

static mut NET_IS_ENABLED: bool = false;

fn main() {
	init_db();

	let application = Application::new(Some("org.albany.project"), Default::default());

	application.connect_activate(build_ui);
	application.run();
}

fn build_ui(application: &Application) {
	let login_window = create_login_window(application);
	application.add_window(&login_window);
	login_window.present();
}

fn init_db() {
	let db_file = String::from(DB_FILE);

	if !Path::new(&*db_file).exists() {
		let db_sql_file = String::from(DB_INIT_SQL_FILE);
		let text = fs::read_to_string(&*db_sql_file).unwrap();
		let connection = sqlite::open(&*db_file).unwrap();
		connection.execute(text).unwrap();
	}
}
