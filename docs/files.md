# Files

Chore operates on files within the `~/.chore` directory.

This is currently not configurable or overridable, but may be so later.

## ~/.chore/tasks

The file at `~/.chore/tasks` contains the list of tasks Chore operates on.
When Chore modifies this file, it sorts it, utilizing the format's natural
ordering tendency to put higher priority pending tasks toward the top.

If this file becomes ungainly, consider manually moving completed contents to
another file, such as `~/.chore/archived`.  Chore does not support doing this
automatically, as in the author's experience tasks do not accumulate fast
enough for the file to become problematically long.

## ~/.chore/date-keys

The file at `~/.chore/date-keys` contains keys which, in addition to the
default `end:` and `entry:`, have values that are expected to be absolute date
times. If a relative date is provided on the CLI, Chore will automatically
translate this to the corresponding absolute date.  If a non-date is provided
on the CLI, Chore will error accordingly.

## ~/.chore/filter-aliases

The directory at `~/.chore/filter-aliases` may contain files whose names are
to be treated as aliases for filters described by the file's contents.

## ~/.chore/default-filters

The directory at `~/.chore/default-filters` may contain files whose names are
ignored by Chore and whose contents are used as default filters.

## ~/.chore/command-aliases

The directory at `~/.chore/command-aliases` may contain files whose names are
to be treated as aliases for commands described by the file's contents.

## ~/.chore/modification-aliases

The directory at `~/.chore/modification-aliases` may contain files whose names are
to be treated as aliases for modifications described by the file's contents.

## ~/.chore/undo

The file at `~/.chore/undo` contains information Chore uses for the `undo` command.

# Configuration examples

Chore can be configured to support the various concepts offered other task
management software such as taskwarrior and various todo.txt clients:

- Completed tasks could be hidden by default by:
	- Creating `~/.chore/default-filter/pending` which contains `-+done`
- A due date could be configured by:
	- Adding `due:` to `~/.chore/date-keys`
	- Creating `~/.chore/filter-aliases/overdue` which contains
	  `due.before:now`
- A scheduled date indicating when work may begin on a task:
	- Adding `scheduled:` to `~/.chore/date-keys`
	- Creating `~/.chore/filter-aliases/ready` which contains
	  `scheduled.before:now`
- A wait date until which a task should be hidden:
	- Adding `wait:` to `~/.chore/date-keys`
	- Creating `~/.chore/filter-aliases/waiting` which contains
	  `wait.after:now`
	- Creating `~/.chore/default-filters/waiting` which contains
	  `-wait.before:now`
- Some people call `wait:` "threshold" and mark it with the terse `t:`:
	- Adding `t:` to `~/.chore/date-keys`
	- Creating `~/.chore/default-filters/threshold` which contains
	  `-t.before:now`
- An until date after which a task is no longer valid and should be hidden:
	- Adding `until:` to `~/.chore/date-keys`
	- Creating `~/.chore/default-filters/missed` which contains
	  `-until.before:now`
- A tag to unconditionally hide tasks:
	- Creating `~/.chore/default-filters/hide` which contains `-+hide`
- New tasks may be created with an entry date by default via:
	- Creating `~/.chore/command-aliases/add` which contains `add entry:today`
- New tasks may be created with a `@work` context by default by:
	- Creating `~/.chore/command-aliases/add` which contains `add @work`
	- Or, if combine with the entry date, `add entry:today @work`
