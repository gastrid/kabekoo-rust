-- Your SQL goes here

CREATE TYPE milk AS ENUM ('cow', 'goat', 'sheep', 'other');
CREATE TYPE cheese_type AS ENUM ('fresh', 'soft', 'cold-pressed', 'hot-pressed', 'hard', 'blue', 'pasta-filata', 'other');
CREATE TYPE rind AS ENUM ('velvety', 'washed', 'natural', 'NA');
CREATE TYPE country AS ENUM ('France', 'England', 'Italy', 'Switzerland', 'Wales', 'Spain', 'Scotland', 'Ireland', 'Other');


CREATE TABLE cheeses (
    id SERIAL PRIMARY KEY,
    name character varying(50) NOT NULL,
    photo character varying(250),
    milk milk,
    pasteurised boolean,
    cheesetype cheese_type,
    rind rind,
    additive character varying(50),
    region character varying(50),
    country country,
    rating integer,
    comment character varying(250),
    maturity integer
);
