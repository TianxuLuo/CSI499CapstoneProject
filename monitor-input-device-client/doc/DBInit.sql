CREATE TABLE "User" (
	"ID"	TEXT,
	"UserName"	TEXT NOT NULL UNIQUE,
	"Password"	TEXT NOT NULL,
	PRIMARY KEY("ID")
);

CREATE TABLE "KeyPressingEvent" (
	"ID"	TEXT,
	"UserID"	TEXT NOT NULL,
	"Time"	TEXT NOT NULL,
	"KeyName"	TEXT NOT NULL,
	"Status"	NUMERIC NOT NULL,
	"IsSynchro"	NUMERIC NOT NULL DEFAULT '=0',
	FOREIGN KEY("UserID") REFERENCES "User"("ID"),
	PRIMARY KEY("ID")
);

CREATE TABLE "MouseMovingEvent" (
	"ID"	TEXT,
	"UserID"	TEXT NOT NULL,
	"Time"	TEXT NOT NULL,
	"OriginalPosition"	TEXT NOT NULL,
	"NewPosition"	TEXT NOT NULL,
	"IsSynchro"	INTEGER NOT NULL DEFAULT '=0',
	PRIMARY KEY("ID")
);