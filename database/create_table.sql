/*
 Navicat PostgreSQL Data Transfer

 Source Server         : mb
 Source Server Type    : PostgreSQL
 Source Server Version : 140007 (140007)
 Source Host           : localhost:5432
 Source Catalog        : biene
 Source Schema         : public

 Target Server Type    : PostgreSQL
 Target Server Version : 140007 (140007)
 File Encoding         : 65001

 Date: 23/03/2023 22:06:09
*/


-- ----------------------------
-- Table structure for durchsicht
-- ----------------------------
DROP TABLE IF EXISTS "durchsicht";
CREATE TABLE "durchsicht" (
  "id" int4 NOT NULL DEFAULT nextval('durchsicht_seq'::regclass),
  "datum" date,
  "volk" varchar(30) COLLATE "pg_catalog"."default",
  "koenigin" bool,
  "stifte" bool,
  "offene" bool,
  "verdeckelte" bool,
  "weiselzelle" bool,
  "spielnaepfe" bool,
  "sanftmut" int2,
  "volksstaerke" int2,
  "anz_brutwaben" int2
)
;

-- ----------------------------
-- Records of durchsicht
-- ----------------------------
BEGIN;
INSERT INTO "durchsicht" ("id", "datum", "volk", "koenigin", "stifte", "offene", "verdeckelte", "weiselzelle", "spielnaepfe", "sanftmut", "volksstaerke", "anz_brutwaben") VALUES (13, '2023-03-23', 'Volk 01', 't', 't', 't', 'f', 'f', 'f', 5, 5, 8);
COMMIT;

-- ----------------------------
-- Table structure for volk
-- ----------------------------
DROP TABLE IF EXISTS "volk";
CREATE TABLE "volk" (
  "id" int4 NOT NULL DEFAULT nextval('volk_seq'::regclass),
  "volk" varchar(30) COLLATE "pg_catalog"."default",
  "nummer" int4,
  "koenigin" varchar(30) COLLATE "pg_catalog"."default",
  "erstellt" date,
  "aufgeloest" date,
  "typ" varchar(30) COLLATE "pg_catalog"."default",
  "raehmchenmass" varchar(30) COLLATE "pg_catalog"."default",
  "stand" varchar(4) COLLATE "pg_catalog"."default"
)
;

-- ----------------------------
-- Records of volk
-- ----------------------------
BEGIN;
INSERT INTO "volk" ("id", "volk", "nummer", "koenigin", "erstellt", "aufgeloest", "typ", "raehmchenmass", "stand") VALUES (12, 'Volk 01', 1, NULL, '2017-05-21', NULL, 'Dadant', 'Dadant Blatt', 'zuHa');
COMMIT;

-- ----------------------------
-- Checks structure for table durchsicht
-- ----------------------------
ALTER TABLE "durchsicht" ADD CONSTRAINT "sanftmut_c" CHECK (sanftmut <= 5);
ALTER TABLE "durchsicht" ADD CONSTRAINT "volksstaerke_c" CHECK (volksstaerke <= 5);

-- ----------------------------
-- Primary Key structure for table durchsicht
-- ----------------------------
ALTER TABLE "durchsicht" ADD CONSTRAINT "durchsicht_p" PRIMARY KEY ("id");

-- ----------------------------
-- Indexes structure for table volk
-- ----------------------------
CREATE UNIQUE INDEX "volk_idx" ON "volk" USING btree (
  "volk" COLLATE "pg_catalog"."default" "pg_catalog"."text_ops" ASC NULLS LAST
);

-- ----------------------------
-- Primary Key structure for table volk
-- ----------------------------
ALTER TABLE "volk" ADD CONSTRAINT "volk_p" PRIMARY KEY ("id");

-- ----------------------------
-- Foreign Keys structure for table durchsicht
-- ----------------------------
ALTER TABLE "durchsicht" ADD CONSTRAINT "durchsicht_volk_f" FOREIGN KEY ("volk") REFERENCES "volk" ("volk") MATCH FULL ON DELETE NO ACTION ON UPDATE NO ACTION;

-- ----------------------------
-- Foreign Keys structure for table volk
-- ----------------------------
ALTER TABLE "volk" ADD CONSTRAINT "volk_koenigin_f" FOREIGN KEY ("koenigin") REFERENCES "koenigin" ("koenigin") MATCH FULL ON DELETE NO ACTION ON UPDATE NO ACTION;
ALTER TABLE "volk" ADD CONSTRAINT "volk_stand_f" FOREIGN KEY ("stand") REFERENCES "stand" ("kuerzel") MATCH FULL ON DELETE NO ACTION ON UPDATE NO ACTION;
