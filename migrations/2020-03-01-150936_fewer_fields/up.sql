-- Your SQL goes here

CREATE TYPE milk AS ENUM ('cow', 'goat', 'sheep', 'other');
CREATE TYPE cheese_type AS ENUM ('fresh', 'soft', 'cold-pressed', 'hot-pressed', 'hard', 'blue', 'pasta-filata', 'other');
CREATE TYPE rind AS ENUM ('velvety', 'washed', 'natural', 'na');
CREATE TYPE country AS ENUM ('france', 'england', 'italy', 'switzerland', 'wales', 'spain', 'scotland', 'ireland', 'other');


CREATE TABLE cheeses (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    photo character varying(250),
    milk milk NOT NULL,
    pasteurised boolean,
    cheesetype cheese_type NOT NULL,
    rind rind NOT NULL,
    additive character varying(50),
    region character varying(50),
    country country,
    rating integer,
    comment character varying(250),
    maturity integer
);
