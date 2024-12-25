DROP TABLE IF EXISTS session;

DROP TABLE IF EXISTS classes;

DROP TABLE IF EXISTS student;

DROP TABLE IF EXISTS classes_student;

CREATE TABLE session (
    id INTEGER NOT NULL PRIMARY KEY,
    year INT NOT NULL,
    session_name TEXT NOT NULL
);

CREATE TABLE classes (
    id INTEGER NOT NULL PRIMARY KEY,
    name VARCHAR NOT NULL,
    group_id VARCHAR NOT NULL,
    code VARCHAR NOT NULL,
    session_id INTEGER NOT NULL,
    FOREIGN KEY (session_id) REFERENCES session (id)
);

CREATE TABLE student (
    id INTEGER NOT NULL PRIMARY KEY,
    first_name VARCHAR NOT NULL,
    last_name VARCHAR NOT NULL,
    email VARCHAR NOT NULL,
    code VARCHAR NOT NULL
);

CREATE TABLE classes_student (
    id INTEGER NOT NULL PRIMARY KEY,
    class_id INTEGER NOT NULL,
    student_id INTEGER NOT NULL,
    FOREIGN KEY (class_id) REFERENCES classes (id),
    FOREIGN KEY (student_id) REFERENCES student (id)
);
