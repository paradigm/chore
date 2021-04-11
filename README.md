# Chore

Chore is yet another opinionated command line task management utility.

Chore uses a plain text file format greatly influenced by but incompatible with
[todo.txt](http://todotxt.org/).  This format interplays nicely with plain
text-friendly tooling such as git and grep, and can be easily maintained
manually in a text editor.

Chore's command line interface was broadly influenced by
[taskwarrior](https://taskwarrior.org/)'s.  In particular, Chore borrows the
filter-command-modification pattern and the `key.mod:` syntax.

## Concepts

### Tasks

Chore is built around _tasks_, which are the smallest unit of work to be
tracked.  These are represented with a single line of text.  For example:

```
write README.md
```

### Projects

A _project_ is metadata associated with a given task used to organize it.
Multiple tasks may be associated with the same project to create a "super" task
of all the tasks with the same project.  If it fits your mental model better,
consider thinking of what Chore calls a project as a "task" and what Chore
calls a task as a "subtask."

Projects are represented by a `+` followed by non-whitespace characters.  They
may appear anywhere within the body of a task.  For example,

```
write +chore README.md
complete +chore test coverage
```

are two tasks associated with the `+chore` project.  A task may have zero, one,
or more projects.

### Contexts

A _context_ is metadata associated with a given task used to restrict the
task's visibility to the time and place it is meaningful.  Contexts are
generally locations, tools, or people required to pursue the task.

Contexts are represented by a `@` followed by non-whitespace characters.  They
may appear anywhere within the body of a task.  For example,

```
@home clean kitchen
give intern tour around @work office
```

are two tasks with different contexts.  The first task is only applicable at
home, and the second only at work.  A task may have zero, one, or more
contexts.

### Pairs

A _pair_ is generic key-value metadata associated with a given task.

These are represented by a _key_ composed of a series of non-whitespace
characters followed by a `:` which is in turn followed by a _value_ composed of
non-whitespace characters.  Keys have several additional constraints to avoid
parsing ambiguities:

- Keys may not contain a `.` or `:`.
- Keys may not start with a `+` or `@`.

Keys must contain at least one character, but values may (for most keys) be
empty.  Pairs may appear anywhere within the body of a task.  For example,

```
@work code review issue:123
```

contains the pair `issue:123` composed of the key `issue` paired with the value
`123`.  A task may have no more than one pair for any given key.

### Tags

Collectively, projects, contexts, and pairs are referred to as "tags."

### Annotations

An _annotation_ is a bit of text associated with a task used to quickly add
information to an already created task.

These are segregated from the main text body by a `|` character.  By design,
they must occur at the end of a task.  For example, after this task is created:

```
+chore implement annotation system
```

the user may wish to annotate additional thoughts:

```
+chore implement annotation system | maybe with pipe character?
```

A task may have zero, one, or more annotations.

### Completion marker

A task which starts with an `x` whitespace separated from any other content is
considered completed; otherwise, a task is considered pending. For example,

```
+chore +document entry and end dates
+chore +document priority
x +chore +document contexts
x +chore +document pairs
x +chore +document projects
```

has two pending tasks and three completed tasks.  This `x` is referred to as a
_completion marker_.

Note that when sorting by ASCII value pending tasks will generally sort before
completed ones.

### End date

A task's _end_ indicates when the task was completed.

These are represented in the `YYYY-MM-DD` form and must immediately follow a
task's completion marker.  For example, in

```
x 2020-12-29 +chore +document projects
x 2020-12-30 +chore +document contexts
x 2020-12-30 +chore +document pairs
```

one task was completed on December 29th, 2020, and two were completed on the
following day.

To ease manual maintenance of Chore files, end dates are optional.

Note that when sorting by ASCII value completed tasks are generally sorted by
end date.

### Priority

A _priority_ is an indication of the relative importance and urgency of a given
task.

These are represented by a `(` followed by a capital letter `A` through `Z`
then a `)`.  Earlier letters in the English alphabet are interpreted as higher
priorities than later letters.  To be interpreted as a priority, these must
follow any completion marker and end date but precede a task's body.  For
example:

```
(H) pay +gas bill due:2020-12-31
(M) +chore +document entry and end dates
(M) +chore +document priority
(Z) +movies watch Citizen Kane
x 2020-12-29 (M) +chore +document projects
x 2020-12-30 (M) +chore +document contexts
x 2020-12-30 (M) +chore +document pairs
```

is composed of tasks with priority `H`, `M`, and `Z`.

A task may lack a priority, in which case it is interpreted a being lower
priority than `(Z)`.

Note that when sorting by ASCII values pending tasks are generally sorted by
priority.

### Entry date

A task's _entry_ indicates when the task was created.

These are represented in `YYYY-MM-DD` form.  To be interpreted as an entry
date, these must immediately precede the task body, following any completion
marker, end date, and priority.

For example:

```
(H) 2020-12-14 pay +gas bill due:2020-12-31
(M) 2020-12-27 +chore +document entry and end dates
(M) 2020-12-27 +chore +document priority
(Z) 2021-01-02 +movies watch Citizen Kane
x 2020-12-29 (M) 2020-12-27 +chore +document projects
x 2020-12-30 (M) 2020-12-27 +chore +document contexts
x 2020-12-30 (M) 2020-12-27 +chore +document pairs
```

To ease manual maintenance of Chore files, entry dates are optional.  Chore's
`add` command will automatically set an entry date.

If a completed task which lacks a priority only has one date, making it
ambiguous whether the date is an end date or an entry date, it is
interpreted as an end date.  For example:

```
x 2020-12-29 +chore +document projects
```

is interpreted to have completed on December 29th, 2020 and lacks an entry date.

### Task format summary

Tasks are represented by:

```
x YYYY-MM-DD (X) YYYY-MM-DD task body +proj @ctx key:val
| |        | | | |        | |                          |
| |        | | | |        |  `-------------------------+- task body, including
| |        | | | `--------+- optional entry               tags and annotations
| |        | `-+- optional priority
| `--------+- optional end
`- completion marker, if task is completed
```

## Dates

Chore interprets a specific list of pairs as describing dates.  Their values
are constrained the recognized formats described below.

All Chore dates include an implied time range.  If a task was completed on a
given day, this means it was completed some time between the start of that day
and that day's end.

### Absolute date format

Chore recognizes the following absolute date formats:

- `YYYY-MM-DDTHH:MM:SS`
- `YYYY-MM-DDTHH:MM`
- `YYYY-MM-DDTHH`
- `YYYY-MM-DD`
- `YYYY-MM`
- `YYYY`

Any fields left out are interpreted as covering the entire range when
performing date comparisons.  For example, `2020-12` is treated as on or after
`2020-12-01T00:00:00` and before `2021-01-01T00:00:00`

The timezone is always assumed to be local.

### Relative date format

Chore recognizes the following relative date formats, which are interpreted
to the next absolute date that matches the description:

- Abbreviated day of the week (e.g. `mon`)
- Day of the week (e.g. `monday`)
- Ordinal numbers (`1st`, `2nd`, `3rd`, etc) corresponding to the day of a
  month
- Abbreviated month (e.g. `oct`)
- Month (e.g. `october`)
- `HH:MM:SS`
- `HH:MM`

Chore also recognizes a series of digits followed by a given character as a
given amount of time in in the future:

- `s` indicates seconds, e.g. `3s`.
- `m` indicates minutes, e.g. `3m`.
- `h` indicates hours, e.g. `3h`.
- `d` indicates days, e.g. `3d`.
- `w` indicates weeks, e.g. `3w`.
- `M` indicates months, e.g. `3M`.
- `y` indicates years, e.g. `3y`.

Chore additionally recognizes these descriptions:

- `now`
- `today`
- `tomorrow`
- `yesterday`

## Special tags

### Dedicated field tags

Chore's format has four positional fields with special meanings:

- The completion marker `x`.
- The optional end date in `YYYY-MM-DD` form.
- The optional priority in `(X)` form.
- The optional entry date in `YYYY-MM-DD` form.

Chore's CLI supports accessing these as tags:

- `+done` indicates a task is completed.
- `end:` is used to reference a task's end date.  Its value may be either a
  date with a resolution of one day, or empty to represent the absence of an
  end date.
- `pri:` is used to reference a task's priority.  It may be any single capital
  letter `A` through `Z`, or empty to represent a task without a priority.
- `entry:` is used to reference a task's entry date.  Its value may be either a
  date with a resolution of one day, or empty to represent the absence of an
  end date.

These keys should not be used in Chore's on-disk files; they are only valid in
the CLI.

### Date keys

Chore knows that the `end:` and `entry:` keys describe absolute dates.  If a
relative date is provided on the CLI, Chore will automatically translate this
to the corresponding absolute date.  If a non-date is provided on the CLI,
Chore will error accordingly.

Chore can be configured to treat additional keys in this manner by listing them
in a file at `~/.chore/date-keys`, one per line.  To emphasize the fact they
are keys, they may optionally contain a trailing `:` character.  For example,
`~/.chore/date-keys` may contain:

```
due:
scheduled:
until:
wait:
```

### recur:

The `recur:` key is used for recurring tasks.  Its value must be a relative
date.  When a task with this key is completed or deleted through Chore, Chore
will automatically create a new instance of the task, with any date keys
adjusted by the `recur:` value.  For example, if `~/.chore/date-keys` contains
(at least):

```
due:
wait:
```

and the task

```
2020-12-01 @home give +dog flea meds recur:1M due:2021-01-07 wait:2021-01-01
```

is modified to be completed through Chore, Chore will generate a new
task with the date fields incremented by one month:

```
2021-01-01 @home give +dog flea meds recur:1M due:2021-02-07 wait:2021-02-01
x 2020-12-01 @home give +dog flea meds recur:1M due:2021-01-07 wait:2021-01-01
```

To delete such a task without creating a new instance, first modify the task to
remove the `recur:` tag.

### +update

Users may not care to clutter the task list with every completed instance of a
frequently occurring task.  The `+update` tag may be used to tell Chore to keep
no more than one completed instance of a given task.  Rather than create a
second completed instance of such a task, a preexisting task which exactly
matches the newly completed one in every way except date values will have the
corresponding task's date values updated.  This is generally paired with the
`recur:` tag.

For example, if the task list contains:

```
2020-12-09 garbage day due:2020-12-09 recur:1w +update
x 2020-12-02 (M) 2020-12-02 garbage day due:2020-12-02 recur:1w +update
```

And chore is told on 2020-12-09 that the pending task is completed, Chore will
update the task list to:

```
2020-12-16 garbage day due:2020-12-16 recur:1w +update
x 2020-12-09 (M) 2020-12-09 garbage day due:2020-12-09 recur:1w +update
```

Where `+update` resulted in the completed task having its dates rather than the
normal behavior of creating a new completed entry, and where `recur:1w` created
a new task.

## Command line syntax

Commands are formatted in the following pattern:

```
chore [filters] [command] [modifications]
```

### Filters

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

#### Inverted filters

All filters can be inverted by prefixing either a `-` or `!` to the filter.
For example, `-@work` will filter out all tasks that have the `@work` context
and `!/foo/` will filter out all tasks that match the regular expression
`/foo/`.

#### Filter aliases

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

#### Default filters

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

### Modifications

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

#### Modification aliases

Users may configure aliases for modifications by creating a directory at
`~/.chore/modification-aliases` which contains files whose name describes the
alias and contents contain whitespace-separated modifications.  For example,
`~/.chore/modification-aliases/reopen` may contain `-+done -end:`, which would
then allow users to run:

```
chore 10 modify reopen
```

to modify a completed task on line 10 to be no longer considered completed.

### Commands

#### Listing commands

These commands may not have any `modification`.

- `list`: lists tasks.  If Chore is run without any non-filter arguments this
  command is assumed by default.
- `projects`: lists all projects in use by at least one task.
- `contexts`: lists all contexts in use by at least one task.
- `keys`: lists all keys in use by at least one task.

#### Non-listing commands

- `add`: Add a new task.  Entire modification text is treated as new task body,
  and an entry date is set automatically.
- `delete`: Delete a task.  No modification is allowed.
- `modify`: modifies tasks with the specified modifications.  At least one
  modification is required.
- `undo`: undo last add, remove, or modify command.

#### Command aliases

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

## Files

Chore operates on files within the `~/.chore` directory.

This is currently not configurable or overridable, but may be so later.

### ~/.chore/tasks

The file at `~/.chore/tasks` contains the list of tasks Chore operates on.
When Chore modifies this file, it sorts it, utilizing the format's natural
ordering tendency to put higher priority pending tasks toward the top.

If this file becomes ungainly, consider manually moving completed contents to
another file, such as `~/.chore/archived`.  Chore does not support doing this
automatically, as in the author's experience tasks do not accumulate fast
enough for the file to become problematically long.

### ~/.chore/date-keys

The file at `~/.chore/date-keys` contains keys which, in addition to the
default `end:` and `entry:`, have values that are expected to be absolute date
times. If a relative date is provided on the CLI, Chore will automatically
translate this to the corresponding absolute date.  If a non-date is provided
on the CLI, Chore will error accordingly.

### ~/.chore/filter-aliases

The directory at `~/.chore/filter-aliases` may contain files whose names are
to be treated as aliases for filters described by the file's contents.

### ~/.chore/default-filters

The directory at `~/.chore/default-filters` may contain files whose names are
ignored by Chore and whose contents are used as default filters.

### ~/.chore/command-aliases

The directory at `~/.chore/command-aliases` may contain files whose names are
to be treated as aliases for commands described by the file's contents.

### ~/.chore/modification-aliases

The directory at `~/.chore/modification-aliases` may contain files whose names are
to be treated as aliases for modifications described by the file's contents.

### ~/.chore/undo

The file at `~/.chore/undo` contains information Chore uses for the `undo` command.

## Configuration examples

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
- New tasks may be created with a `@work` context by default by:
	- Creating `~/.chore/command-aliases/add` which contains `add @work`
