-- Your SQL goes here

ALTER TABLE "organizations" ADD CONSTRAINT "organizations_name_unique" UNIQUE("name");
