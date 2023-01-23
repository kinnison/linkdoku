-- Remove the UUID field from puzzle states

ALTER TABLE puzzle_state
    DROP COLUMN uuid;
