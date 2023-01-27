-- Remove the extra tags

DELETE FROM tag where uuid in (
  md5(('tag:variant:Japanese sums')::bytea),
  md5(('tag:variant:XV')::bytea),
  md5(('tag:variant:Anti-King')::bytea),
  md5(('tag:variant:Yin-Yang')::bytea),
  md5(('tag:variant:Cave')::bytea),
  md5(('tag:variant:Dutch Whispers')::bytea),
  md5(('tag:variant:Chinese Whispers')::bytea),
  md5(('tag:variant:Look and Say')::bytea),
  md5(('tag:variant:Knightmare')::bytea),
  md5(('tag:variant:Global Entropy')::bytea),
  md5(('tag:variant:Novel')::bytea),
  md5(('tag:suitable:Speed Solving')::bytea), 
  md5(('tag:suitable:Pencil Newbies')::bytea),
  md5(('tag:suitable:Sudoku Newbies')::bytea)
);