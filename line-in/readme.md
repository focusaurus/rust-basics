# line-in

This program ensures a particular line exists within a file. The program is idempotent: it leaves the file unchanged and exits zero (success) if the line is already present. If the line is not presented, it is appended to the file.

## Usage

`line-in path/to/some/file some-line-that-should-be-there`

- first argument is a path
  - can be absolute or relative
  - should exist and be writeable
- second argument is the text of the line to add
  - Shell quotes may be necessary
  - `line-in /tmp/foo.txt 'Quote or escape spaces and **shell** metachars!'`
