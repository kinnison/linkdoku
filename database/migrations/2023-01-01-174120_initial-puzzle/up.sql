-- This migration is intended to deal with the initial puzzle tables necessary
-- We want to store puzzles, which could have many states, and we need to
-- be able to tag those puzzles for searching etc.
--
-- Initially puzzle tags will be global and controlled by the database
-- though later we may permit users to define tags as well.
-- Also for now the tags are only for puzzles, though later we might
-- add tags for roles too.

CREATE TYPE visibility AS ENUM ('restricted', 'public', 'published');

CREATE TABLE puzzle (
    uuid VARCHAR PRIMARY KEY,
    owner VARCHAR NOT NULL REFERENCES role(uuid),
    display_name VARCHAR NOT NULL,
    short_name VARCHAR NOT NULL,
    visibility visibility NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL
);

CREATE INDEX puzzle_by_owner ON puzzle(owner);

CREATE TABLE tag (
    uuid VARCHAR NOT NULL PRIMARY KEY,
    name VARCHAR NOT NULL
);

CREATE TABLE puzzle_tag (
    uuid VARCHAR NOT NULL PRIMARY KEY,
    puzzle VARCHAR NOT NULL REFERENCES puzzle(uuid),
    tag VARCHAR NOT NULL REFERENCES tag(uuid),

    CONSTRAINT puzzle_tag_unique UNIQUE (puzzle,tag)
);

CREATE INDEX puzzle_tag_by_puzzle ON puzzle_tag(puzzle);
CREATE INDEX puzzle_tag_by_tag ON puzzle_tag(tag);

CREATE TABLE puzzle_state (
    id SERIAL PRIMARY KEY,
    puzzle VARCHAR NOT NULL REFERENCES puzzle(uuid),
    description TEXT NOT NULL,
    visibility visibility NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL,
    data TEXT NOT NULL
);

CREATE INDEX puzzle_state_by_puzzle ON puzzle_state(puzzle);
CREATE INDEX puzzle_state_by_puzzle_visibility ON puzzle_state(puzzle,visibility);
