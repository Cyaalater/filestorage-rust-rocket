-- Your SQL goes here
CREATE TABLE "users" (
                         "id"	INTEGER NOT NULL,
                         "username"	TEXT NOT NULL,
                         "hashed_password"	TEXT NOT NULL,
                         "permissions"	INTEGER NOT NULL,
                         PRIMARY KEY("id" AUTOINCREMENT)
);