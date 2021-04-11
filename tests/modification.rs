use chore::*;

#[test]
fn general() -> Result<()> {
    for (args, expect_stdout, expect_confirm, expect_tasks) in &[
        (
            vec!["modify", "+done"],
            concat!(
                "DEL (M) 2001-02-03 @home +chore add tests\n",
                "ADD x (M) 2001-02-03 @home +chore add tests\n",
                "DEL add task due:2002-03-04T05:06:07\n",
                "ADD x add task due:2002-03-04T05:06:07\n",
            ),
            true,
            concat!(
                "x (M) 2001-02-03 @home +chore add tests\n",
                "x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
                "x add task due:2002-03-04T05:06:07\n"
            ),
        ),
        (
            vec!["modify", "-+done"],
            concat!(
                "DEL x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
                "ADD (H) 2001-01-02 @work issue:123\n",
            ),
            false,
            concat!(
                "(H) 2001-01-02 @work issue:123\n",
                "(M) 2001-02-03 @home +chore add tests\n",
                "add task due:2002-03-04T05:06:07\n",
            ),
        ),
        (
            vec!["modify", "+chore"],
            concat!(
                "DEL add task due:2002-03-04T05:06:07\n",
                "ADD add task due:2002-03-04T05:06:07 +chore\n",
                "DEL x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
                "ADD x 2001-02-03 (H) 2001-01-02 @work issue:123 +chore\n",
            ),
            true,
            concat!(
                "(M) 2001-02-03 @home +chore add tests\n",
                "add task due:2002-03-04T05:06:07 +chore\n",
                "x 2001-02-03 (H) 2001-01-02 @work issue:123 +chore\n",
            ),
        ),
        (
            vec!["modify", "-+chore"],
            concat!(
                "DEL (M) 2001-02-03 @home +chore add tests\n",
                "ADD (M) 2001-02-03 @home add tests\n",
            ),
            false,
            concat!(
                "(M) 2001-02-03 @home add tests\n",
                "add task due:2002-03-04T05:06:07\n",
                "x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
            ),
        ),
        (
            vec!["modify", "@work"],
            concat!(
                "DEL (M) 2001-02-03 @home +chore add tests\n",
                "ADD (M) 2001-02-03 @home +chore add tests @work\n",
                "DEL add task due:2002-03-04T05:06:07\n",
                "ADD add task due:2002-03-04T05:06:07 @work\n",
            ),
            true,
            concat!(
                "(M) 2001-02-03 @home +chore add tests @work\n",
                "add task due:2002-03-04T05:06:07 @work\n",
                "x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
            ),
        ),
        (
            vec!["modify", "-@work"],
            concat!(
                "DEL x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
                "ADD x 2001-02-03 (H) 2001-01-02 issue:123\n",
            ),
            false,
            concat!(
                "(M) 2001-02-03 @home +chore add tests\n",
                "add task due:2002-03-04T05:06:07\n",
                "x 2001-02-03 (H) 2001-01-02 issue:123\n",
            ),
        ),
        (
            vec!["modify", "issue:999"],
            concat!(
                "DEL (M) 2001-02-03 @home +chore add tests\n",
                "ADD (M) 2001-02-03 @home +chore add tests issue:999\n",
                "DEL add task due:2002-03-04T05:06:07\n",
                "ADD add task due:2002-03-04T05:06:07 issue:999\n",
                "DEL x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
                "ADD x 2001-02-03 (H) 2001-01-02 @work issue:999\n",
            ),
            true,
            concat!(
                "(M) 2001-02-03 @home +chore add tests issue:999\n",
                "add task due:2002-03-04T05:06:07 issue:999\n",
                "x 2001-02-03 (H) 2001-01-02 @work issue:999\n",
            ),
        ),
        (
            vec!["modify", "end:2009-09-09"],
            concat!(
                "DEL x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
                "ADD x 2009-09-09 (H) 2001-01-02 @work issue:123\n",
            ),
            false,
            concat!(
                "(M) 2001-02-03 @home +chore add tests\n",
                "add task due:2002-03-04T05:06:07\n",
                "x 2009-09-09 (H) 2001-01-02 @work issue:123\n",
            ),
        ),
        (
            vec!["modify", "pri:Z"],
            concat!(
                "DEL (M) 2001-02-03 @home +chore add tests\n",
                "ADD (Z) 2001-02-03 @home +chore add tests\n",
                "DEL add task due:2002-03-04T05:06:07\n",
                "ADD (Z) add task due:2002-03-04T05:06:07\n",
                "DEL x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
                "ADD x 2001-02-03 (Z) 2001-01-02 @work issue:123\n",
            ),
            true,
            concat!(
                "(Z) 2001-02-03 @home +chore add tests\n",
                "(Z) add task due:2002-03-04T05:06:07\n",
                "x 2001-02-03 (Z) 2001-01-02 @work issue:123\n",
            ),
        ),
        (
            vec!["modify", "entry:2009-09-09"],
            concat!(
                "DEL (M) 2001-02-03 @home +chore add tests\n",
                "ADD (M) 2009-09-09 @home +chore add tests\n",
                "DEL add task due:2002-03-04T05:06:07\n",
                "ADD 2009-09-09 add task due:2002-03-04T05:06:07\n",
                "DEL x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
                "ADD x 2001-02-03 (H) 2009-09-09 @work issue:123\n",
            ),
            true,
            concat!(
                "(M) 2009-09-09 @home +chore add tests\n",
                "2009-09-09 add task due:2002-03-04T05:06:07\n",
                "x 2001-02-03 (H) 2009-09-09 @work issue:123\n",
            ),
        ),
        (
            vec!["modify", "due:2009-09-09"],
            concat!(
                "DEL (M) 2001-02-03 @home +chore add tests\n",
                "ADD (M) 2001-02-03 @home +chore add tests due:2009-09-09\n",
                "DEL add task due:2002-03-04T05:06:07\n",
                "ADD add task due:2009-09-09\n",
                "DEL x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
                "ADD x 2001-02-03 (H) 2001-01-02 @work issue:123 due:2009-09-09\n",
            ),
            true,
            concat!(
                "(M) 2001-02-03 @home +chore add tests due:2009-09-09\n",
                "add task due:2009-09-09\n",
                "x 2001-02-03 (H) 2001-01-02 @work issue:123 due:2009-09-09\n",
            ),
        ),
        (
            vec!["modify", "end:"],
            concat!(
                "DEL x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
                "ADD x (H) 2001-01-02 @work issue:123\n",
            ),
            false,
            concat!(
                "(M) 2001-02-03 @home +chore add tests\n",
                "add task due:2002-03-04T05:06:07\n",
                "x (H) 2001-01-02 @work issue:123\n",
            ),
        ),
        (
            vec!["modify", "pri:"],
            concat!(
                "DEL (M) 2001-02-03 @home +chore add tests\n",
                "ADD 2001-02-03 @home +chore add tests\n",
                "DEL x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
                "ADD x 2001-02-03 2001-01-02 @work issue:123\n",
            ),
            true,
            concat!(
                "2001-02-03 @home +chore add tests\n",
                "add task due:2002-03-04T05:06:07\n",
                "x 2001-02-03 2001-01-02 @work issue:123\n",
            ),
        ),
        (
            vec!["modify", "entry:"],
            concat!(
                "DEL (M) 2001-02-03 @home +chore add tests\n",
                "ADD (M) @home +chore add tests\n",
                "DEL x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
                "ADD x 2001-02-03 (H) @work issue:123\n",
            ),
            true,
            concat!(
                "(M) @home +chore add tests\n",
                "add task due:2002-03-04T05:06:07\n",
                "x 2001-02-03 (H) @work issue:123\n",
            ),
        ),
        (
            vec!["modify", "due:"],
            concat!("DEL add task due:2002-03-04T05:06:07\n", "ADD add task\n",),
            false,
            concat!(
                "(M) 2001-02-03 @home +chore add tests\n",
                "add task\n",
                "x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
            ),
        ),
        (
            vec!["modify", "issue:"],
            concat!(
                "DEL (M) 2001-02-03 @home +chore add tests\n",
                "ADD (M) 2001-02-03 @home +chore add tests issue:\n",
                "DEL add task due:2002-03-04T05:06:07\n",
                "ADD add task due:2002-03-04T05:06:07 issue:\n",
                "DEL x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
                "ADD x 2001-02-03 (H) 2001-01-02 @work issue:\n",
            ),
            true,
            concat!(
                "(M) 2001-02-03 @home +chore add tests issue:\n",
                "add task due:2002-03-04T05:06:07 issue:\n",
                "x 2001-02-03 (H) 2001-01-02 @work issue:\n",
            ),
        ),
        (
            vec!["modify", "-end:"],
            concat!(
                "DEL x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
                "ADD x (H) 2001-01-02 @work issue:123\n",
            ),
            false,
            concat!(
                "(M) 2001-02-03 @home +chore add tests\n",
                "add task due:2002-03-04T05:06:07\n",
                "x (H) 2001-01-02 @work issue:123\n",
            ),
        ),
        (
            vec!["modify", "-pri:"],
            concat!(
                "DEL (M) 2001-02-03 @home +chore add tests\n",
                "ADD 2001-02-03 @home +chore add tests\n",
                "DEL x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
                "ADD x 2001-02-03 2001-01-02 @work issue:123\n",
            ),
            true,
            concat!(
                "2001-02-03 @home +chore add tests\n",
                "add task due:2002-03-04T05:06:07\n",
                "x 2001-02-03 2001-01-02 @work issue:123\n",
            ),
        ),
        (
            vec!["modify", "-entry:"],
            concat!(
                "DEL (M) 2001-02-03 @home +chore add tests\n",
                "ADD (M) @home +chore add tests\n",
                "DEL x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
                "ADD x 2001-02-03 (H) @work issue:123\n",
            ),
            true,
            concat!(
                "(M) @home +chore add tests\n",
                "add task due:2002-03-04T05:06:07\n",
                "x 2001-02-03 (H) @work issue:123\n",
            ),
        ),
        (
            vec!["modify", "-issue:"],
            concat!(
                "DEL x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
                "ADD x 2001-02-03 (H) 2001-01-02 @work\n",
            ),
            false,
            concat!(
                "(M) 2001-02-03 @home +chore add tests\n",
                "add task due:2002-03-04T05:06:07\n",
                "x 2001-02-03 (H) 2001-01-02 @work\n",
            ),
        ),
        (
            vec!["modify", ">>Z"],
            concat!(
                "DEL (M) 2001-02-03 @home +chore add tests\n",
                "ADD (M) 2001-02-03 @home +chore add tests Z\n",
                "DEL add task due:2002-03-04T05:06:07\n",
                "ADD add task due:2002-03-04T05:06:07 Z\n",
                "DEL x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
                "ADD x 2001-02-03 (H) 2001-01-02 @work issue:123 Z\n",
            ),
            true,
            concat!(
                "(M) 2001-02-03 @home +chore add tests Z\n",
                "add task due:2002-03-04T05:06:07 Z\n",
                "x 2001-02-03 (H) 2001-01-02 @work issue:123 Z\n",
            ),
        ),
        (
            vec!["modify", ">>A", "B", "C", "pri:Z"],
            concat!(
                "DEL (M) 2001-02-03 @home +chore add tests\n",
                "ADD (Z) 2001-02-03 @home +chore add tests A B C\n",
                "DEL add task due:2002-03-04T05:06:07\n",
                "ADD (Z) add task due:2002-03-04T05:06:07 A B C\n",
                "DEL x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
                "ADD x 2001-02-03 (Z) 2001-01-02 @work issue:123 A B C\n",
            ),
            true,
            concat!(
                "(Z) 2001-02-03 @home +chore add tests A B C\n",
                "(Z) add task due:2002-03-04T05:06:07 A B C\n",
                "x 2001-02-03 (Z) 2001-01-02 @work issue:123 A B C\n",
            ),
        ),
        (
            vec!["modify", "Z"],
            concat!(
                "DEL (M) 2001-02-03 @home +chore add tests\n",
                "ADD (M) 2001-02-03 Z\n",
                "DEL add task due:2002-03-04T05:06:07\n",
                "ADD Z\n",
                "DEL x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
                "ADD x 2001-02-03 (H) 2001-01-02 Z\n",
            ),
            true,
            concat!(
                "(M) 2001-02-03 Z\n",
                "Z\n",
                "x 2001-02-03 (H) 2001-01-02 Z\n",
            ),
        ),
        (
            vec!["add", "pri:Z", "@home", "entry:2009-09-09", "+chore", "example"],
            concat!(
                "ADD (Z) 2009-09-09 @home +chore example\n",
            ),
            false,
            concat!(
                "(M) 2001-02-03 @home +chore add tests\n",
                "(Z) 2009-09-09 @home +chore example\n",
                "add task due:2002-03-04T05:06:07\n",
                "x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
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
            date_keys: Some("due:\nscheduled:\nwait:\nuntil:\n".to_owned()),
            ..Default::default()
        };

        let mut expect_undo = String::from("---\n");
        expect_undo.push_str(expect_stdout);

        match chore::run(config)? {
            Output::WriteFiles {
                stdout,
                confirm,
                tasks,
                undo,
            } => {
                assert_eq!(&stdout, expect_stdout);
                assert_eq!(&confirm, expect_confirm);
                assert_eq!(&tasks, expect_tasks);
                assert_eq!(undo, expect_undo);
            }
            _ => panic!("expected WriteFiles"),
        }
    }
    Ok(())
}

#[test]
fn special_tags() -> Result<()> {
    for (tasks, args, expect_stdout, expect_confirm, expect_tasks) in &[
        // recur:
        (
            concat!(
                "(M) 2001-02-03 @home +chore recur:1w add tests\n",
                "add task due:2002-03-04T05:06:07\n",
                "x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
            ),
            vec!["recur.any:", "modify", "+done", "end:today"],
            concat!(
                "DEL (M) 2001-02-03 @home +chore recur:1w add tests\n",
                "ADD (M) 2001-02-10 @home +chore recur:1w add tests\n",
                "ADD x 2001-02-03 (M) 2001-02-03 @home +chore recur:1w add tests\n",
            ),
            false,
            concat!(
                "(M) 2001-02-10 @home +chore recur:1w add tests\n",
                "add task due:2002-03-04T05:06:07\n",
                "x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
                "x 2001-02-03 (M) 2001-02-03 @home +chore recur:1w add tests\n",
            ),
        ),
        (
            concat!(
                "(M) 2001-02-03 @home +chore add tests\n",
                "add task due:2002-03-04T05:06:07 recur:1w\n",
                "x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
            ),
            vec!["recur.any:", "modify", "+done", "end:today"],
            concat!(
                "DEL add task due:2002-03-04T05:06:07 recur:1w\n",
                "ADD add task due:2002-03-11T05:06:07 recur:1w\n",
                "ADD x 2001-02-03 add task due:2002-03-04T05:06:07 recur:1w\n",
            ),
            false,
            concat!(
                "(M) 2001-02-03 @home +chore add tests\n",
                "add task due:2002-03-11T05:06:07 recur:1w\n",
                "x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
                "x 2001-02-03 add task due:2002-03-04T05:06:07 recur:1w\n",
            ),
        ),
        // +update
        (
            concat!(
                "(M) 2001-02-03 @home +chore add tests +update\n",
                "add task due:2002-03-04T05:06:07\n",
                "x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
            ),
            vec!["+update", "modify", "+done", "end:today"],
            concat!(
                "DEL (M) 2001-02-03 @home +chore add tests +update\n",
                "ADD x 2001-02-03 (M) 2001-02-03 @home +chore add tests +update\n",
            ),
            false,
            concat!(
                "add task due:2002-03-04T05:06:07\n",
                "x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
                "x 2001-02-03 (M) 2001-02-03 @home +chore add tests +update\n",
            ),
        ),
        (
            concat!(
                "(M) 2001-02-03 @home +chore add tests\n",
                "add task due:2002-03-04T05:06:07 +update\n",
                "x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
            ),
            vec!["+update", "modify", "+done", "end:today"],
            concat!(
                "DEL add task due:2002-03-04T05:06:07 +update\n",
                "ADD x 2001-02-03 add task due:2002-03-04T05:06:07 +update\n",
            ),
            false,
            concat!(
                "(M) 2001-02-03 @home +chore add tests\n",
                "x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
                "x 2001-02-03 add task due:2002-03-04T05:06:07 +update\n",
            ),
        ),
        (
            concat!(
                "(M) 2001-02-03 @home +chore add tests +update\n",
                "add task due:2002-03-04T05:06:07\n",
                "x 2001-01-01 (M) 2001-01-01 @home +chore add tests +update\n",
                "x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
            ),
            vec!["-+done", "+update", "modify", "+done", "end:today"],
            concat!(
                "DEL (M) 2001-02-03 @home +chore add tests +update\n",
                "DEL x 2001-01-01 (M) 2001-01-01 @home +chore add tests +update\n",
                "ADD x 2001-02-03 (M) 2001-02-03 @home +chore add tests +update\n",
            ),
            false,
            concat!(
                "add task due:2002-03-04T05:06:07\n",
                "x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
                "x 2001-02-03 (M) 2001-02-03 @home +chore add tests +update\n",
            ),
        ),
        (
            concat!(
                "(M) 2001-02-03 @home +chore add tests\n",
                "add task due:2002-03-04T05:06:07 +update\n",
                "x 2001-01-01 add task due:2002-03-04T05:06:07 +update\n",
                "x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
            ),
            vec!["-+done", "+update", "modify", "+done", "end:today"],
            concat!(
                "DEL add task due:2002-03-04T05:06:07 +update\n",
                "DEL x 2001-01-01 add task due:2002-03-04T05:06:07 +update\n",
                "ADD x 2001-02-03 add task due:2002-03-04T05:06:07 +update\n",
            ),
            false,
            concat!(
                "(M) 2001-02-03 @home +chore add tests\n",
                "x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
                "x 2001-02-03 add task due:2002-03-04T05:06:07 +update\n",
            ),
        ),
        (
            concat!(
                "(M) 2001-02-03 @home +chore add tests\n",
                "add task due:2002-03-04T05:06:07 +update\n",
                "x 2001-01-01 add task due:2003 +update\n",
                "x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
            ),
            vec!["-+done", "+update", "modify", "+done", "end:today"],
            concat!(
                "DEL add task due:2002-03-04T05:06:07 +update\n",
                "DEL x 2001-01-01 add task due:2003 +update\n",
                "ADD x 2001-02-03 add task due:2002-03-04T05:06:07 +update\n",
            ),
            false,
            concat!(
                "(M) 2001-02-03 @home +chore add tests\n",
                "x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
                "x 2001-02-03 add task due:2002-03-04T05:06:07 +update\n",
            ),
        ),
        (
            concat!(
                "(M) 2001-02-03 @home +chore add tests\n",
                "add task due:2002-03-04T05:06:07\n",
                "x 2001-01-01 (H) 2001-01-02 @work issue:123 +update\n",
            ),
            vec!["+update", "modify", "+done", "end:today"],
            concat!(
                "DEL x 2001-01-01 (H) 2001-01-02 @work issue:123 +update\n",
                "ADD x 2001-02-03 (H) 2001-01-02 @work issue:123 +update\n",
            ),
            false,
            concat!(
                "(M) 2001-02-03 @home +chore add tests\n",
                "add task due:2002-03-04T05:06:07\n",
                "x 2001-02-03 (H) 2001-01-02 @work issue:123 +update\n",
            ),
        ),
        // recur: and +update
        (
            concat!(
                "(M) 2001-02-03 @home +chore recur:1w +update add tests\n",
                "add task due:2002-03-04T05:06:07\n",
                "x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
            ),
            vec!["recur.any:", "modify", "+done", "end:today"],
            concat!(
                "DEL (M) 2001-02-03 @home +chore recur:1w +update add tests\n",
                "ADD (M) 2001-02-10 @home +chore recur:1w +update add tests\n",
                "ADD x 2001-02-03 (M) 2001-02-03 @home +chore recur:1w +update add tests\n",
            ),
            false,
            concat!(
                "(M) 2001-02-10 @home +chore recur:1w +update add tests\n",
                "add task due:2002-03-04T05:06:07\n",
                "x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
                "x 2001-02-03 (M) 2001-02-03 @home +chore recur:1w +update add tests\n",
            ),
        ),
        (
            concat!(
                "(M) 2001-02-03 @home +chore recur:1w +update add tests\n",
                "add task due:2002-03-04T05:06:07\n",
                "x 2001-01-01 (M) 2001-02-01 @home +chore recur:1w +update add tests\n",
                "x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
            ),
            vec!["-+done", "recur.any:", "modify", "+done", "end:today"],
            concat!(
                "DEL (M) 2001-02-03 @home +chore recur:1w +update add tests\n",
                "ADD (M) 2001-02-10 @home +chore recur:1w +update add tests\n",
                "DEL x 2001-01-01 (M) 2001-02-01 @home +chore recur:1w +update add tests\n",
                "ADD x 2001-02-03 (M) 2001-02-03 @home +chore recur:1w +update add tests\n",
            ),
            false,
            concat!(
                "(M) 2001-02-10 @home +chore recur:1w +update add tests\n",
                "add task due:2002-03-04T05:06:07\n",
                "x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
                "x 2001-02-03 (M) 2001-02-03 @home +chore recur:1w +update add tests\n",
            ),
        ),
    ] {
        let config = Config {
            now: chrono::NaiveDate::from_ymd(2001, 2, 3).and_hms(4, 5, 6),
            args: args.iter().map(|s| s.to_string()).collect(),
            tasks: Some(tasks.to_string()),
            date_keys: Some("due:\nscheduled:\nwait:\nuntil:\n".to_owned()),
            ..Default::default()
        };

        let mut expect_undo = String::from("---\n");
        expect_undo.push_str(expect_stdout);

        match chore::run(config)? {
            Output::WriteFiles {
                stdout,
                confirm,
                tasks,
                undo,
            } => {
                assert_eq!(&stdout, expect_stdout);
                assert_eq!(&confirm, expect_confirm);
                assert_eq!(&tasks, expect_tasks);
                assert_eq!(undo, expect_undo);
            }
            _ => panic!("expected WriteFiles"),
        }
    }
    Ok(())
}

#[test]
fn invalid() -> Result<()> {
    for (args, expect) in &[
        (
            vec!["modify", "end:2001-02-03T04:05:06"],
            InvalidEnd("end:2001-02-03T04:05:06".to_owned()),
        ),
        (
            vec!["modify", "end:x"],
            KeyExpectsDateValue("end:x".to_owned()),
        ),
        (vec!["modify", "pri:x"], InvalidPriority("pri:x".to_owned())),
        (
            vec!["modify", "entry:2001-02-03T04:05:06"],
            InvalidEntry("entry:2001-02-03T04:05:06".to_owned()),
        ),
        (
            vec!["modify", "entry:x"],
            KeyExpectsDateValue("entry:x".to_owned()),
        ),
        (
            vec!["modify", "due:x"],
            KeyExpectsDateValue("due:x".to_owned()),
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
