# Linkdoku backend

The backend is the application which, ultimately serves linkdoku.
It provides a server which expects to answer to all of:

- `/api/` -> internal API routes
- `/-/{puzzlename}[-{view}]` -> special short-form for a puzzle URL, only works for unique puzzle shortcodes
  if the view is given, it must be a supported one and will result in redirection for that view kind
  instead of to linkdoku itself.
- Anything else is provided to the frontend, though some sub-URIs can gain some
  server-side data too
