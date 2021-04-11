use chore::*;
use regex::Regex;

#[test]
fn general() -> Result<()> {
    let ansii_color = Regex::new("\x1b\\[[0-9;]*m").unwrap();

    for (args, tasks, expect) in &[
        (
        vec!["list"],
        concat!(
            "(M) 2001-02-03 @home +chore add tests\n",
            "(Z) foo | bar\n",
            "add task due:2002-03-04T05:06:07\n",
            "x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
        ),
        concat!(
            "1 (M) 2001-02-03 @home +chore add tests\n",
            "2 (Z) foo | bar\n",
            "3 add task due:2002-03-04T05:06:07\n",
            "4 x 2001-02-03 (H) 2001-01-02 @work issue:123\n",
        ),
    ),
        (
        vec!["modify", "pri:A"],
        concat!(
            "(M) 2001-02-03 @home +chore add tests\n",
        ),
        concat!(
            "DEL (M) 2001-02-03 @home +chore add tests\n",
            "ADD (A) 2001-02-03 @home +chore add tests\n",
        ),
    ),
        (
        vec!["delete"],
        concat!(
            "(M) 2001-02-03 @home +chore add tests\n",
        ),
        concat!(
            "DEL (M) 2001-02-03 @home +chore add tests\n",
        ),
    ),
    ] {
        let config = Config {
            now: chrono::NaiveDate::from_ymd(2001, 2, 3).and_hms(4, 5, 6),
            args: args.iter().map(|s| s.to_string()).collect(),
            tasks: Some(tasks.to_string()),
            date_keys: Some("due:\nscheduled:\nwait:\nuntil:\n".to_owned()),
            print_color: true,
            ..Default::default()
        };

        match chore::run(config)? {
            Output::JustPrint { stdout } => {
                let stdout = ansii_color.split(&stdout).collect::<Vec<_>>().join("");
                assert_eq!(&stdout, expect)
            }
            Output::WriteFiles { stdout, .. } => {
                let stdout = ansii_color.split(&stdout).collect::<Vec<_>>().join("");
                assert_eq!(&stdout, expect)
            }
        }
    }
    Ok(())
}
