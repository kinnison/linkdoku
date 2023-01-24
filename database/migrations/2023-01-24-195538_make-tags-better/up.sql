-- Add some more tag metadata

ALTER TABLE tag
  ADD COLUMN colour VARCHAR;

ALTER TABLE tag
  ADD COLUMN black_text BOOLEAN;

UPDATE tag set colour='#ffffff';
UPDATE tag set black_text=TRUE;


ALTER TABLE tag
  ALTER COLUMN colour SET NOT NULL;
ALTER TABLE tag
  ALTER COLUMN black_text SET NOT NULL;

INSERT INTO tag (uuid, name, colour, black_text) VALUES 
  (md5(('tag:variant:Classic')::bytea), 'variant:Classic', '#3e8ed0', TRUE),
  (md5(('tag:variant:Arrow')::bytea), 'variant:Arrow', '#3e8ed0', TRUE),
  (md5(('tag:variant:Renban')::bytea), 'variant:Renban', '#3e8ed0', TRUE),
  (md5(('tag:variant:Thermo')::bytea), 'variant:Thermo', '#3e8ed0', TRUE),

  (md5(('tag:suitable:Tutorial')::bytea), 'suitable:Tutorial', '#ffe08a', TRUE),
  (md5(('tag:suitable:Streaming')::bytea), 'suitable:Streaming', '#ffe08a', TRUE),
  (md5(('tag:suitable:Joint Solve')::bytea), 'suitable:Joint Solve', '#ffe08a', TRUE)
;