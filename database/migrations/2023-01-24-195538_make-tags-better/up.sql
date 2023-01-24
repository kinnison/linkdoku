-- Add some more tag metadata

ALTER TABLE tag
  ADD COLUMN colour VARCHAR;

ALTER TABLE tag
  ADD COLUMN black_text BOOLEAN;

ALTER TABLE tag
  ADD COLUMN description VARCHAR;

UPDATE tag set colour='#ffffff';
UPDATE tag set black_text=TRUE;
UPDATE tag set description='';


ALTER TABLE tag
  ALTER COLUMN colour SET NOT NULL;
ALTER TABLE tag
  ALTER COLUMN black_text SET NOT NULL;

ALTER TABLE tag
  ALTER COLUMN description SET NOT NULL;

INSERT INTO tag (uuid, name, colour, black_text, description) VALUES 
  (md5(('tag:variant:Classic')::bytea), 'variant:Classic', '#3e8ed0', TRUE, 'Classic Sudoku'),
  (md5(('tag:variant:Arrow')::bytea), 'variant:Arrow', '#3e8ed0', TRUE, 'Includes Arrow variant'),
  (md5(('tag:variant:Renban')::bytea), 'variant:Renban', '#3e8ed0', TRUE, 'Includes Renban variant'),
  (md5(('tag:variant:Thermo')::bytea), 'variant:Thermo', '#3e8ed0', TRUE, 'Includes Thermo variant'),
  (md5(('tag:variant:Killer')::bytea), 'variant:Killer', '#3e8ed0', TRUE, 'Includes Killer variant'),
  (md5(('tag:variant:Irregular')::bytea), 'variant:Irregular', '#3e8ed0', TRUE, 'Includes Irregular variant'),
  (md5(('tag:variant:Indexing')::bytea), 'variant:Indexing', '#3e8ed0', TRUE, 'Includes Indexing variant'),
  (md5(('tag:variant:Little Killer')::bytea), 'variant:Little Killer', '#3e8ed0', TRUE, 'Includes Little Killer variant'),
  (md5(('tag:variant:Kropki')::bytea), 'variant:Kropki', '#3e8ed0', TRUE, 'Includes Kropki variant'),
  (md5(('tag:variant:Sandwich')::bytea), 'variant:Sandwich', '#3e8ed0', TRUE, 'Includes Sandwich variant'),
  (md5(('tag:variant:Non-Consecutive')::bytea), 'variant:Non-Consecutive', '#3e8ed0', TRUE, 'Includes Non-Consecutive variant'),
  (md5(('tag:variant:Sudoku-X')::bytea), 'variant:Sudoku-X', '#3e8ed0', TRUE, 'Includes Sudoku-X variant'),
  (md5(('tag:variant:Odd-Even')::bytea), 'variant:Odd-Even', '#3e8ed0', TRUE, 'Includes Odd-Even variant'),
  (md5(('tag:variant:Quadruples')::bytea), 'variant:Quadruples', '#3e8ed0', TRUE, 'Includes Quadruples variant'),
  (md5(('tag:variant:Between Lines')::bytea), 'variant:Between Lines', '#3e8ed0', TRUE, 'Includes Between Lines variant'),
  (md5(('tag:variant:Clones')::bytea), 'variant:Clones', '#3e8ed0', TRUE, 'Includes Clones variant'),
  (md5(('tag:variant:German Whispers')::bytea), 'variant:German Whispers', '#3e8ed0', TRUE, 'Includes German Whispers variant'),
  (md5(('tag:variant:Entropic Lines')::bytea), 'variant:Entropic Lines', '#3e8ed0', TRUE, 'Includes Entropic Lines variant'),
  (md5(('tag:variant:Fog of War')::bytea), 'variant:Fog of War', '#3e8ed0', TRUE, 'Includes Fog of War variant'),
  (md5(('tag:variant:Anti-Knight')::bytea), 'variant:Anti-Knight', '#3e8ed0', TRUE, 'Includes Anti-Knight variant'),
  (md5(('tag:variant:Palindromes')::bytea), 'variant:Palindromes', '#3e8ed0', TRUE, 'Includes Palindromes variant'),
  (md5(('tag:variant:Min/Max')::bytea), 'variant:Min/Max', '#3e8ed0', TRUE, 'Includes Min/Max variant'),
  (md5(('tag:variant:Equal Sum Lines')::bytea), 'variant:Equal Sum Lines', '#3e8ed0', TRUE, 'Includes Equal Sum Lines variant'),
  (md5(('tag:variant:Ten-Lines')::bytea), 'variant:Ten-Lines', '#3e8ed0', TRUE, 'Includes Ten-Lines variant'),
  (md5(('tag:variant:Extra regions')::bytea), 'variant:Extra regions', '#3e8ed0', TRUE, 'Includes Extra regions variant'),
  (md5(('tag:variant:Windoku')::bytea), 'variant:Windoku', '#3e8ed0', TRUE, 'Includes Windoku variant'),
  (md5(('tag:variant:X-Sums')::bytea), 'variant:X-Sums', '#3e8ed0', TRUE, 'Includes X-Sums variant'),
  (md5(('tag:variant:Skyscraper')::bytea), 'variant:Skyscraper', '#3e8ed0', TRUE, 'Includes Skyscraper variant'),
  (md5(('tag:variant:Double Arrow')::bytea), 'variant:Double Arrow', '#3e8ed0', TRUE, 'Includes Double Arrow variant'),
  (md5(('tag:variant:Doublers')::bytea), 'variant:Doublers', '#3e8ed0', TRUE, 'Includes Doublers variant'),

  (md5(('tag:suitable:Tutorial')::bytea), 'suitable:Tutorial', '#ffe08a', TRUE, 'Good teaching a variant or technique'),
  (md5(('tag:suitable:Streaming')::bytea), 'suitable:Streaming', '#ffe08a', TRUE, 'Good for streaming the solve'),
  (md5(('tag:suitable:Joint Solve')::bytea), 'suitable:Joint Solve', '#ffe08a', TRUE, 'Good for joint solving')
;