# Brief description
This system includes two parts, a server and a client, which can run on the same computer running the Linux operating system, or on two computers running the Linux operating system respectively.The server is monitor-input-device-server，The client is monitor-input-device-client.

# Deployment environment
Both the server and the client, the system is based on the Linux operating system and has been tested on the Ubuntu 21.10 release version.

# Preparation before compilation

## Install the build environment
This system is developed using Rust and needs to be compiled with a Rust compiler. You can visit the webpage [Install Rust](https://www.rust-lang.org/tools/install) for installation help. You can also execute the following commands directly in the terminal:
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Install the dependency library GTK
The GUI part of this system is based on GTK, so the dependency library gtk should be installed. You can visit the webpage [Installing GTK](https://www.gtk.org/docs/installations/linux/) for installation help. If the operating system is the Ubuntu Linux distribution, you can also run the following commands in the terminal:
```
sudo apt install libgtk-4-dev
```

## Server installation of MySQL database
The server database of this system adopts MySQL, you can visit the webpage [Install MySQL](https://www.mysql.com/downloads/) for installation help. If the operating system is the Ubuntu Linux distribution, you can also run the following commands in the terminal:
```
sudo apt install mysql-server
```
At the same time, in order to facilitate the management of the database, you can install MySQL Workbench:
```
sudo apt install mysql-workbench
```

## Client install SQLite database
To facilitate testing or debugging, a SQLite database can be installed on the client:
```
sudo apt install sqlite
```
Similarly, to facilitate database management, you can install DB Browser for SQLite:
```
sudo apt install sqlitebrowser
```

## Deploy the database

### Deploy the client SQLite database
Copy monitor-input-device-client/doc/DBInit.sql to the /tmp directory. The client program automatically creates the database when it is first started.

The default target location is /tmp, which can be changed on line 26 of monitor-input-device-client/src/main.rs.

### Deploy the server MySQL database
1. Start MySQL Workbench.
2. Create a user `hy` and set the password to `huo04ying11xia`. Can be changed in monitor-input-device-server/src/main.rs line 4 and line 5.
2. Open the monitor-input-device-client/doc/MonitorInputDevice_mysql.sql script file in MySQL Workbench.
3. Execute the opened library building script and make sure that the library is built successfully.

# Compile
## Compile server
```
cd monitor-input-device-server
cargo build
```

## Compile client
```
cd monitor-input-device-client
cargo build
```

# Run
## RUn server first.
```
cd monitor-input-device-server
cargo run
```

## RUn client
```
cd monitor-input-device-client
cargo run
```
