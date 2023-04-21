CREATE SEQUENCE test_seq
    INCREMENT BY 1
    MINVALUE 0
    MAXVALUE 2147483647;

CREATE TABLE test_schema.test_table(
    id INTEGER NOT NULL default nextval('test_seq'),
    name varchar(50),
    value integer,
    description varchar(300)
);

ALTER SEQUENCE test_seq OWNER TO postgres;
