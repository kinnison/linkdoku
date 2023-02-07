# Puzzle Description

The puzzle description is the majority of the content offered when someone visits
your puzzle's page on Linkdoku. In here you **must** tell the reader how to access
your puzzle and ideally something about it.

Depending on the kind of puzzle you are publishing, there are some "special" pieces
of Markdown which can help you with this, and if you have not set a description
yourself, a default description will be added when you add puzzle data to your puzzle.

## Special puzzle syntax for fpuzzles data

If your puzzle is primarily represented as data in an fpuzzles format, then you can
use the following special syntax to help you to make your puzzle description more easily.

- `[rules]` on its own will expand to the rules contained within the puzzle data. Those
  rules will also be interpreted as Markdown, meaning that you can include formatting in
  those rules and expect it to be honoured here. _Note_ these special forms are not available
  to the rules themselves, so you cannot accidentall create ever-expanding rulesets.
- `![grid]` on its own will expand to a reference to the Sudokupad API to render your puzzle
  data as a grid. This can be used to give viewers a preview of your puzzle.
- Each of `[fpuzzles]`, `[sudokupad]`, and `[beta-sudokupad]` create links to your puzzle
  on the respective solving websites.

If you have suggestions for other convenient special syntax, please let us know.
