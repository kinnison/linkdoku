-- Make tags more boring

ALTER TABLE tag
  DROP COLUMN colour;

ALTER TABLE tag
  DROP COLUMN black_text;

ALTER TABLE tag
  DROP COLUMN description;

DELETE FROM tag where uuid in (
  md5(('tag:variant:Classic')::bytea),
  md5(('tag:variant:Arrow')::bytea),
  md5(('tag:variant:Renban')::bytea),
  md5(('tag:variant:Thermo')::bytea),
  md5(('tag:variant:Killer')::bytea),
  md5(('tag:variant:Irregular')::bytea),
  md5(('tag:variant:Indexing')::bytea),
  md5(('tag:variant:Little Killer')::bytea),
  md5(('tag:variant:Kropki')::bytea),
  md5(('tag:variant:Sandwich')::bytea),
  md5(('tag:variant:Non-Consecutive')::bytea),
  md5(('tag:variant:Sudoku-X')::bytea),
  md5(('tag:variant:Odd-Even')::bytea),
  md5(('tag:variant:Quadruples')::bytea),
  md5(('tag:variant:Between Lines')::bytea),
  md5(('tag:variant:Clones')::bytea),
  md5(('tag:variant:German Whispers')::bytea),
  md5(('tag:variant:Entropic Lines')::bytea),
  md5(('tag:variant:Fog of War')::bytea),
  md5(('tag:variant:Anti-Knight')::bytea),
  md5(('tag:variant:Palindromes')::bytea),
  md5(('tag:variant:Min/Max')::bytea),
  md5(('tag:variant:Equal Sum Lines')::bytea),
  md5(('tag:variant:Ten-Lines')::bytea),
  md5(('tag:variant:Extra regions')::bytea),
  md5(('tag:variant:Windoku')::bytea),
  md5(('tag:variant:X-Sums')::bytea),
  md5(('tag:variant:Skyscraper')::bytea),
  md5(('tag:variant:Double Arrow')::bytea),
  md5(('tag:variant:Doublers')::bytea),


  md5(('tag:suitable:Tutorial')::bytea),
  md5(('tag:suitable:Streaming')::bytea),
  md5(('tag:suitable:Joint Solve')::bytea)
);