-- Make tags more boring

ALTER TABLE tag
  DROP COLUMN colour;

ALTER TABLE tag
  DROP COLUMN black_text;
