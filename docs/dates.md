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
