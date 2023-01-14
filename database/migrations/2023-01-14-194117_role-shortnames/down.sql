-- Remove role shortname

DROP INDEX puzzle_by_owner_short_name;

ALTER TABLE puzzle DROP CONSTRAINT puzzle_short_name_unique_in_role;

DROP INDEX role_short_name;

ALTER TABLE role DROP COLUMN short_name;
