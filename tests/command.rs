use chore::*;

#[test]
fn general() -> Result<()> {
    for (tasks, args, undo, expect) in &[
        (
            concat!(
                "(M) 2001-02-03 @home +chore add tests\n",
                "add task due:2002-03-04T05:06:07\n",
                "x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
            ),
            vec![],
            None,
            Output::JustPrint {
                stdout: concat!(
                    "1 (M) 2001-02-03 @home +chore add tests\n",
                    "2 add task due:2002-03-04T05:06:07\n",
                    "3 x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
                )
                .to_string(),
            },
        ),
        (
            concat!(
                "(M) 2001-02-03 @home +chore add tests\n",
                "add task due:2002-03-04T05:06:07\n",
                "x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
            ),
            vec!["list"],
            None,
            Output::JustPrint {
                stdout: concat!(
                    "1 (M) 2001-02-03 @home +chore add tests\n",
                    "2 add task due:2002-03-04T05:06:07\n",
                    "3 x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
                )
                .to_string(),
            },
        ),
        (
            concat!(
                "a\n",
                "b\n",
                "c\n",
                "d\n",
                "e\n",
                "f\n",
                "g\n",
                "h\n",
                "i\n",
                "j\n",
                "k\n",
                "l\n",
            ),
            vec!["list"],
            None,
            Output::JustPrint {
                stdout: concat!(
                    " 1 a\n",
                    " 2 b\n",
                    " 3 c\n",
                    " 4 d\n",
                    " 5 e\n",
                    " 6 f\n",
                    " 7 g\n",
                    " 8 h\n",
                    " 9 i\n",
                    "10 j\n",
                    "11 k\n",
                    "12 l\n",
                )
                .to_string(),
            },
        ),
        (
            concat!(
                "(M) 2001-02-03 @home +chore add tests\n",
                "add task due:2002-03-04T05:06:07\n",
                "x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
            ),
            vec!["projects"],
            None,
            Output::JustPrint {
                stdout: concat!("+chore\n",).to_string(),
            },
        ),
        (
            concat!(
                "(M) 2001-02-03 @home +chore add tests\n",
                "add task due:2002-03-04T05:06:07\n",
                "x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
            ),
            vec!["contexts"],
            None,
            Output::JustPrint {
                stdout: concat!("@home\n", "@work\n",).to_string(),
            },
        ),
        (
            concat!(
                "(M) 2001-02-03 @home +chore add tests\n",
                "add task due:2002-03-04T05:06:07\n",
                "x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
            ),
            vec!["keys"],
            None,
            Output::JustPrint {
                stdout: concat!("due:\n", "issue:\n",).to_string(),
            },
        ),
        (
            concat!(
                "(M) 2001-02-03 @home +chore add tests\n",
                "add task due:2002-03-04T05:06:07\n",
                "x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
            ),
            vec!["add", "pri:Z", "foo", "bar"],
            None,
            Output::WriteFiles {
                stdout: "ADD (Z) foo bar\n".to_string(),
                confirm: false,
                tasks: concat!(
                    "(M) 2001-02-03 @home +chore add tests\n",
                    "(Z) foo bar\n",
                    "add task due:2002-03-04T05:06:07\n",
                    "x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
                )
                .to_string(),
                undo: "---\nADD (Z) foo bar\n".to_string(),
            },
        ),
        (
            concat!(
                "(M) 2001-02-03 @home +chore add tests\n",
                "add task due:2002-03-04T05:06:07\n",
                "x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
            ),
            vec!["+chore", "delete"],
            None,
            Output::WriteFiles {
                stdout: "DEL (M) 2001-02-03 @home +chore add tests\n".to_string(),
                confirm: false,
                tasks: concat!(
                    "add task due:2002-03-04T05:06:07\n",
                    "x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
                )
                .to_string(),
                undo: "---\nDEL (M) 2001-02-03 @home +chore add tests\n".to_string(),
            },
        ),
        (
            concat!(
                "(M) 2001-02-03 @home +chore add tests\n",
                "add task due:2002-03-04T05:06:07\n",
                "x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
            ),
            vec!["due.any:", "delete"],
            None,
            Output::WriteFiles {
                stdout: "DEL add task due:2002-03-04T05:06:07\n".to_string(),
                confirm: false,
                tasks: concat!(
                    "(M) 2001-02-03 @home +chore add tests\n",
                    "x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
                )
                .to_string(),
                undo: "---\nDEL add task due:2002-03-04T05:06:07\n".to_string(),
            },
        ),
        (
            concat!(
                "(M) 2001-02-03 @home +chore add tests\n",
                "add task due:2002-03-04T05:06:07\n",
                "x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
            ),
            vec!["+done", "delete"],
            None,
            Output::WriteFiles {
                stdout: "DEL x 2001-02-03 (H) 2001-01-02 @work issue:123\n".to_string(),
                confirm: false,
                tasks: concat!(
                    "(M) 2001-02-03 @home +chore add tests\n",
                    "add task due:2002-03-04T05:06:07\n",
                )
                .to_string(),
                undo: "---\nDEL x 2001-02-03 (H) 2001-01-02 @work issue:123\n".to_string(),
            },
        ),
        (
            concat!(
                "(M) 2001-02-03 @home +chore add tests\n",
                "add task due:2002-03-04T05:06:07\n",
                "x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
            ),
            vec!["/add/", "delete"],
            None,
            Output::WriteFiles {
                stdout: concat!(
                    "DEL (M) 2001-02-03 @home +chore add tests\n",
                    "DEL add task due:2002-03-04T05:06:07\n",
                )
                .to_string(),
                confirm: true,
                tasks: concat!("x 2001-02-03 (H) 2001-01-02 @work issue:123\n",).to_string(),
                undo: concat!(
                    "---\n",
                    "DEL (M) 2001-02-03 @home +chore add tests\n",
                    "DEL add task due:2002-03-04T05:06:07\n",
                )
                .to_string(),
            },
        ),
        (
            concat!(
                "(M) 2001-02-03 @home +chore add tests\n",
                "add task due:2002-03-04T05:06:07\n",
                "x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
            ),
            vec!["delete"],
            None,
            Output::WriteFiles {
                stdout: concat!(
                    "DEL (M) 2001-02-03 @home +chore add tests\n",
                    "DEL add task due:2002-03-04T05:06:07\n",
                    "DEL x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
                )
                .to_string(),
                confirm: true,
                tasks: "".to_string(),
                undo: concat!(
                    "---\n",
                    "DEL (M) 2001-02-03 @home +chore add tests\n",
                    "DEL add task due:2002-03-04T05:06:07\n",
                    "DEL x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
                )
                .to_string(),
            },
        ),
        (
            concat!(
                "add task due:2002-03-04T05:06:07\n",
                "x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
            ),
            vec!["undo"],
            Some(concat!("---\n", "DEL (M) 2001-02-03 @home +chore add tests\n",).to_string()),
            Output::WriteFiles {
                stdout: concat!("ADD (M) 2001-02-03 @home +chore add tests\n",).to_string(),
                confirm: true,
                tasks: concat!(
                    "(M) 2001-02-03 @home +chore add tests\n",
                    "add task due:2002-03-04T05:06:07\n",
                    "x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
                )
                .to_string(),
                undo: "".to_string(),
            },
        ),
        (
            concat!(
                "x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
            ),
            vec!["undo"],
            Some(concat!("---\n", "DEL add task due:2002-03-04T05:06:07\n",).to_string()),
            Output::WriteFiles {
                stdout: concat!("ADD add task due:2002-03-04T05:06:07\n",).to_string(),
                confirm: true,
                tasks: concat!(
                    "add task due:2002-03-04T05:06:07\n",
                    "x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
                )
                .to_string(),
                undo: "".to_string(),
            },
        ),
        (
            concat!(
                "(Z) foo bar\n",
                "add task due:2002-03-04T05:06:07\n",
                "x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
            ),
            vec!["undo"],
            Some(concat!(
                    "---\n",
                    "DEL (M) 2001-02-03 @home +chore add tests\n",
                    "---\n",
                    "ADD (Z) foo bar\n",
                    ).to_string()),
            Output::WriteFiles {
                stdout: concat!("DEL (Z) foo bar\n",).to_string(),
                confirm: true,
                tasks: concat!(
                    "add task due:2002-03-04T05:06:07\n",
                    "x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
                )
                .to_string(),
                undo: concat!(
                    "---\n",
                    "DEL (M) 2001-02-03 @home +chore add tests\n",
                ).to_string(),
            },
        ),
    ] {
        let config = Config {
            now: chrono::NaiveDate::from_ymd(2001, 2, 3).and_hms(4, 5, 6),
            args: args.iter().map(|s| s.to_string()).collect(),
            tasks: Some(tasks.to_string()),
            date_keys: Some("due:\nscheduled:\nwait:\nuntil:\n".to_owned()),
            undo: undo.clone(),
            ..Default::default()
        };

        let actual = chore::run(config)?;
        assert_eq!(actual, *expect);
    }
    Ok(())
}
#[test]
fn invalid() -> Result<()> {
    for (tasks, args, undo, expect) in &[
        (
            "",
            vec!["not-a-command"],
            None,
            NotAFilterOrCommand("not-a-command".to_owned()),
        ),
        (
            "",
            vec!["list", "+done"],
            None,
            CmdDisallowsMod,
        ),
        (
            "",
            vec!["projects", "+done"],
            None,
            CmdDisallowsMod,
        ),
        (
            "",
            vec!["contexts", "+done"],
            None,
            CmdDisallowsMod,
        ),
        (
            "",
            vec!["keys", "+done"],
            None,
            CmdDisallowsMod,
        ),
        (
            "",
            vec!["delete", "+done"],
            None,
            CmdDisallowsMod,
        ),
        (
            "",
            vec!["undo", "+done"],
            None,
            CmdDisallowsMod,
        ),
        (
            "",
            vec!["undo"],
            Some("---\nADD (Z) foo bar\n".to_string()),
            UndoMismatch("(Z) foo bar".to_string()),
        ),
        (
            "",
            vec!["undo"],
            Some("---\nMOD (Z) foo bar\n".to_string()),
            MalformedUndo("MOD (Z) foo bar".to_string()),
        ),
    ] {
        let config = Config {
            now: chrono::NaiveDate::from_ymd(2001, 2, 3).and_hms(4, 5, 6),
            args: args.iter().map(|s| s.to_string()).collect(),
            tasks: Some(tasks.to_string()),
            date_keys: Some("due:\nscheduled:\nwait:\nuntil:\n".to_owned()),
            undo: undo.clone(),
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
