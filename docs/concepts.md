# Chore Concepts

## Tasks

Chore is built around _tasks_, which are the smallest unit of work to be
tracked.  These are represented with a single line of text.  For example:

```
write README.md
```

## Projects

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

## Contexts

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

## Pairs

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

## Tags

Collectively, projects, contexts, and pairs are referred to as "tags."

## Annotations

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

## Completion marker

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

## End date

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

## Priority

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

## Entry date

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

## Task format summary

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
