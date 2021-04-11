use chore::*;

#[test]
fn general() -> Result<()> {
    for (args, expect) in &[
        (
            vec![],
            concat!(
                "1 (M) 2001-02-03 @home +chore add tests\n",
                "2 add task due:2002-03-04T05:06:07\n",
                "3 x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
            ),
        ),
        (
            vec!["all"],
            concat!(
                "1 (M) 2001-02-03 @home +chore add tests\n",
                "2 add task due:2002-03-04T05:06:07\n",
                "3 x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
            ),
        ),
        (
            vec!["-all"],
            "",
        ),
        (
            vec!["+done"],
            concat!(
                "3 x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
            ),
        ),
        (
            vec!["-+done"],
            concat!(
                "1 (M) 2001-02-03 @home +chore add tests\n",
                "2 add task due:2002-03-04T05:06:07\n",
            ),
        ),
        (
            vec!["+chore"],
            concat!(
                "1 (M) 2001-02-03 @home +chore add tests\n",
            ),
        ),
        (
            vec!["-+chore"],
            concat!(
                "2 add task due:2002-03-04T05:06:07\n",
                "3 x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
            ),
        ),
        (
            vec!["@work"],
            concat!(
                "3 x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
            ),
        ),
        (
            vec!["-@work"],
            concat!(
                "1 (M) 2001-02-03 @home +chore add tests\n",
                "2 add task due:2002-03-04T05:06:07\n",
            ),
        ),
        (
            vec!["issue:123"],
            concat!(
                "3 x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
            ),
        ),
        (
            vec!["-issue:123"],
            concat!(
                "1 (M) 2001-02-03 @home +chore add tests\n",
                "2 add task due:2002-03-04T05:06:07\n",
            ),
        ),
        (
            vec!["/task/"],
            concat!(
                "2 add task due:2002-03-04T05:06:07\n",
            ),
        ),
        (
            vec!["!/task/"],
            concat!(
                "1 (M) 2001-02-03 @home +chore add tests\n",
                "3 x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
            ),
        ),
        (
            vec!["/due:/"],
            concat!(
                "2 add task due:2002-03-04T05:06:07\n",
            ),
        ),
        (
            vec!["pri:M"],
            concat!(
                "1 (M) 2001-02-03 @home +chore add tests\n",
            ),
        ),
        (
            vec!["-pri:M"],
            concat!(
                "2 add task due:2002-03-04T05:06:07\n",
                "3 x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
            ),
        ),
        (
            vec!["(M)"],
            concat!(
                "1 (M) 2001-02-03 @home +chore add tests\n",
            ),
        ),
        (
            vec!["-(M)"],
            concat!(
                "2 add task due:2002-03-04T05:06:07\n",
                "3 x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
            ),
        ),
        (
            vec!["(L-N)"],
            concat!(
                "1 (M) 2001-02-03 @home +chore add tests\n",
            ),
        ),
        (
            vec!["-(L-N)"],
            concat!(
                "2 add task due:2002-03-04T05:06:07\n",
                "3 x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
            ),
        ),
        (
            vec!["1"],
            concat!(
                "1 (M) 2001-02-03 @home +chore add tests\n",
            ),
        ),
        (
            vec!["1,3"],
            concat!(
                "1 (M) 2001-02-03 @home +chore add tests\n",
                "3 x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
            ),
        ),
        (
            vec!["2-3"],
            concat!(
                "2 add task due:2002-03-04T05:06:07\n",
                "3 x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
            ),
        ),
        (
            vec!["list"], // test passing an argument that is not a filter
            concat!(
                "1 (M) 2001-02-03 @home +chore add tests\n",
                "2 add task due:2002-03-04T05:06:07\n",
                "3 x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
            ),
        ),
        (
            vec!["end:2001-02-03"],
            concat!(
                "3 x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
            ),
        ),
        (
            vec!["entry:2001-02-03"],
            concat!(
                "1 (M) 2001-02-03 @home +chore add tests\n",
            ),
        ),
        (
            vec!["end.any:"],
            concat!(
                "3 x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
            ),
        ),
        (
            vec!["pri.any:"],
            concat!(
                "1 (M) 2001-02-03 @home +chore add tests\n",
                "3 x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
            ),
        ),
        (
            vec!["entry.any:"],
            concat!(
                "1 (M) 2001-02-03 @home +chore add tests\n",
                "3 x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
            ),
        ),
        (
            vec!["due.any:"],
            concat!(
                "2 add task due:2002-03-04T05:06:07\n",
            ),
        ),
        (
            vec!["end.none:"],
            concat!(
                "1 (M) 2001-02-03 @home +chore add tests\n",
                "2 add task due:2002-03-04T05:06:07\n",
            ),
        ),
        (
            vec!["pri.none:"],
            concat!(
                "2 add task due:2002-03-04T05:06:07\n",
            ),
        ),
        (
            vec!["entry.none:"],
            concat!(
                "2 add task due:2002-03-04T05:06:07\n",
            ),
        ),
        (
            vec!["due.none:"],
            concat!(
                "1 (M) 2001-02-03 @home +chore add tests\n",
                "3 x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
            ),
        ),
        (
            vec!["end.before:2001-02-04"],
            concat!(
                "3 x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
            ),
        ),
        (
            vec!["end.before:2001-02-03"],
            "",
        ),
        (
            vec!["entry.before:2001-02-03"],
            concat!(
                "3 x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
            ),
        ),
        (
            vec!["entry.before:2001-02-04"],
            concat!(
                "1 (M) 2001-02-03 @home +chore add tests\n",
                "3 x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
            ),
        ),
        (
            vec!["due.before:2002-03-04T05:06:08"],
            concat!(
                "2 add task due:2002-03-04T05:06:07\n",
            ),
        ),
        (
            vec!["end.after:2001-02-02"],
            concat!(
                "3 x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
            ),
        ),
        (
            vec!["end.after:2001-02-03"],
            "",
        ),
        (
           vec!["entry.after:2001-01-01"],
           concat!(
               "1 (M) 2001-02-03 @home +chore add tests\n",
               "3 x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
           ),
        ),
        (
           vec!["entry.after:2001-02-02"],
           concat!(
               "1 (M) 2001-02-03 @home +chore add tests\n",
           ),
        ),
        (
           vec!["entry.after:2001-02-03"],
           "",
        ),
        (
            vec!["due.after:2002-03-03"],
            concat!(
                "2 add task due:2002-03-04T05:06:07\n",
            ),
        ),
        (
            vec!["due.after:2002-03-04"],
            "",
        ),
        (
            vec!["end.in:2001-02"],
            concat!(
                "3 x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
            ),
        ),
        (
            vec!["end.in:2001"],
            concat!(
                "3 x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
            ),
        ),
        (
            vec!["end.in:2002"],
            "",
        ),
        (
            vec!["entry.in:2001-02"],
            concat!(
                "1 (M) 2001-02-03 @home +chore add tests\n",
            ),
        ),
        (
            vec!["entry.in:2001"],
            concat!(
                "1 (M) 2001-02-03 @home +chore add tests\n",
                "3 x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
            ),
        ),
        (
            vec!["entry.in:2002"],
            "",
        ),
        (
            vec!["due.in:2002"],
            concat!(
                "2 add task due:2002-03-04T05:06:07\n",
            ),
        ),
        (
            vec!["due.in:2003"],
            "",
        ),
    ] {
        let config = Config {
            now: chrono::NaiveDate::from_ymd(2001, 2, 3).and_hms(4, 5, 6),
            args: args.iter().map(|s| s.to_string()).collect(),
            tasks: Some(
                concat!(
                    "(M) 2001-02-03 @home +chore add tests\n",
                    "add task due:2002-03-04T05:06:07\n",
                    "x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
                )
                .to_owned(),
            ),
            date_keys: Some("due:\nscheduled:\nwait:\nuntil:\n".to_owned()),
            ..Default::default()
        };

        match chore::run(config)? {
            Output::JustPrint { stdout } => {
                assert_eq!(&stdout, expect)
            }
            _ => panic!("expected JustPrint"),
        }
    }
    Ok(())
}

#[test]
fn conflicts() -> Result<()> {
    for (args, defaults, expect) in &[
        (
            vec!["all"],
            "-+done -+hide -until.before:now -wait.after:now",
            concat!(
                "1 (M) 2001-02-03 @home +chore add tests\n",
                "2 add task due:2002-03-04T05:06:07\n",
                "3 x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
            ),
        ),
        (
            vec!["-+done"],
            "all",
            concat!(
                "1 (M) 2001-02-03 @home +chore add tests\n",
                "2 add task due:2002-03-04T05:06:07\n",
            ),
        ),
        (
            vec!["+done"],
            "-+done -+hide -until.before:now -wait.after:now",
            concat!(
                "3 x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
            ),
        ),
        (
            vec!["+chore"],
            "-+chore",
            concat!(
                "1 (M) 2001-02-03 @home +chore add tests\n",
            ),
        ),
        (
            vec!["-+chore"],
            "+chore",
            concat!(
                "2 add task due:2002-03-04T05:06:07\n",
                "3 x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
            ),
        ),
        (
            vec!["@home"],
            "-@home",
            concat!(
                "1 (M) 2001-02-03 @home +chore add tests\n",
            ),
        ),
        (
            vec!["-@home"],
            "@home",
            concat!(
                "2 add task due:2002-03-04T05:06:07\n",
                "3 x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
            ),
        ),
        (
            vec!["/tests/"],
            "/tasks/",
            concat!(
                "1 (M) 2001-02-03 @home +chore add tests\n",
            ),
        ),
        (
            vec!["(A-M)"],
            "(Z)",
            concat!(
                "1 (M) 2001-02-03 @home +chore add tests\n",
                "3 x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
            ),
        ),
        (
            vec!["2-3"],
            "1-2",
            concat!(
                "2 add task due:2002-03-04T05:06:07\n",
                "3 x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
            ),
        ),
        (
            vec!["1,3"],
            "2",
            concat!(
                "1 (M) 2001-02-03 @home +chore add tests\n",
                "3 x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
            ),
        ),
        (
            vec!["end.any:"],
            "!end:today",
            concat!(
                "3 x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
            ),
        ),
        (
            vec!["end.none:"],
            "!end:today",
            concat!(
                "1 (M) 2001-02-03 @home +chore add tests\n",
                "2 add task due:2002-03-04T05:06:07\n",
            ),
        ),
        (
            vec!["end.before:2002"],
            "end:2001-01-01",
            concat!(
                "3 x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
            ),
        ),
        (
            vec!["end.after:2000"],
            "end:2001-01-01",
            concat!(
                "3 x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
            ),
        ),
        (
            vec!["end.in:2001"],
            "end:2001-01-01",
            concat!(
                "3 x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
            ),
        ),
        (
            vec!["pri:M"],
            "pri:Z",
            concat!(
                "1 (M) 2001-02-03 @home +chore add tests\n",
            ),
        ),
        (
            vec!["pri.any:"],
            "pri:Z",
            concat!(
                "1 (M) 2001-02-03 @home +chore add tests\n",
                "3 x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
            ),
        ),
        (
            vec!["pri.none:"],
            "pri:Z",
            concat!(
                "2 add task due:2002-03-04T05:06:07\n",
            ),
        ),
        (
            vec!["entry:2001-02-03"],
            "entry:2001-01-01",
            concat!(
                "1 (M) 2001-02-03 @home +chore add tests\n",
            ),
        ),
        (
            vec!["entry.any:"],
            "entry:2001-01-01",
            concat!(
                "1 (M) 2001-02-03 @home +chore add tests\n",
                "3 x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
            ),
        ),
        (
            vec!["entry.none:"],
            "entry:2001-01-01",
            concat!(
                "2 add task due:2002-03-04T05:06:07\n",
            ),
        ),
        (
            vec!["entry.before:2001-02"],
            "entry:2001-01-01",
            concat!(
                "3 x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
            ),
        ),
        (
            vec!["entry.after:2001-01"],
            "entry:2001-01-01",
            concat!(
                "1 (M) 2001-02-03 @home +chore add tests\n",
            ),
        ),
        (
            vec!["entry.in:2001"],
            "entry:2001-01-01",
            concat!(
                "1 (M) 2001-02-03 @home +chore add tests\n",
                "3 x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
            ),
        ),
        (
            vec!["due:2002-03-04T05:06:07"],
            "due:2001-02-03",
            concat!(
                "2 add task due:2002-03-04T05:06:07\n",
            ),
        ),
        (
            vec!["due.any:"],
            "due:2001-02-03",
            concat!(
                "2 add task due:2002-03-04T05:06:07\n",
            ),
        ),
        (
            vec!["due.none:"],
            "due:2001-02-03",
            concat!(
                "1 (M) 2001-02-03 @home +chore add tests\n",
                "3 x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
            ),
        ),
        (
            vec!["due.after:2001"],
            "due:2001-02-03",
            concat!(
                "2 add task due:2002-03-04T05:06:07\n",
            ),
        ),
        (
            vec!["due.before:2003"],
            "due:2001-02-03",
            concat!(
                "2 add task due:2002-03-04T05:06:07\n",
            ),
        ),
        (
            vec!["due.in:2002"],
            "due:2001-02-03",
            concat!(
                "2 add task due:2002-03-04T05:06:07\n",
            ),
        ),
    ] {
        let config = Config {
            now: chrono::NaiveDate::from_ymd(2001, 2, 3).and_hms(4, 5, 6),
            args: args.iter().map(|s| s.to_string()).collect(),
            tasks: Some(
                concat!(
                    "(M) 2001-02-03 @home +chore add tests\n",
                    "add task due:2002-03-04T05:06:07\n",
                    "x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
                )
                .to_owned(),
            ),
            default_filters: vec![File{name: "test".to_owned(), content: defaults.to_string()}],
            date_keys: Some("due:\nscheduled:\nwait:\nuntil:\n".to_owned()),
            ..Default::default()
        };

        match chore::run(config)? {
            Output::JustPrint { stdout } => {
                assert_eq!(&stdout, expect)
            }
            _ => panic!("expected JustPrint"),
        }
    }
    Ok(())
}

#[test]
fn invalid() -> Result<()> {
    for (args, expect) in &[
        (
            vec!["pri:1"],
            InvalidPriority("pri:1".to_owned()),
        ),
        (
            vec!["end.before:x"],
            ModExpectsDateValue("end.before:x".to_owned()),
        ),
        (
            vec!["pri.before:now"],
            ModExpectsDateKey("pri.before:now".to_owned()),
        ),
        (
            vec!["entry.before:x"],
            ModExpectsDateValue("entry.before:x".to_owned()),
        ),
        (
            vec!["issue.before:now"],
            ModExpectsDateKey("issue.before:now".to_owned()),
        ),
        (
            vec!["due.before:x"],
            ModExpectsDateValue("due.before:x".to_owned()),
        ),
        (
            vec!["end.after:x"],
            ModExpectsDateValue("end.after:x".to_owned()),
        ),
        (
            vec!["pri.after:now"],
            ModExpectsDateKey("pri.after:now".to_owned()),
        ),
        (
            vec!["entry.after:x"],
            ModExpectsDateValue("entry.after:x".to_owned()),
        ),
        (
            vec!["issue.after:now"],
            ModExpectsDateKey("issue.after:now".to_owned()),
        ),
        (
            vec!["due.after:x"],
            ModExpectsDateValue("due.after:x".to_owned()),
        ),
        (
            vec!["end.in:x"],
            ModExpectsDateValue("end.in:x".to_owned()),
        ),
        (
            vec!["pri.in:now"],
            ModExpectsDateKey("pri.in:now".to_owned()),
        ),
        (
            vec!["entry.in:x"],
            ModExpectsDateValue("entry.in:x".to_owned()),
        ),
        (
            vec!["issue.in:now"],
            ModExpectsDateKey("issue.in:now".to_owned()),
        ),
        (
            vec!["due.in:x"],
            ModExpectsDateValue("due.in:x".to_owned()),
        ),
        (
            vec!["due.foo:bar"],
            InvalidMod("due.foo:bar".to_owned()),
        ),
        (
            vec!["/[foo/"],
            InvalidRegex("/[foo/".to_owned()),
        ),
    ] {
        let config = Config {
            now: chrono::NaiveDate::from_ymd(2001, 2, 3).and_hms(4, 5, 6),
            args: args.iter().map(|s| s.to_string()).collect(),
            tasks: Some(
                concat!(
                    "(M) 2001-02-03 @home +chore add tests\n",
                    "add task due:2002-03-04T05:06:07\n",
                    "x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
                )
                .to_owned(),
            ),
            date_keys: Some("due:\nscheduled:\nwait:\nuntil:\n".to_owned()),
            ..Default::default()
        };

        let actual = match chore::run(config) {
            Ok(_) => panic!("expected error"),
            Err(e) => e,
        };
        assert_eq!(format!("{:?}", actual), format!("{:?}", expect));
    }
    Ok(())
}

#[test]
fn defaults() -> Result<()> {
    for (defaults, expect) in &[
        (
            "-+done",
            concat!(
                "1 (M) 2001-02-03 @home +chore add tests\n",
                "2 add task due:2002-03-04T05:06:07\n",
            ),
        ),
        (
            "due.any:",
            "2 add task due:2002-03-04T05:06:07\n",
        ),
        (
            "-+done due.any:",
            "2 add task due:2002-03-04T05:06:07\n",
        ),
    ] {
        let config = Config {
            now: chrono::NaiveDate::from_ymd(2001, 2, 3).and_hms(4, 5, 6),
            args: vec![],
            tasks: Some(
                concat!(
                    "(M) 2001-02-03 @home +chore add tests\n",
                    "add task due:2002-03-04T05:06:07\n",
                    "x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
                )
                .to_owned(),
            ),
            default_filters: vec![File{name: "test".to_owned(), content: defaults.to_string()}],
            date_keys: Some("due:\nscheduled:\nwait:\nuntil:\n".to_owned()),
            ..Default::default()
        };

        match chore::run(config)? {
            Output::JustPrint { stdout } => {
                assert_eq!(&stdout, expect)
            }
            _ => panic!("expected JustPrint"),
        }
    }
    Ok(())
}
