BEGIN;


CREATE TABLE IF NOT EXISTS compounds."Compound"
(
    entry character(6) COLLATE pg_catalog."default" NOT NULL,
    formula character(20) COLLATE pg_catalog."default",
    exact_mass numeric(3, 4),
    mol_weight numeric(3, 2),
    CONSTRAINT "Compound_pkey" PRIMARY KEY (entry)
);

CREATE TABLE IF NOT EXISTS compounds.names
(
    entry character(6) COLLATE pg_catalog."default",
    name character(155) COLLATE pg_catalog."default"
);

ALTER TABLE IF EXISTS compounds.names
    ADD CONSTRAINT entry_key FOREIGN KEY (entry)
    REFERENCES compounds."Compound" (entry) MATCH SIMPLE
    ON UPDATE NO ACTION
    ON DELETE NO ACTION;

END;