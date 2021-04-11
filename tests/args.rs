use chore::*;

#[test]
fn general() -> Result<()> {
    for (config, expect) in &[
        (
            Config {
                now: chrono::NaiveDate::from_ymd(2001, 2, 3).and_hms(4, 5, 6),
                args: vec!["completed".to_owned(), "  ".to_owned()],
                tasks: Some(
                    concat!(
                        "(M) 2001-02-03 @home +chore add tests\n",
                        "add task due:2002-03-04T05:06:07\n",
                        "x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
                    )
                    .to_string(),
                ),
                filter_aliases: vec![File {
                    name: "completed".to_string(),
                    content: "+done".to_string(),
                }],
                ..Default::default()
            },
            Output::JustPrint {
                stdout: concat!("3 x 2001-02-03 (H) 2001-01-02 @work issue:123\n",).to_string(),
            },
        ),
        (
            Config {
                now: chrono::NaiveDate::from_ymd(2001, 2, 3).and_hms(4, 5, 6),
                args: vec!["pri:M".to_owned(), "done".to_owned()],
                tasks: Some(
                    concat!(
                        "(M) 2001-02-03 @home +chore add tests\n",
                        "add task due:2002-03-04T05:06:07\n",
                        "x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
                    )
                    .to_string(),
                ),
                command_aliases: vec![File {
                    name: "done".to_string(),
                    content: "modify +done end:today".to_string(),
                }],
                ..Default::default()
            },
            Output::WriteFiles {
                stdout: concat!(
                    "DEL (M) 2001-02-03 @home +chore add tests\n",
                    "ADD x 2001-02-03 (M) 2001-02-03 @home +chore add tests\n",
                )
                .to_string(),
                confirm: false,
                tasks: concat!(
                    "add task due:2002-03-04T05:06:07\n",
                    "x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
                    "x 2001-02-03 (M) 2001-02-03 @home +chore add tests\n",
                )
                .to_string(),
                undo: concat!(
                    "---\n",
                    "DEL (M) 2001-02-03 @home +chore add tests\n",
                    "ADD x 2001-02-03 (M) 2001-02-03 @home +chore add tests\n",
                )
                .to_string(),
            },
        ),
        (
            Config {
                now: chrono::NaiveDate::from_ymd(2001, 2, 3).and_hms(4, 5, 6),
                args: vec!["+done".to_owned(), "modify".to_owned(), "reopen".to_owned()],
                tasks: Some(
                    concat!(
                        "(M) 2001-02-03 @home +chore add tests\n",
                        "add task due:2002-03-04T05:06:07\n",
                        "x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
                    )
                    .to_string(),
                ),
                modification_aliases: vec![File {
                    name: "reopen".to_string(),
                    content: "-+done -end:".to_string(),
                }],
                ..Default::default()
            },
            Output::WriteFiles {
                stdout: concat!(
                    "DEL x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
                    "ADD (H) 2001-01-02 @work issue:123\n",
                )
                .to_string(),
                confirm: false,
                tasks: concat!(
                    "(H) 2001-01-02 @work issue:123\n",
                    "(M) 2001-02-03 @home +chore add tests\n",
                    "add task due:2002-03-04T05:06:07\n",
                )
                .to_string(),
                undo: concat!(
                    "---\n",
                    "DEL x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
                    "ADD (H) 2001-01-02 @work issue:123\n",
                )
                .to_string(),
            },
        ),
        (
            Config {
                now: chrono::NaiveDate::from_ymd(2001, 2, 3).and_hms(4, 5, 6),
                args: vec![],
                tasks: Some(
                    concat!(
                        "(M) 2001-02-03 @home +chore add tests\n",
                        "add task due:2002-03-04T05:06:07\n",
                        "x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
                    )
                    .to_string(),
                ),
                default_filters: vec![File {
                    name: "pending".to_string(),
                    content: "-+done".to_string(),
                }],
                ..Default::default()
            },
            Output::JustPrint {
                stdout: concat!(
                    "1 (M) 2001-02-03 @home +chore add tests\n",
                    "2 add task due:2002-03-04T05:06:07\n",
                )
                .to_string(),
            },
        ),
        (
            Config {
                now: chrono::NaiveDate::from_ymd(2001, 2, 3).and_hms(4, 5, 6),
                args: vec![
                    " ".to_owned(),
                    "-pri:M".to_owned(),
                    "  ".to_owned(),
                    "-+done".to_owned(),
                    "  ".to_owned(),
                ],
                tasks: Some(
                    concat!(
                        "(M) 2001-02-03 @home +chore add tests\n",
                        "add task due:2002-03-04T05:06:07\n",
                        "x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
                    )
                    .to_string(),
                ),
                ..Default::default()
            },
            Output::JustPrint {
                stdout: concat!("2 add task due:2002-03-04T05:06:07\n",).to_string(),
            },
        ),
        (
            Config {
                now: chrono::NaiveDate::from_ymd(2001, 2, 3).and_hms(4, 5, 6),
                args: vec![
                    "/add tests/".to_owned(),
                    "modify".to_owned(),
                    "pri:Z +done".to_owned(),
                ],
                tasks: Some(
                    concat!(
                        "(M) 2001-02-03 @home +chore add tests\n",
                        "add task due:2002-03-04T05:06:07\n",
                        "x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
                    )
                    .to_string(),
                ),
                command_aliases: vec![File {
                    name: "done".to_string(),
                    content: "modify +done end:today".to_string(),
                }],
                ..Default::default()
            },
            Output::WriteFiles {
                stdout: concat!(
                    "DEL (M) 2001-02-03 @home +chore add tests\n",
                    "ADD x (Z) 2001-02-03 @home +chore add tests\n",
                )
                .to_string(),
                confirm: false,
                tasks: concat!(
                    "add task due:2002-03-04T05:06:07\n",
                    "x (Z) 2001-02-03 @home +chore add tests\n",
                    "x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
                )
                .to_string(),
                undo: concat!(
                    "---\n",
                    "DEL (M) 2001-02-03 @home +chore add tests\n",
                    "ADD x (Z) 2001-02-03 @home +chore add tests\n",
                )
                .to_string(),
            },
        ),
    ] {
        let actual = chore::run(config.clone())?;
        assert_eq!(actual, *expect);
    }

    Ok(())
}
