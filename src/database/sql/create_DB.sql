BEGIN;


CREATE TABLE IF NOT EXISTS kegg.compound
(
    entry character(6) COLLATE pg_catalog."default" NOT NULL,
    formula character(20) COLLATE pg_catalog."default",
    exact_mass numeric(3, 4),
    mol_weight numeric(3, 2),
    CONSTRAINT "Compound_pkey" PRIMARY KEY (entry)
);

CREATE TABLE IF NOT EXISTS kegg.compound_names
(
    entry character(6) COLLATE pg_catalog."default",
    name character(155) COLLATE pg_catalog."default"
);

CREATE TABLE IF NOT EXISTS kegg.enzyme
(
    entry character(15) COLLATE pg_catalog."default" NOT NULL,
    sysname character(100) COLLATE pg_catalog."default",
    "reaction(IUBMB)" character(200) COLLATE pg_catalog."default",
    CONSTRAINT enzyme_pkey PRIMARY KEY (entry)
);

CREATE TABLE IF NOT EXISTS kegg.enzyme_names
(
    entry character(15) COLLATE pg_catalog."default",
    name character(100) COLLATE pg_catalog."default"
);

CREATE TABLE IF NOT EXISTS kegg.equation_left
(
    react_entry character(6) COLLATE pg_catalog."default",
    comp_entry character(6) COLLATE pg_catalog."default"
);

CREATE TABLE IF NOT EXISTS kegg.equation_right
(
    react_entry character(6) COLLATE pg_catalog."default",
    comp_entry character(6) COLLATE pg_catalog."default"
);

CREATE TABLE IF NOT EXISTS kegg.product
(
    comp_entry character(6) COLLATE pg_catalog."default",
    enzyme_entry character(15) COLLATE pg_catalog."default"
);

CREATE TABLE IF NOT EXISTS kegg.reaction
(
    entry character(6) COLLATE pg_catalog."default" NOT NULL,
    name character(155) COLLATE pg_catalog."default",
    definition character(255) COLLATE pg_catalog."default",
    CONSTRAINT reaction_pkey PRIMARY KEY (entry)
);

CREATE TABLE IF NOT EXISTS kegg.reaction_enzyme
(
    react_entry character(6) COLLATE pg_catalog."default",
    enzyme_entry character(15) COLLATE pg_catalog."default"
);

CREATE TABLE IF NOT EXISTS kegg.substrate
(
    comp_entry character(6) COLLATE pg_catalog."default",
    enzyme_entry character(15) COLLATE pg_catalog."default"
);

ALTER TABLE IF EXISTS kegg.compound_names
    ADD CONSTRAINT comp_entry_key FOREIGN KEY (entry)
    REFERENCES kegg.compound (entry) MATCH SIMPLE
    ON UPDATE NO ACTION
    ON DELETE NO ACTION;


ALTER TABLE IF EXISTS kegg.enzyme_names
    ADD CONSTRAINT enzyme_entry_key FOREIGN KEY (entry)
    REFERENCES kegg.enzyme (entry) MATCH SIMPLE
    ON UPDATE NO ACTION
    ON DELETE NO ACTION;


ALTER TABLE IF EXISTS kegg.equation_left
    ADD CONSTRAINT comp_foreign_key_l FOREIGN KEY (comp_entry)
    REFERENCES kegg.compound (entry) MATCH SIMPLE
    ON UPDATE NO ACTION
    ON DELETE NO ACTION;


ALTER TABLE IF EXISTS kegg.equation_left
    ADD CONSTRAINT react_foreign_key_l FOREIGN KEY (react_entry)
    REFERENCES kegg.reaction (entry) MATCH SIMPLE
    ON UPDATE NO ACTION
    ON DELETE NO ACTION;


ALTER TABLE IF EXISTS kegg.equation_right
    ADD CONSTRAINT comp_foreign_key_r FOREIGN KEY (comp_entry)
    REFERENCES kegg.compound (entry) MATCH SIMPLE
    ON UPDATE NO ACTION
    ON DELETE NO ACTION;


ALTER TABLE IF EXISTS kegg.equation_right
    ADD CONSTRAINT react_foreign_key_r FOREIGN KEY (react_entry)
    REFERENCES kegg.reaction (entry) MATCH SIMPLE
    ON UPDATE NO ACTION
    ON DELETE NO ACTION;


ALTER TABLE IF EXISTS kegg.product
    ADD CONSTRAINT comp_fk FOREIGN KEY (comp_entry)
    REFERENCES kegg.compound (entry) MATCH SIMPLE
    ON UPDATE NO ACTION
    ON DELETE NO ACTION;


ALTER TABLE IF EXISTS kegg.product
    ADD CONSTRAINT enzyme_fk FOREIGN KEY (enzyme_entry)
    REFERENCES kegg.enzyme (entry) MATCH SIMPLE
    ON UPDATE NO ACTION
    ON DELETE NO ACTION;


ALTER TABLE IF EXISTS kegg.reaction_enzyme
    ADD CONSTRAINT enzyme_foreign_key FOREIGN KEY (enzyme_entry)
    REFERENCES kegg.enzyme (entry) MATCH SIMPLE
    ON UPDATE NO ACTION
    ON DELETE NO ACTION;


ALTER TABLE IF EXISTS kegg.reaction_enzyme
    ADD CONSTRAINT react_foreign_key FOREIGN KEY (react_entry)
    REFERENCES kegg.reaction (entry) MATCH SIMPLE
    ON UPDATE NO ACTION
    ON DELETE NO ACTION;


ALTER TABLE IF EXISTS kegg.substrate
    ADD CONSTRAINT comp_subst_fk FOREIGN KEY (comp_entry)
    REFERENCES kegg.compound (entry) MATCH SIMPLE
    ON UPDATE NO ACTION
    ON DELETE NO ACTION;


ALTER TABLE IF EXISTS kegg.substrate
    ADD CONSTRAINT enzyme_subst_fk FOREIGN KEY (enzyme_entry)
    REFERENCES kegg.enzyme (entry) MATCH SIMPLE
    ON UPDATE NO ACTION
    ON DELETE NO ACTION;

END;