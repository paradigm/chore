# Command line syntax

Commands are formatted in the following pattern:

```
chore [filters] [command] [modifications]
```

## Filters

A filter is information used to restrict the tasks a command applies to.
Filters may be any of:

- A `+project`, `@context`, or `key:value` pair: tasks which have the given
  tag.
- A key with a modifier:
	- `key.any:`: tasks which have the key, irrelevant of its value.
	- `key.none:`: tasks which lack a pair with the key.
	- `key.before:date:`: tasks with the key whose value ends before the
	  specified date's starts.
	- `key.after:date:`: tasks with the key whose value starts after the
	  specified date's ends.
	- `key.in:date:`: tasks with the key whose value either matches or is
	  contained within the specified date.
- `/regex/`: tasks which match the regular expression.
- `(P)`: tasks which have the specified priority.
- `(A-Z)`: tasks which have a priority within the specified range.
- `1`: tasks which occur on the specified line number.
- `1-3`: tasks which occur within the specified range of line numbers.
- `1,2,3`: tasks which have any of the listed line numbers.
- `all`: this matches all filters.  Its intended use is to override Chore's
  default filters.

### Inverted filters

All filters can be inverted by prefixing either a `-` or `!` to the filter.
For example, `-@work` will filter out all tasks that have the `@work` context
and `!/foo/` will filter out all tasks that match the regular expression
`/foo/`.

### Filter aliases

Users may configure aliases for filters by creating a directory at
`~/.chore/filter-aliases` which contains files whose name describes the alias
and contents contain newline-separated filters.  For example:

- `~/.chore/filter-aliases/waiting` may contain `wait.after:now`
- `~/.chore/filter-aliases/overdue` may contain `due.before:now`
- `~/.chore/filter-aliases/ready` may contain `scheduled.before:now`
- `~/.chore/filter-aliases/pending` may contain `+done`
- `~/.chore/filter-aliases/completed` may contain `-+done`

An example of an alias that describes multiple filters may be a file at
`~/.chore/filter-aliases/missed` which contains:

```
-+done
until.before:now
```

### Default filters

Chore can be configured to apply a list of filters by default by creating a
directory at `~/.chore/default-filters` which contains files that in turn
contain filters, no more than once per line.  For example

- `~/.chore/default-filters/hide-completed` may contain `-+done`
- `~/.chore/default-filters/hide-missed` may contain `-until.after:now`
- `~/.chore/default-filters/hide-waiting` may contain `-wait.before:now`
- `~/.chore/default-filters/hide` may contain `-+hide`

Such filters may be overridden on the command line:

- Any filter that operates on the same term as a default filter will disable
  the default filter.
	- For example, `wait.any:` overrides a default `-wait.before:now`.
	- For example, `+hide` overrides a default `-+hide`.
	- For example, `pri.any:` overrides a default `(A-M)`.
- Any regex filter overrides all default regex filters.
- Any line number filter overrides all default line number filters.
- The `all` filter, which disables all default filters.

The choice to make default-filters a directory rather than a file was primarily
to ease automation around creating and removing default filters; specifically,
around changing contexts.  For example, if someone works from home, a default
`@work` filter may exist during work hours, then be overwritten to `@home` on
nights and weekends.  A mobile device might detect it is in a grocery store via
GPS or wifi and set the default context to `@shopping`.

In practice, a user may be in multiple contexts at once.  For example, a user
may both be in a "@home" context and a "@dog" context when home with his or her
dog.  Chore does not have a way to directly express "or" filters, but it does
support negated filters; consequently De Morgan's law may be of use: users may
create filters which negate currently invalid contexts to effectively pass
multiple valid contexts.  For example, nights and weekends may have a default
`-@work` filter, and GPS or wifi may create a `-@home` when away from home.

## Modifications

A modification is information used by the `modify` command to change the
contents of tasks that pass the provided filters.

- `+project` or `@context: if the task lags the tag, add it.
- `key:value`: if the task lacks the key, add the pair.  If the task has the
  tag, update its value.
- `-+project`, `!+project`, `-@context`, or `!@context`, `-key:value`,
  `!key:value`: if the task has the tag, remove it.
- `-key:` or `!key:`: if the task has the key irrelevant of value, remove it.
- `>>text`: append text.  Following ambiguous fields are also appended.
- Otherwise, the modifiction is assumed to be new body text that completely
  overwrites the task body.  Following ambiguous fields are interpreted as
  additional new body content.

### Modification aliases

Users may configure aliases for modifications by creating a directory at
`~/.chore/modification-aliases` which contains files whose name describes the
alias and contents contain whitespace-separated modifications.  For example,
`~/.chore/modification-aliases/reopen` may contain `-+done -end:`, which would
then allow users to run:

```
chore 10 modify reopen
```

to modify a completed task on line 10 to be no longer considered completed.

## Commands

### Listing commands

These commands may not have any `modification`.

- `list`: lists tasks.  If Chore is run without any non-filter arguments this
  command is assumed by default.
- `projects`: lists all projects in use by at least one task.
- `contexts`: lists all contexts in use by at least one task.
- `keys`: lists all keys in use by at least one task.

### Non-listing commands

- `add`: Add a new task.  Entire modification text is treated as new task body,
  and an entry date is set automatically.
- `delete`: Delete a task.  No modification is allowed.
- `modify`: modifies tasks with the specified modifications.  At least one
  modification is required.
- `undo`: undo last add, remove, or modify command.

### Command aliases

Users may configure aliases for commands by creating a directory at
`~/.chore/command-aliases` which contains files whose name describes the alias
and contents contain whitespace-separated filters.  For example:

- `~/.chore/command-aliases/a` may contain `add`
- `~/.chore/command-aliases/proj` may contain `projects`
- `~/.chore/command-aliases/ctx` may contain `contexts`
- `~/.chore/command-aliases/mod` may contain `modify`
- `~/.chore/command-aliases/done` may contain `modify +done end:today`
- `~/.chore/command-aliases/prepend` may contain `modify <<`
- `~/.chore/command-aliases/append` may contain `modify >>`
- `~/.chore/command-aliases/note` may contain `modify >>|`

This can be used to set default modifications for a command.  For example,
during work hours a file at `~/.chore/command-aliases/add` may be created which
contains `add @work`, setting the `@work` context by default.  On nights and
weekends, this command alias may be deleted, after which `add` refrains from
setting any context.
