-- Create a UUID column for puzzle states

-- We compute early UUIDs by means of the md5 function, but the
-- code will do something subtly different for the inputs in
-- order that we do not conflict.

ALTER TABLE puzzle_state
    ADD COLUMN uuid VARCHAR;

UPDATE puzzle_state set uuid=md5(('migrate' || puzzle || 'entry' || id::text)::bytea);

ALTER TABLE puzzle_state
    ALTER COLUMN uuid SET NOT NULL;
