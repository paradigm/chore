# Dates

Chore interprets a specific list of pairs as describing dates.  Their values
are constrained the recognized formats described below.

All Chore dates include an implied time range.  If a task was completed on a
given day, this means it was completed some time between the start of that day
and that day's end.

## Absolute date format

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

## Relative date format

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

# Special tags

## Dedicated field tags

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

## Date keys

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

## recur:

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

## +update

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
