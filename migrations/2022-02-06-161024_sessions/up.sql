-- Your SQL goes here
CREATE TABLE "sessions" (
                            "session_id"	TEXT NOT NULL,
                            "expire_at"	TEXT NOT NULL,
                            "user_id"	INTEGER NOT NULL,
                            PRIMARY KEY("session_id")
);