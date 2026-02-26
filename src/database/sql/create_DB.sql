PRAGMA foreign_keys = ON;

BEGIN;

CREATE TABLE IF NOT EXISTS compound (
    entry TEXT PRIMARY KEY NOT NULL,
    formula TEXT,
    exact_mass REAL,
    mol_weight REAL
);

CREATE TABLE IF NOT EXISTS compound_names (
    entry TEXT,
    name TEXT,
    FOREIGN KEY (entry) REFERENCES compound(entry) ON DELETE NO ACTION ON UPDATE NO ACTION
);

CREATE TABLE IF NOT EXISTS enzyme (
    entry TEXT PRIMARY KEY NOT NULL,
    sysname TEXT,
    reaction_iubmb TEXT
);

CREATE TABLE IF NOT EXISTS enzyme_names (
    entry TEXT,
    name TEXT,
    FOREIGN KEY (entry) REFERENCES enzyme(entry) ON DELETE NO ACTION ON UPDATE NO ACTION
);

CREATE TABLE IF NOT EXISTS reaction (
    entry TEXT PRIMARY KEY NOT NULL,
    name TEXT,
    definition TEXT
);

CREATE TABLE IF NOT EXISTS equation_left (
    react_entry TEXT,
    comp_entry TEXT,
    FOREIGN KEY (react_entry) REFERENCES reaction(entry) ON DELETE NO ACTION ON UPDATE NO ACTION,
    FOREIGN KEY (comp_entry) REFERENCES compound(entry) ON DELETE NO ACTION ON UPDATE NO ACTION
);

CREATE TABLE IF NOT EXISTS equation_right (
    react_entry TEXT,
    comp_entry TEXT,
    FOREIGN KEY (react_entry) REFERENCES reaction(entry) ON DELETE NO ACTION ON UPDATE NO ACTION,
    FOREIGN KEY (comp_entry) REFERENCES compound(entry) ON DELETE NO ACTION ON UPDATE NO ACTION
);

CREATE TABLE IF NOT EXISTS product (
    comp_entry TEXT,
    enzyme_entry TEXT,
    FOREIGN KEY (comp_entry) REFERENCES compound(entry) ON DELETE NO ACTION ON UPDATE NO ACTION,
    FOREIGN KEY (enzyme_entry) REFERENCES enzyme(entry) ON DELETE NO ACTION ON UPDATE NO ACTION
);

CREATE TABLE IF NOT EXISTS substrate (
    comp_entry TEXT,
    enzyme_entry TEXT,
    FOREIGN KEY (comp_entry) REFERENCES compound(entry) ON DELETE NO ACTION ON UPDATE NO ACTION,
    FOREIGN KEY (enzyme_entry) REFERENCES enzyme(entry) ON DELETE NO ACTION ON UPDATE NO ACTION
);

CREATE TABLE IF NOT EXISTS reaction_enzyme (
    react_entry TEXT,
    enzyme_entry TEXT,
    FOREIGN KEY (react_entry) REFERENCES reaction(entry) ON DELETE NO ACTION ON UPDATE NO ACTION,
    FOREIGN KEY (enzyme_entry) REFERENCES enzyme(entry) ON DELETE NO ACTION ON UPDATE NO ACTION
);

COMMIT;