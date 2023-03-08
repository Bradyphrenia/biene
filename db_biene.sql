-- Database generated with pgModeler (PostgreSQL Database Modeler).
-- pgModeler version: 0.9.4
-- PostgreSQL version: 13.0
-- Project Site: pgmodeler.io
-- Model Author: ---
-- -- object: steffen | type: ROLE --
-- -- DROP ROLE IF EXISTS steffen;
-- CREATE ROLE steffen WITH 
-- 	SUPERUSER
-- 	CREATEDB
-- 	CREATEROLE
-- 	INHERIT
-- 	LOGIN
-- 	REPLICATION
-- 	BYPASSRLS
-- 	ENCRYPTED PASSWORD '********';
-- -- ddl-end --
-- 

-- Database creation must be performed outside a multi lined SQL file. 
-- These commands were put in this file only as a convenience.
-- 
-- object: biene | type: DATABASE --
-- DROP DATABASE IF EXISTS biene;
CREATE DATABASE biene
	ENCODING = 'UTF8'
	LC_COLLATE = 'C'
	LC_CTYPE = 'C'
	TABLESPACE = pg_default
	OWNER = steffen;
-- ddl-end --


SET check_function_bodies = false;
-- ddl-end --

-- object: public.tbl_ins_up_before | type: FUNCTION --
-- DROP FUNCTION IF EXISTS public.tbl_ins_up_before() CASCADE;
CREATE FUNCTION public.tbl_ins_up_before ()
	RETURNS trigger
	LANGUAGE plpgsql
	VOLATILE 
	CALLED ON NULL INPUT
	SECURITY INVOKER
	PARALLEL UNSAFE
	COST 100
	AS $$
 BEGIN-- Routine body goes here...
	IF
		EXISTS (
		SELECT
			1 
		FROM
			gewichte 
		WHERE
			( datum, volk ) = ( NEW.datum, NEW.volk )) THEN
			RETURN NULL;
		raise notice 'Duplikat gefunden!';
		
	END IF;
	RETURN NEW;

END 
$$;
-- ddl-end --
ALTER FUNCTION public.tbl_ins_up_before() OWNER TO postgres;
-- ddl-end --

-- object: public.tbl_ins_up_before2 | type: FUNCTION --
-- DROP FUNCTION IF EXISTS public.tbl_ins_up_before2() CASCADE;
CREATE FUNCTION public.tbl_ins_up_before2 ()
	RETURNS trigger
	LANGUAGE plpgsql
	VOLATILE 
	CALLED ON NULL INPUT
	SECURITY INVOKER
	PARALLEL UNSAFE
	COST 100
	AS $$
 BEGIN-- Routine body goes here...
	IF
		EXISTS (
		SELECT
			1 
		FROM
			stock	
		WHERE
			( datum, volk ) = ( NEW.datum, NEW.volk )) THEN
			RETURN NULL;
		raise notice 'Duplikat gefunden!';
		
	END IF;
	RETURN NEW;

END 
$$;
-- ddl-end --
ALTER FUNCTION public.tbl_ins_up_before2() OWNER TO postgres;
-- ddl-end --

-- object: public.tbl_ins_up_before3 | type: FUNCTION --
-- DROP FUNCTION IF EXISTS public.tbl_ins_up_before3() CASCADE;
CREATE FUNCTION public.tbl_ins_up_before3 ()
	RETURNS trigger
	LANGUAGE plpgsql
	VOLATILE 
	CALLED ON NULL INPUT
	SECURITY INVOKER
	PARALLEL UNSAFE
	COST 100
	AS $$
 BEGIN-- Routine body goes here...
	IF
		EXISTS (
		SELECT
			1 
		FROM
			wetter	
		WHERE
			( datum, stand ) = ( NEW.datum, NEW.stand )) THEN
			RETURN NULL;
		raise notice 'Duplikat gefunden!';
	END IF;
	RETURN NEW;

END 
$$;
-- ddl-end --
ALTER FUNCTION public.tbl_ins_up_before3() OWNER TO postgres;
-- ddl-end --

-- object: public.tbl_ins_up_before4 | type: FUNCTION --
-- DROP FUNCTION IF EXISTS public.tbl_ins_up_before4() CASCADE;
CREATE FUNCTION public.tbl_ins_up_before4 ()
	RETURNS trigger
	LANGUAGE plpgsql
	VOLATILE 
	CALLED ON NULL INPUT
	SECURITY INVOKER
	PARALLEL UNSAFE
	COST 100
	AS $$
 BEGIN-- Routine body goes here...
	IF
		EXISTS (
		SELECT
			1 
		FROM
			sensor_temp
		
		WHERE
			(datum) = ( NEW.datum )) THEN
			RETURN NULL;
		raise info 'Duplikat gefunden!';
		
	END IF;
	RETURN NEW;

END 
$$;
-- ddl-end --
ALTER FUNCTION public.tbl_ins_up_before4() OWNER TO postgres;
-- ddl-end --

-- object: public.abfuellung_seq | type: SEQUENCE --
-- DROP SEQUENCE IF EXISTS public.abfuellung_seq CASCADE;
CREATE SEQUENCE public.abfuellung_seq
	INCREMENT BY 1
	MINVALUE 0
	MAXVALUE 2147483647
	START WITH 1
	CACHE 1
	NO CYCLE
	OWNED BY NONE;

-- ddl-end --
ALTER SEQUENCE public.abfuellung_seq OWNER TO postgres;
-- ddl-end --

-- object: public.arbeiten_seq | type: SEQUENCE --
-- DROP SEQUENCE IF EXISTS public.arbeiten_seq CASCADE;
CREATE SEQUENCE public.arbeiten_seq
	INCREMENT BY 1
	MINVALUE 0
	MAXVALUE 2147483647
	START WITH 1
	CACHE 1
	NO CYCLE
	OWNED BY NONE;

-- ddl-end --
ALTER SEQUENCE public.arbeiten_seq OWNER TO postgres;
-- ddl-end --

-- object: public.bilder_seq | type: SEQUENCE --
-- DROP SEQUENCE IF EXISTS public.bilder_seq CASCADE;
CREATE SEQUENCE public.bilder_seq
	INCREMENT BY 1
	MINVALUE 0
	MAXVALUE 2147483647
	START WITH 1
	CACHE 1
	NO CYCLE
	OWNED BY NONE;

-- ddl-end --
ALTER SEQUENCE public.bilder_seq OWNER TO postgres;
-- ddl-end --

-- object: public.durchsicht_seq | type: SEQUENCE --
-- DROP SEQUENCE IF EXISTS public.durchsicht_seq CASCADE;
CREATE SEQUENCE public.durchsicht_seq
	INCREMENT BY 1
	MINVALUE 0
	MAXVALUE 2147483647
	START WITH 1
	CACHE 1
	NO CYCLE
	OWNED BY NONE;

-- ddl-end --
ALTER SEQUENCE public.durchsicht_seq OWNER TO steffen;
-- ddl-end --

-- object: public.gewichte_seq | type: SEQUENCE --
-- DROP SEQUENCE IF EXISTS public.gewichte_seq CASCADE;
CREATE SEQUENCE public.gewichte_seq
	INCREMENT BY 1
	MINVALUE 0
	MAXVALUE 2147483647
	START WITH 1
	CACHE 1
	NO CYCLE
	OWNED BY NONE;

-- ddl-end --
ALTER SEQUENCE public.gewichte_seq OWNER TO postgres;
-- ddl-end --

-- object: public.koenigin_seq | type: SEQUENCE --
-- DROP SEQUENCE IF EXISTS public.koenigin_seq CASCADE;
CREATE SEQUENCE public.koenigin_seq
	INCREMENT BY 1
	MINVALUE 0
	MAXVALUE 2147483647
	START WITH 1
	CACHE 1
	NO CYCLE
	OWNED BY NONE;

-- ddl-end --
ALTER SEQUENCE public.koenigin_seq OWNER TO postgres;
-- ddl-end --

-- object: public.landesverband_seq | type: SEQUENCE --
-- DROP SEQUENCE IF EXISTS public.landesverband_seq CASCADE;
CREATE SEQUENCE public.landesverband_seq
	INCREMENT BY 1
	MINVALUE 0
	MAXVALUE 2147483647
	START WITH 1
	CACHE 1
	NO CYCLE
	OWNED BY NONE;

-- ddl-end --
ALTER SEQUENCE public.landesverband_seq OWNER TO steffen;
-- ddl-end --

-- object: public.schleuderung_seq | type: SEQUENCE --
-- DROP SEQUENCE IF EXISTS public.schleuderung_seq CASCADE;
CREATE SEQUENCE public.schleuderung_seq
	INCREMENT BY 1
	MINVALUE 0
	MAXVALUE 2147483647
	START WITH 1
	CACHE 1
	NO CYCLE
	OWNED BY NONE;

-- ddl-end --
ALTER SEQUENCE public.schleuderung_seq OWNER TO postgres;
-- ddl-end --

-- object: public.stand_seq | type: SEQUENCE --
-- DROP SEQUENCE IF EXISTS public.stand_seq CASCADE;
CREATE SEQUENCE public.stand_seq
	INCREMENT BY 1
	MINVALUE 0
	MAXVALUE 2147483647
	START WITH 1
	CACHE 1
	NO CYCLE
	OWNED BY NONE;

-- ddl-end --
ALTER SEQUENCE public.stand_seq OWNER TO postgres;
-- ddl-end --

-- object: public.stock_seq | type: SEQUENCE --
-- DROP SEQUENCE IF EXISTS public.stock_seq CASCADE;
CREATE SEQUENCE public.stock_seq
	INCREMENT BY 1
	MINVALUE 1
	MAXVALUE 9223372036854775807
	START WITH 1
	CACHE 1
	NO CYCLE
	OWNED BY NONE;

-- ddl-end --
ALTER SEQUENCE public.stock_seq OWNER TO postgres;
-- ddl-end --

-- object: public.test | type: SEQUENCE --
-- DROP SEQUENCE IF EXISTS public.test CASCADE;
CREATE SEQUENCE public.test
	INCREMENT BY 1
	MINVALUE 1
	MAXVALUE 9223372036854775807
	START WITH 1
	CACHE 1
	NO CYCLE
	OWNED BY NONE;

-- ddl-end --
ALTER SEQUENCE public.test OWNER TO postgres;
-- ddl-end --

-- object: public.veterinaer_seq | type: SEQUENCE --
-- DROP SEQUENCE IF EXISTS public.veterinaer_seq CASCADE;
CREATE SEQUENCE public.veterinaer_seq
	INCREMENT BY 1
	MINVALUE 0
	MAXVALUE 2147483647
	START WITH 1
	CACHE 1
	NO CYCLE
	OWNED BY NONE;

-- ddl-end --
ALTER SEQUENCE public.veterinaer_seq OWNER TO steffen;
-- ddl-end --

-- object: public.volk_seq | type: SEQUENCE --
-- DROP SEQUENCE IF EXISTS public.volk_seq CASCADE;
CREATE SEQUENCE public.volk_seq
	INCREMENT BY 1
	MINVALUE 0
	MAXVALUE 2147483647
	START WITH 1
	CACHE 1
	NO CYCLE
	OWNED BY NONE;

-- ddl-end --
ALTER SEQUENCE public.volk_seq OWNER TO postgres;
-- ddl-end --

-- object: public.wetter_seq | type: SEQUENCE --
-- DROP SEQUENCE IF EXISTS public.wetter_seq CASCADE;
CREATE SEQUENCE public.wetter_seq
	INCREMENT BY 1
	MINVALUE 0
	MAXVALUE 2147483647
	START WITH 1
	CACHE 1
	NO CYCLE
	OWNED BY NONE;

-- ddl-end --
ALTER SEQUENCE public.wetter_seq OWNER TO steffen;
-- ddl-end --

-- object: public.zuchtserie_seq | type: SEQUENCE --
-- DROP SEQUENCE IF EXISTS public.zuchtserie_seq CASCADE;
CREATE SEQUENCE public.zuchtserie_seq
	INCREMENT BY 1
	MINVALUE 0
	MAXVALUE 2147483647
	START WITH 1
	CACHE 1
	NO CYCLE
	OWNED BY NONE;

-- ddl-end --
ALTER SEQUENCE public.zuchtserie_seq OWNER TO postgres;
-- ddl-end --

-- object: public.zuechter_seq | type: SEQUENCE --
-- DROP SEQUENCE IF EXISTS public.zuechter_seq CASCADE;
CREATE SEQUENCE public.zuechter_seq
	INCREMENT BY 1
	MINVALUE 0
	MAXVALUE 2147483647
	START WITH 1
	CACHE 1
	NO CYCLE
	OWNED BY NONE;

-- ddl-end --
ALTER SEQUENCE public.zuechter_seq OWNER TO postgres;
-- ddl-end --

-- object: public.abfuellung | type: TABLE --
-- DROP TABLE IF EXISTS public.abfuellung CASCADE;
CREATE TABLE public.abfuellung (
	id integer NOT NULL DEFAULT nextval('public.abfuellung_seq'::regclass),
	datum date,
	charge character(10),
	honigsorte character(30),
	honigbehaelter character(30),
	anzahl smallint,
	wassergehalt real,
	CONSTRAINT abfuellung_p PRIMARY KEY (id)
);
-- ddl-end --
ALTER TABLE public.abfuellung OWNER TO steffen;
-- ddl-end --

-- object: public.adresse | type: TABLE --
-- DROP TABLE IF EXISTS public.adresse CASCADE;
CREATE TABLE public.adresse (
	kuerzel character varying(4) NOT NULL,
	anrede character varying(30),
	titel character varying(30),
	name character varying(50),
	vorname character varying(30),
	land character varying(30),
	plz character varying(5),
	ort character varying(30),
	strasse character varying(30),
	hausnummer character varying(4),
	CONSTRAINT adresse_p PRIMARY KEY (kuerzel)
);
-- ddl-end --
ALTER TABLE public.adresse OWNER TO steffen;
-- ddl-end --

-- object: public.arbeiten | type: TABLE --
-- DROP TABLE IF EXISTS public.arbeiten CASCADE;
CREATE TABLE public.arbeiten (
	id integer NOT NULL DEFAULT nextval('public.arbeiten_seq'::regclass),
	datum date,
	stand character varying(30),
	volk character varying(30),
	arbeit text,
	CONSTRAINT arbeiten_p PRIMARY KEY (id)
);
-- ddl-end --
ALTER TABLE public.arbeiten OWNER TO steffen;
-- ddl-end --

-- object: public.behaelter | type: TABLE --
-- DROP TABLE IF EXISTS public.behaelter CASCADE;
CREATE TABLE public.behaelter (
	honigbehaelter character(30) NOT NULL,
	CONSTRAINT behaelter_p PRIMARY KEY (honigbehaelter)
);
-- ddl-end --
ALTER TABLE public.behaelter OWNER TO steffen;
-- ddl-end --

-- object: public.bilder | type: TABLE --
-- DROP TABLE IF EXISTS public.bilder CASCADE;
CREATE TABLE public.bilder (
	id integer NOT NULL DEFAULT nextval('public.bilder_seq'::regclass),
	datum date,
	stand character varying(30),
	volk character varying(30),
	bilder bytea,
	CONSTRAINT bilder_p PRIMARY KEY (id)
);
-- ddl-end --
ALTER TABLE public.bilder OWNER TO steffen;
-- ddl-end --

-- object: public.durchsicht | type: TABLE --
-- DROP TABLE IF EXISTS public.durchsicht CASCADE;
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
-- ddl-end --
ALTER TABLE public.durchsicht OWNER TO steffen;
-- ddl-end --

-- object: public.fuetterung | type: TABLE --
-- DROP TABLE IF EXISTS public.fuetterung CASCADE;
CREATE TABLE public.fuetterung (
	id integer NOT NULL DEFAULT nextval('public.durchsicht_seq'::regclass),
	futter character varying(30),
	menge smallint,
	mengeneinheit character varying(30),
	winterfutter boolean,
-- 	datum date,
-- 	volk character varying(30),
-- 	koenigin boolean,
-- 	stifte boolean,
-- 	offene boolean,
-- 	verdeckelte boolean,
-- 	weiselzelle boolean,
-- 	spielnaepfe boolean,
-- 	sanftmut smallint,
-- 	volksstaerke smallint,
-- 	anz_brutwaben smallint,
	CONSTRAINT fuetterung_pk PRIMARY KEY (id)
)
 INHERITS(public.durchsicht);
-- ddl-end --
ALTER TABLE public.fuetterung OWNER TO steffen;
-- ddl-end --

-- object: public.gewichte | type: TABLE --
-- DROP TABLE IF EXISTS public.gewichte CASCADE;
CREATE TABLE public.gewichte (
	id integer NOT NULL DEFAULT nextval('public.gewichte_seq'::regclass),
	datum timestamp(6),
	volk character varying(30),
	gewicht real,
	einheit character varying(30),
	CONSTRAINT gewichte_p PRIMARY KEY (id)
);
-- ddl-end --
COMMENT ON TABLE public.gewichte IS E'delete from gewichte where id in (select b.id from gewichte  a, gewichte  b where (a.datum = b.datum and a.volk = b.volk) and a.id < b.id);';
-- ddl-end --
ALTER TABLE public.gewichte OWNER TO steffen;
-- ddl-end --

-- object: public.honigsorte | type: TABLE --
-- DROP TABLE IF EXISTS public.honigsorte CASCADE;
CREATE TABLE public.honigsorte (
	honigsorte character(30) NOT NULL,
	CONSTRAINT honigsorte_p PRIMARY KEY (honigsorte)
);
-- ddl-end --
ALTER TABLE public.honigsorte OWNER TO steffen;
-- ddl-end --

-- object: public.koenigin | type: TABLE --
-- DROP TABLE IF EXISTS public.koenigin CASCADE;
CREATE TABLE public.koenigin (
	id integer DEFAULT nextval('public.koenigin_seq'::regclass),
	koenigin character varying(30) NOT NULL,
	generation integer,
	art_paarung character varying(30),
	rasse character varying(30),
	stamm character varying(30),
	linie character varying(30),
	jahr character varying(4),
	volk character varying(30),
	schlupftag date,
	eiablage date,
	zeichnung character varying(30),
	nummer smallint,
	bemerkungen text,
	zuechter character varying(4),
	belegstelle character varying(30),
	status character varying DEFAULT 'aktiv',
	CONSTRAINT koenigin_p PRIMARY KEY (koenigin)
);
-- ddl-end --
ALTER TABLE public.koenigin OWNER TO steffen;
-- ddl-end --

-- object: public.kontakt | type: TABLE --
-- DROP TABLE IF EXISTS public.kontakt CASCADE;
CREATE TABLE public.kontakt (
	kuerzel character varying(4) NOT NULL,
	telefon character varying(20),
	mobil character varying(20),
	email character varying(30),
	internet character varying(30),
	bemerkung text,
	CONSTRAINT kontakt_p PRIMARY KEY (kuerzel)
);
-- ddl-end --
ALTER TABLE public.kontakt OWNER TO steffen;
-- ddl-end --

-- object: public.landesverband | type: TABLE --
-- DROP TABLE IF EXISTS public.landesverband CASCADE;
CREATE TABLE public.landesverband (
	kuerzel character varying(4) NOT NULL,
	id integer DEFAULT nextval('public.landesverband_seq'::regclass),
-- 	telefon character varying(20),
-- 	mobil character varying(20),
-- 	email character varying(30),
-- 	internet character varying(30),
-- 	bemerkung text,
-- 	anrede character varying(30),
-- 	titel character varying(30),
-- 	name character varying(50),
-- 	vorname character varying(30),
-- 	land character varying(30),
-- 	plz character varying(5),
-- 	ort character varying(30),
-- 	strasse character varying(30),
-- 	hausnummer character varying(4),
	CONSTRAINT landesverband_pk PRIMARY KEY (kuerzel)
)
 INHERITS(public.kontakt,public.adresse);
-- ddl-end --
ALTER TABLE public.landesverband OWNER TO steffen;
-- ddl-end --

-- object: public.mengeneinheit | type: TABLE --
-- DROP TABLE IF EXISTS public.mengeneinheit CASCADE;
CREATE TABLE public.mengeneinheit (
	mengeneinheit character varying(30) NOT NULL,
	CONSTRAINT mengeneinheit_p PRIMARY KEY (mengeneinheit)
);
-- ddl-end --
ALTER TABLE public.mengeneinheit OWNER TO steffen;
-- ddl-end --

-- object: public.paarung | type: TABLE --
-- DROP TABLE IF EXISTS public.paarung CASCADE;
CREATE TABLE public.paarung (
	art character(30) NOT NULL,
	CONSTRAINT paarung_p PRIMARY KEY (art)
);
-- ddl-end --
ALTER TABLE public.paarung OWNER TO steffen;
-- ddl-end --

-- object: public.schleuderung | type: TABLE --
-- DROP TABLE IF EXISTS public.schleuderung CASCADE;
CREATE TABLE public.schleuderung (
	id integer NOT NULL DEFAULT nextval('public.schleuderung_seq'::regclass),
	datum date,
	stand character(30),
	honigsorte character(30),
	charge character(10),
	menge smallint,
	wassergehalt real,
	CONSTRAINT schleuderung_p PRIMARY KEY (id)
);
-- ddl-end --
ALTER TABLE public.schleuderung OWNER TO steffen;
-- ddl-end --

-- object: public.sensor_temp | type: TABLE --
-- DROP TABLE IF EXISTS public.sensor_temp CASCADE;
CREATE TABLE public.sensor_temp (
	datum timestamp(6) NOT NULL,
	temperatur real,
	CONSTRAINT sensor_temp_pkey PRIMARY KEY (datum)
);
-- ddl-end --
ALTER TABLE public.sensor_temp OWNER TO postgres;
-- ddl-end --

-- object: public.stand | type: TABLE --
-- DROP TABLE IF EXISTS public.stand CASCADE;
CREATE TABLE public.stand (
	kuerzel character varying(4) NOT NULL,
	id integer DEFAULT nextval('public.stand_seq'::regclass),
	veterinaer character varying(4),
	geokoordinaten character varying(50),
-- 	anrede character varying(30),
-- 	titel character varying(30),
-- 	name character varying(50),
-- 	vorname character varying(30),
-- 	land character varying(30),
-- 	plz character varying(5),
-- 	ort character varying(30),
-- 	strasse character varying(30),
-- 	hausnummer character varying(4),
	CONSTRAINT stand_pk PRIMARY KEY (kuerzel)
)
 INHERITS(public.adresse);
-- ddl-end --
ALTER TABLE public.stand OWNER TO steffen;
-- ddl-end --

-- object: public.stock | type: TABLE --
-- DROP TABLE IF EXISTS public.stock CASCADE;
CREATE TABLE public.stock (
	datum timestamp(6),
	luftfeuchte real,
	temperatur real,
	volk character varying(30),
	id integer NOT NULL DEFAULT nextval('public.stock_seq'::regclass),
	CONSTRAINT wetter_copy_pkey PRIMARY KEY (id)
);
-- ddl-end --
COMMENT ON TABLE public.stock IS E'delete from stock where id in (select b.id from stock  a, stock  b where (a.datum = b.datum and a.volk = b.volk) and a.id < b.id);';
-- ddl-end --
ALTER TABLE public.stock OWNER TO postgres;
-- ddl-end --

-- object: public.veterinaer | type: TABLE --
-- DROP TABLE IF EXISTS public.veterinaer CASCADE;
CREATE TABLE public.veterinaer (
	kuerzel character varying(4) NOT NULL,
	id integer DEFAULT nextval('public.veterinaer_seq'::regclass),
-- 	anrede character varying(30),
-- 	titel character varying(30),
-- 	name character varying(50),
-- 	vorname character varying(30),
-- 	land character varying(30),
-- 	plz character varying(5),
-- 	ort character varying(30),
-- 	strasse character varying(30),
-- 	hausnummer character varying(4),
-- 	telefon character varying(20),
-- 	mobil character varying(20),
-- 	email character varying(30),
-- 	internet character varying(30),
-- 	bemerkung text,
	CONSTRAINT veterinaer_pk PRIMARY KEY (kuerzel)
)
 INHERITS(public.adresse,public.kontakt);
-- ddl-end --
ALTER TABLE public.veterinaer OWNER TO steffen;
-- ddl-end --

-- object: public.volk | type: TABLE --
-- DROP TABLE IF EXISTS public.volk CASCADE;
CREATE TABLE public.volk (
	id integer NOT NULL DEFAULT nextval('public.volk_seq'::regclass),
	volk character varying(30),
	nummer integer,
	koenigin character varying(30),
	erstellt date,
	aufgeloest date,
	typ character varying(30),
	raehmchenmass character varying(30),
	stand character varying(4),
	CONSTRAINT volk_p PRIMARY KEY (id)
);
-- ddl-end --
ALTER TABLE public.volk OWNER TO steffen;
-- ddl-end --

-- object: public.wetter | type: TABLE --
-- DROP TABLE IF EXISTS public.wetter CASCADE;
CREATE TABLE public.wetter (
	datum timestamp(6),
	luftfeuchte real,
	temperatur real,
	stand character varying(4),
	id integer NOT NULL DEFAULT nextval('public.wetter_seq'::regclass),
	CONSTRAINT wetter_p PRIMARY KEY (id)
);
-- ddl-end --
COMMENT ON TABLE public.wetter IS E'Fremdschlüssel Stand fehlt noch!\n\ndelete from wetter where id in (select b.id from wetter  a, wetter  b where (a.datum = b.datum and a.stand = b.stand) and a.id < b.id);';
-- ddl-end --
ALTER TABLE public.wetter OWNER TO steffen;
-- ddl-end --

-- object: public.zuchtserie | type: TABLE --
-- DROP TABLE IF EXISTS public.zuchtserie CASCADE;
CREATE TABLE public.zuchtserie (
	id integer NOT NULL DEFAULT nextval('public.zuchtserie_seq'::regclass),
	zuchtserie character varying(30),
	zuechter character varying(4),
	zuchtstoff character varying(30),
	pflegevolk character varying(30),
	rasse character varying(30),
	datum date,
	angesetzt smallint,
	gepflegt smallint,
	geschluepft smallint,
	schlupf date,
	CONSTRAINT zuchtserie_p PRIMARY KEY (id)
);
-- ddl-end --
ALTER TABLE public.zuchtserie OWNER TO steffen;
-- ddl-end --

-- object: public.zuechter | type: TABLE --
-- DROP TABLE IF EXISTS public.zuechter CASCADE;
CREATE TABLE public.zuechter (
	kuerzel character varying(4) NOT NULL,
	id integer DEFAULT nextval('public.zuechter_seq'::regclass),
	zuechternummer character varying(10),
	landesverband character varying(4),
-- 	telefon character varying(20),
-- 	mobil character varying(20),
-- 	email character varying(30),
-- 	internet character varying(30),
-- 	bemerkung text,
-- 	anrede character varying(30),
-- 	titel character varying(30),
-- 	name character varying(50),
-- 	vorname character varying(30),
-- 	land character varying(30),
-- 	plz character varying(5),
-- 	ort character varying(30),
-- 	strasse character varying(30),
-- 	hausnummer character varying(4),
	CONSTRAINT zuechter_pk PRIMARY KEY (kuerzel)
)
 INHERITS(public.kontakt,public.adresse);
-- ddl-end --
ALTER TABLE public.zuechter OWNER TO steffen;
-- ddl-end --

-- object: abfuellung_behaelter_idx | type: INDEX --
-- DROP INDEX IF EXISTS public.abfuellung_behaelter_idx CASCADE;
CREATE INDEX abfuellung_behaelter_idx ON public.abfuellung
USING btree
(
	honigbehaelter
)
WITH (FILLFACTOR = 90);
-- ddl-end --

-- object: abfuellung_honigsorte_idx | type: INDEX --
-- DROP INDEX IF EXISTS public.abfuellung_honigsorte_idx CASCADE;
CREATE INDEX abfuellung_honigsorte_idx ON public.abfuellung
USING btree
(
	honigsorte
)
WITH (FILLFACTOR = 90);
-- ddl-end --

-- object: public.behandlung | type: TABLE --
-- DROP TABLE IF EXISTS public.behandlung CASCADE;
CREATE TABLE public.behandlung (
	id integer NOT NULL DEFAULT nextval('public.durchsicht_seq'::regclass),
	krankheit character varying(30),
	behandlung character varying(30),
	anwendungsform character varying(30),
	menge integer,
	mengeneinheit character varying(30),
	konzentration real,
	wartezeit smallint,
	zulassungsnummer integer,
	meldepflicht boolean,
-- 	datum date,
-- 	volk character varying(30),
-- 	koenigin boolean,
-- 	stifte boolean,
-- 	offene boolean,
-- 	verdeckelte boolean,
-- 	weiselzelle boolean,
-- 	spielnaepfe boolean,
-- 	sanftmut smallint,
-- 	volksstaerke smallint,
-- 	anz_brutwaben smallint,
	CONSTRAINT behandlung_pk PRIMARY KEY (id)
)
 INHERITS(public.durchsicht);
-- ddl-end --
ALTER TABLE public.behandlung OWNER TO steffen;
-- ddl-end --

-- object: public.harvests | type: TABLE --
-- DROP TABLE IF EXISTS public.harvests CASCADE;
CREATE TABLE public.harvests (
	"Harvest ID" character varying(255) NOT NULL,
	"Date" character varying(255),
	"Product" character varying(255),
	"Quantity" character varying(255),
	"Unit" character varying(255),
	"Description" character varying(255),
	"Yards" character varying(255),
	"Hives" character varying(255),
	CONSTRAINT harvests_pkey PRIMARY KEY ("Harvest ID")
);
-- ddl-end --
ALTER TABLE public.harvests OWNER TO steffen;
-- ddl-end --

-- object: public.hives | type: TABLE --
-- DROP TABLE IF EXISTS public.hives CASCADE;
CREATE TABLE public.hives (
	"Hive ID" character varying(255) NOT NULL,
	"Hive Name" character varying(255),
	"Yard Name" character varying(255),
	"Yard ID" character varying(255),
	"Date Created" character varying(255),
	"Description" character varying(255),
	"Lat" character varying(255),
	"Lng" character varying(255),
	"Strength" character varying(255),
	"Bee Source" character varying(255),
	"Frame Count" character varying(255),
	"Hive State" character varying(255),
	"Component Type" character varying(255),
	"Sun Exposure" character varying(255),
	"Components" character varying(255),
	CONSTRAINT hives_pkey PRIMARY KEY ("Hive ID")
);
-- ddl-end --
ALTER TABLE public.hives OWNER TO steffen;
-- ddl-end --

-- object: public.inspections | type: TABLE --
-- DROP TABLE IF EXISTS public.inspections CASCADE;
CREATE TABLE public.inspections (
	"Inspection ID" character varying(255) NOT NULL DEFAULT nextval('inspections_seq'::regclass),
	"Hive ID" character varying(255),
	"Hive Name" character varying(255),
	"Inspection Date" character varying(255),
	"Sighted" character varying(255),
	"Notes" character varying(255),
	"Strength" character varying(255),
	"Temper" character varying(255),
	"Weight" character varying(255),
	"Population" character varying(255),
	"Queen Cells" character varying(255),
	"Laying Pattern" character varying(255),
	"Odor" character varying(255),
	"Equipment Condition" character varying(255),
	"Hive Condition" character varying(255),
	"Foundation Type" character varying(255),
	"Diseases" character varying(255),
	"Diseases Other" character varying(255),
	"Treatments" character varying(255),
	"Treatments Other" character varying(255),
	"Pollen Level" character varying(255),
	"Honey Level" character varying(255),
	"Feedings" character varying(255),
	"Feedings Other" character varying(255),
	"Weather Conditions" character varying(255),
	"Temperature" character varying(255),
	"Humidity" character varying(255),
	"Wind Speed" character varying(255),
	"Wind Direction" character varying(255),
	"Pressure" character varying(255),
	"Pressure Direction" character varying(255),
	"Weather Description" character varying(255),
	CONSTRAINT inspections_pkey PRIMARY KEY ("Inspection ID")
);
-- ddl-end --
ALTER TABLE public.inspections OWNER TO steffen;
-- ddl-end --

-- object: public.queens | type: TABLE --
-- DROP TABLE IF EXISTS public.queens CASCADE;
CREATE TABLE public.queens (
	"Queen ID" character varying(255) NOT NULL,
	"Hive ID" character varying(255),
	"Hive Name" character varying(255),
	"Queen Name" character varying(255),
	"Description" character varying(255),
	"Breed" character varying(255),
	"Date Installed" character varying(255),
	"Accepted" character varying(255),
	"Marked" character varying(255),
	"Marked Color" character varying(255),
	"Clipped" character varying(255),
	CONSTRAINT queens_pkey PRIMARY KEY ("Queen ID")
);
-- ddl-end --
ALTER TABLE public.queens OWNER TO steffen;
-- ddl-end --

-- object: public.yards | type: TABLE --
-- DROP TABLE IF EXISTS public.yards CASCADE;
CREATE TABLE public.yards (
	"Yard ID" character varying(255) NOT NULL,
	"Yard Name" character varying(255),
	"Address" character varying(255),
	"City" character varying(255),
	"State" character varying(255),
	"Zip Code" character varying(255),
	"Country" character varying(255),
	"Time Zone" character varying(255),
	"Description" character varying(255),
	"Lat" character varying(255),
	"Lng" character varying(255),
	CONSTRAINT yards_pkey PRIMARY KEY ("Yard ID")
);
-- ddl-end --
ALTER TABLE public.yards OWNER TO steffen;
-- ddl-end --

-- object: "P_Bilder" | type: INDEX --
-- DROP INDEX IF EXISTS public."P_Bilder" CASCADE;
CREATE UNIQUE INDEX "P_Bilder" ON public.bilder
USING btree
(
	id
)
WITH (FILLFACTOR = 90);
-- ddl-end --

-- object: id | type: INDEX --
-- DROP INDEX IF EXISTS public.id CASCADE;
CREATE UNIQUE INDEX id ON public.wetter
USING btree
(
	id
)
WITH (FILLFACTOR = 90);
-- ddl-end --

-- object: stock_idx | type: INDEX --
-- DROP INDEX IF EXISTS public.stock_idx CASCADE;
CREATE INDEX stock_idx ON public.stock
USING btree
(
	datum,
	id
)
WITH (FILLFACTOR = 90);
-- ddl-end --

-- object: veterinaer_idx | type: INDEX --
-- DROP INDEX IF EXISTS public.veterinaer_idx CASCADE;
CREATE UNIQUE INDEX veterinaer_idx ON public.veterinaer
USING btree
(
	kuerzel
)
WITH (FILLFACTOR = 90);
-- ddl-end --

-- object: wetter_idx | type: INDEX --
-- DROP INDEX IF EXISTS public.wetter_idx CASCADE;
CREATE INDEX wetter_idx ON public.wetter
USING btree
(
	datum,
	id
)
WITH (FILLFACTOR = 90);
-- ddl-end --

-- object: dbl_test | type: TRIGGER --
-- DROP TRIGGER IF EXISTS dbl_test ON public.sensor_temp CASCADE;
CREATE TRIGGER dbl_test
	BEFORE INSERT OR UPDATE
	ON public.sensor_temp
	FOR EACH ROW
	EXECUTE PROCEDURE public.tbl_ins_up_before4();
-- ddl-end --

-- object: ins_up_before | type: TRIGGER --
-- DROP TRIGGER IF EXISTS ins_up_before ON public.gewichte CASCADE;
CREATE TRIGGER ins_up_before
	BEFORE INSERT OR UPDATE
	ON public.gewichte
	FOR EACH ROW
	EXECUTE PROCEDURE public.tbl_ins_up_before();
-- ddl-end --

-- object: tbl_ins | type: TRIGGER --
-- DROP TRIGGER IF EXISTS tbl_ins ON public.stock CASCADE;
CREATE TRIGGER tbl_ins
	BEFORE INSERT OR UPDATE
	ON public.stock
	FOR EACH ROW
	EXECUTE PROCEDURE public.tbl_ins_up_before2();
-- ddl-end --

-- object: tbl_ins | type: TRIGGER --
-- DROP TRIGGER IF EXISTS tbl_ins ON public.wetter CASCADE;
CREATE TRIGGER tbl_ins
	BEFORE INSERT OR UPDATE
	ON public.wetter
	FOR EACH ROW
	EXECUTE PROCEDURE public.tbl_ins_up_before3();
-- ddl-end --

-- object: public.inspections_seq | type: SEQUENCE --
-- DROP SEQUENCE IF EXISTS public.inspections_seq CASCADE;
CREATE SEQUENCE public.inspections_seq
	INCREMENT BY 1
	MINVALUE 1
	MAXVALUE 9223372036854775807
	START WITH 606000
	CACHE 1
	NO CYCLE
	OWNED BY NONE;

-- ddl-end --
ALTER SEQUENCE public.inspections_seq OWNER TO steffen;
-- ddl-end --

-- object: public.gewicht_maerz | type: VIEW --
-- DROP VIEW IF EXISTS public.gewicht_maerz CASCADE;
CREATE VIEW public.gewicht_maerz
AS 

SELECT gewichte.datum,
    gewichte.gewicht
   FROM gewichte
  WHERE (gewichte.datum >= '2020-03-10 00:00:00'::timestamp without time zone)
  ORDER BY gewichte.datum;
-- ddl-end --
ALTER VIEW public.gewicht_maerz OWNER TO postgres;
-- ddl-end --

-- object: adresse_kuerzel_idx | type: INDEX --
-- DROP INDEX IF EXISTS public.adresse_kuerzel_idx CASCADE;
CREATE UNIQUE INDEX adresse_kuerzel_idx ON public.adresse
USING btree
(
	kuerzel
)
WITH (FILLFACTOR = 90);
-- ddl-end --

-- object: abfuellung_honigbehaelter_f | type: CONSTRAINT --
-- ALTER TABLE public.abfuellung DROP CONSTRAINT IF EXISTS abfuellung_honigbehaelter_f CASCADE;
ALTER TABLE public.abfuellung ADD CONSTRAINT abfuellung_honigbehaelter_f FOREIGN KEY (honigbehaelter)
REFERENCES public.behaelter (honigbehaelter) MATCH SIMPLE
ON DELETE NO ACTION ON UPDATE NO ACTION;
-- ddl-end --

-- object: abfuellung_honigsorte_f | type: CONSTRAINT --
-- ALTER TABLE public.abfuellung DROP CONSTRAINT IF EXISTS abfuellung_honigsorte_f CASCADE;
ALTER TABLE public.abfuellung ADD CONSTRAINT abfuellung_honigsorte_f FOREIGN KEY (honigsorte)
REFERENCES public.honigsorte (honigsorte) MATCH SIMPLE
ON DELETE NO ACTION ON UPDATE NO ACTION;
-- ddl-end --

-- object: arbeiten_s | type: CONSTRAINT --
-- ALTER TABLE public.arbeiten DROP CONSTRAINT IF EXISTS arbeiten_s CASCADE;
ALTER TABLE public.arbeiten ADD CONSTRAINT arbeiten_s FOREIGN KEY (stand)
REFERENCES public.stand (kuerzel) MATCH FULL
ON DELETE NO ACTION ON UPDATE NO ACTION;
-- ddl-end --

-- object: arbeiten_volk_f | type: CONSTRAINT --
-- ALTER TABLE public.arbeiten DROP CONSTRAINT IF EXISTS arbeiten_volk_f CASCADE;
ALTER TABLE public.arbeiten ADD CONSTRAINT arbeiten_volk_f FOREIGN KEY (volk)
REFERENCES public.volk (volk) MATCH FULL
ON DELETE NO ACTION ON UPDATE NO ACTION;
-- ddl-end --

-- object: bilder_stand_f | type: CONSTRAINT --
-- ALTER TABLE public.bilder DROP CONSTRAINT IF EXISTS bilder_stand_f CASCADE;
ALTER TABLE public.bilder ADD CONSTRAINT bilder_stand_f FOREIGN KEY (stand)
REFERENCES public.stand (kuerzel) MATCH FULL
ON DELETE NO ACTION ON UPDATE NO ACTION;
-- ddl-end --

-- object: bilder_volk_f | type: CONSTRAINT --
-- ALTER TABLE public.bilder DROP CONSTRAINT IF EXISTS bilder_volk_f CASCADE;
ALTER TABLE public.bilder ADD CONSTRAINT bilder_volk_f FOREIGN KEY (volk)
REFERENCES public.volk (volk) MATCH FULL
ON DELETE NO ACTION ON UPDATE NO ACTION;
-- ddl-end --

-- object: durchsicht_volk_f | type: CONSTRAINT --
-- ALTER TABLE public.durchsicht DROP CONSTRAINT IF EXISTS durchsicht_volk_f CASCADE;
ALTER TABLE public.durchsicht ADD CONSTRAINT durchsicht_volk_f FOREIGN KEY (volk)
REFERENCES public.volk (volk) MATCH FULL
ON DELETE NO ACTION ON UPDATE NO ACTION;
-- ddl-end --

-- object: fuetterung_mengeneinheit_f | type: CONSTRAINT --
-- ALTER TABLE public.fuetterung DROP CONSTRAINT IF EXISTS fuetterung_mengeneinheit_f CASCADE;
ALTER TABLE public.fuetterung ADD CONSTRAINT fuetterung_mengeneinheit_f FOREIGN KEY (mengeneinheit)
REFERENCES public.mengeneinheit (mengeneinheit) MATCH FULL
ON DELETE NO ACTION ON UPDATE NO ACTION;
-- ddl-end --

-- object: gewichte_mengeneinheit_f | type: CONSTRAINT --
-- ALTER TABLE public.gewichte DROP CONSTRAINT IF EXISTS gewichte_mengeneinheit_f CASCADE;
ALTER TABLE public.gewichte ADD CONSTRAINT gewichte_mengeneinheit_f FOREIGN KEY (einheit)
REFERENCES public.mengeneinheit (mengeneinheit) MATCH FULL
ON DELETE NO ACTION ON UPDATE NO ACTION;
-- ddl-end --

-- object: gewicht_volk_f | type: CONSTRAINT --
-- ALTER TABLE public.gewichte DROP CONSTRAINT IF EXISTS gewicht_volk_f CASCADE;
ALTER TABLE public.gewichte ADD CONSTRAINT gewicht_volk_f FOREIGN KEY (volk)
REFERENCES public.volk (volk) MATCH FULL
ON DELETE NO ACTION ON UPDATE NO ACTION;
-- ddl-end --

-- object: koenigin_zuechter_f | type: CONSTRAINT --
-- ALTER TABLE public.koenigin DROP CONSTRAINT IF EXISTS koenigin_zuechter_f CASCADE;
ALTER TABLE public.koenigin ADD CONSTRAINT koenigin_zuechter_f FOREIGN KEY (zuechter)
REFERENCES public.zuechter (kuerzel) MATCH FULL
ON DELETE NO ACTION ON UPDATE NO ACTION;
-- ddl-end --

-- object: volk_paarung_f | type: CONSTRAINT --
-- ALTER TABLE public.koenigin DROP CONSTRAINT IF EXISTS volk_paarung_f CASCADE;
ALTER TABLE public.koenigin ADD CONSTRAINT volk_paarung_f FOREIGN KEY (art_paarung)
REFERENCES public.paarung (art) MATCH FULL
ON DELETE NO ACTION ON UPDATE NO ACTION;
-- ddl-end --

-- object: schleuderung_honigsorte_f | type: CONSTRAINT --
-- ALTER TABLE public.schleuderung DROP CONSTRAINT IF EXISTS schleuderung_honigsorte_f CASCADE;
ALTER TABLE public.schleuderung ADD CONSTRAINT schleuderung_honigsorte_f FOREIGN KEY (honigsorte)
REFERENCES public.honigsorte (honigsorte) MATCH FULL
ON DELETE NO ACTION ON UPDATE NO ACTION;
-- ddl-end --

-- object: volk_koenigin_f | type: CONSTRAINT --
-- ALTER TABLE public.volk DROP CONSTRAINT IF EXISTS volk_koenigin_f CASCADE;
ALTER TABLE public.volk ADD CONSTRAINT volk_koenigin_f FOREIGN KEY (koenigin)
REFERENCES public.koenigin (koenigin) MATCH FULL
ON DELETE NO ACTION ON UPDATE NO ACTION;
-- ddl-end --

-- object: volk_stand_f | type: CONSTRAINT --
-- ALTER TABLE public.volk DROP CONSTRAINT IF EXISTS volk_stand_f CASCADE;
ALTER TABLE public.volk ADD CONSTRAINT volk_stand_f FOREIGN KEY (stand)
REFERENCES public.stand (kuerzel) MATCH FULL
ON DELETE NO ACTION ON UPDATE NO ACTION;
-- ddl-end --

-- object: wetter_stand_f | type: CONSTRAINT --
-- ALTER TABLE public.wetter DROP CONSTRAINT IF EXISTS wetter_stand_f CASCADE;
ALTER TABLE public.wetter ADD CONSTRAINT wetter_stand_f FOREIGN KEY (stand)
REFERENCES public.stand (kuerzel) MATCH FULL
ON DELETE NO ACTION ON UPDATE NO ACTION;
-- ddl-end --

-- object: zuchtserie_volk_f | type: CONSTRAINT --
-- ALTER TABLE public.zuchtserie DROP CONSTRAINT IF EXISTS zuchtserie_volk_f CASCADE;
ALTER TABLE public.zuchtserie ADD CONSTRAINT zuchtserie_volk_f FOREIGN KEY (pflegevolk)
REFERENCES public.volk (volk) MATCH FULL
ON DELETE NO ACTION ON UPDATE NO ACTION;
-- ddl-end --

-- object: zuechter_landesverband_f | type: CONSTRAINT --
-- ALTER TABLE public.zuechter DROP CONSTRAINT IF EXISTS zuechter_landesverband_f CASCADE;
ALTER TABLE public.zuechter ADD CONSTRAINT zuechter_landesverband_f FOREIGN KEY (landesverband)
REFERENCES public.landesverband (kuerzel) MATCH FULL
ON DELETE NO ACTION ON UPDATE NO ACTION;
-- ddl-end --

-- object: behandlung_mengeneinheit_f | type: CONSTRAINT --
-- ALTER TABLE public.behandlung DROP CONSTRAINT IF EXISTS behandlung_mengeneinheit_f CASCADE;
ALTER TABLE public.behandlung ADD CONSTRAINT behandlung_mengeneinheit_f FOREIGN KEY (mengeneinheit)
REFERENCES public.mengeneinheit (mengeneinheit) MATCH FULL
ON DELETE NO ACTION ON UPDATE NO ACTION;
-- ddl-end --

-- object: "grant_CU_c90a7737f8" | type: PERMISSION --
GRANT CREATE,USAGE
   ON SCHEMA public
   TO steffen;
-- ddl-end --

-- object: "grant_CU_cd8e46e7b6" | type: PERMISSION --
GRANT CREATE,USAGE
   ON SCHEMA public
   TO PUBLIC;
-- ddl-end --


