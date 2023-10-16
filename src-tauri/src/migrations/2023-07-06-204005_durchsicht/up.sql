CREATE SEQUENCE public.durchsicht_seq
	INCREMENT BY 1
	MINVALUE 0
	MAXVALUE 2147483647
	START WITH 1
	CACHE 1
	NO CYCLE
	OWNED BY NONE;



CREATE TABLE public.durchsicht (
	id integer NOT NULL DEFAULT nextval('public.durchsicht_seq'::regclass),
	datum date,
	volk character varying(30),
	koenigin boolean,
	stifte boolean,
	offene boolean,
	verdeckelte boolean,
	weiselzelle boolean,
	spielnaepfe boolean,
	sanftmut smallint,
	volksstaerke smallint,
	anz_brutwaben smallint,
	CONSTRAINT durchsicht_p PRIMARY KEY (id),
	CONSTRAINT sanftmut_c CHECK ((sanftmut <= 5)),
	CONSTRAINT volksstaerke_c CHECK ((volksstaerke <= 5))
);
