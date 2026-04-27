# ChatFS Protocol Specification

- This server provides file system access for a connected client workspace.

- A workspace is identified by a unique URL:
  - Base URL: https://servername.com/
  - Client ID: <client_id>
  - Workspace URL: https://servername.com/<client_id>/

- All commands must be appended to the workspace URL.
  - Example: https://servername.com/<client_id>/list?path=&recursive=false&item_type=file


--------------------------------------------------
COMMAND: list
--------------------------------------------------

URL:
  /list?path=<string>&recursive=<true|false>&item_type=<folder|file|all>

PARAMS:
  path:
    - default: ""

  recursive:
    - default: false

  item_type:
    - default: all


--------------------------------------------------
COMMAND: content
--------------------------------------------------

URL:
  /content?path=<string>&lines=<start-end>

PARAMS:
  path:
    - required
    - must be a file

  lines:
    - default: 1-*
    - "*" means all
    - examples: 2-2, 2-5, 1-*, *-10, *-*


--------------------------------------------------
COMMAND: create
--------------------------------------------------

URL:
  /create?path=<string>&item_type=<folder|file>

PARAMS:
  path:
    - required

  item_type:
    - required


--------------------------------------------------
COMMAND: copy
--------------------------------------------------

URL:
  /copy?path=<string>&dest_path=<string>

PARAMS:
  path:
    - required

  dest_path:
    - required
    - must be same type as source (file -> file, folder -> folder)


--------------------------------------------------
COMMAND: move
--------------------------------------------------

URL:
  /move?path=<string>&dest_path=<string>

PARAMS:
  Same as copy


--------------------------------------------------
COMMAND: delete
--------------------------------------------------

URL:
  /delete?path=<string>

PARAMS:
  path:
    - required


--------------------------------------------------
COMMAND: write
--------------------------------------------------

URL:
  /write?path=<string>&lines=<start-end>&mode=<shift|replace>&content=<string>

PARAMS:
  path:
    - required
    - must be a file

  lines: start-end
    - default: 1-*
    - same format as content command

  mode:
    - default: shift

  content:
    - must be URL-encoded

NOTES:
  - Intended for only small writes or single-line updates
  - Do not use this for full file content writes


--------------------------------------------------
PATHS BEHAVIOUR
--------------------------------------------------

- All paths are relative to the workspace root
- "" or "." represents the workspace root
- Directory traversal outside workspace is rejected (e.g. "../")
- Ignore rules may exclude files/folders
- Copy and move behave like standard cp/mv commands


--------------------------------------------------
LINES BEHAVIOR
--------------------------------------------------

- Line ranges are 1-based and inclusive
- "*" means unbounded
- Patterns other than 'start-end' are rejected for safety


--------------------------------------------------
RESPONSE FORMAT
--------------------------------------------------

All responses follow below format
Status shows the success or failure of the operation

{
  "status": boolean,
  "result": json
}


--------------------------------------------------
USAGE NOTES
--------------------------------------------------

- Always construct full URLs using workspace URL + command
- Always include required parameters
- Paths and Contents must be URL-encoded if they contain URL unsafe characters
- Prefer minimal queries (avoid unnecessary params)
- Ask user confirmation before executing any state-changing operation (create, copy, move, delete, write)
- No confirmation required for read-only operations (list, content)
