-- Your SQL goes here
CREATE TABLE "files" (
                         "id"	INTEGER NOT NULL,
                         "name"	TEXT NOT NULL,
                         "description"	TEXT NOT NULL,
                         "path"	TEXT NOT NULL,
                         "uploader"	TEXT NOT NULL,
                         "date"	TEXT NOT NULL,
                         PRIMARY KEY("id" AUTOINCREMENT)
);