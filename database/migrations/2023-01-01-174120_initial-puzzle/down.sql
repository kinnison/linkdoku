-- Undo the creation of puzzles

DROP INDEX puzzle_state_by_puzzle_visibility;
DROP INDEX puzzle_state_by_puzzle;

DROP TABLE puzzle_state;

DROP INDEX puzzle_tag_by_tag;
DROP INDEX puzzle_tag_by_puzzle;

DROP TABLE puzzle_tag;

DROP TABLE tag;

DROP INDEX puzzle_by_owner;

DROP TABLE puzzle;

DROP TYPE visibility;
