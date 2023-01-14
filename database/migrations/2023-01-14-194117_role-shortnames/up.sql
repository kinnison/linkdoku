-- Add short-names to roles

ALTER TABLE role
    ADD COLUMN short_name VARCHAR;

UPDATE role SET short_name=uuid;

ALTER TABLE role
    ALTER COLUMN short_name SET NOT NULL;

ALTER TABLE role
    ADD CONSTRAINT role_short_name_unique UNIQUE (short_name);

CREATE INDEX role_short_name ON role(short_name);

ALTER TABLE puzzle
    ADD CONSTRAINT puzzle_short_name_unique_in_role UNIQUE (owner, short_name);

CREATE INDEX puzzle_by_owner_short_name ON puzzle(owner, short_name);
