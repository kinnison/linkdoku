-- Here's a bunch more tags

INSERT INTO tag (uuid, name, colour, black_text, description) VALUES 
  (md5(('tag:variant:Japanese sums')::bytea), 'variant:Japanese sums', '#3e8ed0', TRUE, 'Includes Japanese sums variant'),
  (md5(('tag:variant:XV')::bytea), 'variant:XV', '#3e8ed0', TRUE, 'Includes XV variant'),
  (md5(('tag:variant:Anti-King')::bytea), 'variant:Anti-King', '#3e8ed0', TRUE, 'Includes Anti-King variant'),
  (md5(('tag:variant:Yin-Yang')::bytea), 'variant:Yin-Yang', '#3e8ed0', TRUE, 'Includes Yin-Yang variant'),
  (md5(('tag:variant:Cave')::bytea), 'variant:Cave', '#3e8ed0', TRUE, 'Includes Cave variant'),
  (md5(('tag:variant:Dutch Whispers')::bytea), 'variant:Dutch Whispers', '#3e8ed0', TRUE, 'Includes Dutch Whispers variant'),
  (md5(('tag:variant:Chinese Whispers')::bytea), 'variant:Chinese Whispers', '#3e8ed0', TRUE, 'Includes Chinese Whispers variant'),
  (md5(('tag:variant:Look and Say')::bytea), 'variant:Look and Say', '#3e8ed0', TRUE, 'Includes Look and Say variant'),
  (md5(('tag:variant:Knightmare')::bytea), 'variant:Knightmare', '#3e8ed0', TRUE, 'Includes Knightmare variant'),
  (md5(('tag:variant:Global Entropy')::bytea), 'variant:Global Entropy', '#3e8ed0', TRUE, 'Includes Global Entropy variant'),
  (md5(('tag:variant:Novel')::bytea), 'variant:Novel', '#3e8ed0', TRUE, 'Includes Novel variant'),
  (md5(('tag:suitable:Speed Solving')::bytea), 'suitable:Speed Solving', '#ffe08a', TRUE, 'Good for speed solving'),
  (md5(('tag:suitable:Pencil Newbies')::bytea), 'suitable:Pencil Newbies', '#ffe08a', TRUE, 'Good for pencil puzzle newbies'),
  (md5(('tag:suitable:Sudoku Newbies')::bytea), 'suitable:Sudoku Newbies', '#ffe08a', TRUE, 'Good for sudoku newbies')
;
