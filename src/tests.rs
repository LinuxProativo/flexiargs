use super::*;
use std::collections::VecDeque;
use std::path::PathBuf;

#[test]
fn test_setup_parsing_success() -> Result<(), Box<dyn std::error::Error>> {
    let mut edge = false;
    let mut minimal = false;
    let mut mirror_url = String::new();
    let mut rootfs = PathBuf::new();

    let input_args = vec![
        "--edge".to_string(),
        "--minimal".to_string(),
        "--mirror".to_string(),
        "https://alpine.mirror.com".to_string(),
        "--rootfs=/mnt/alpine_root".to_string(),
    ];

    {
        let mut rules = [
            Arg::bool(None, "--edge", &mut edge),
            Arg::bool(Some("-m"), "--minimal", &mut minimal),
            Arg::value(None, "--mirror", "url", &mut mirror_url),
            Arg::value(Some("-R"), "--rootfs", "directory", &mut rootfs),
        ];

        let help_rules = [];
        let deque: VecDeque<String> = input_args.into();
        parse_into_vars("setup", &mut rules, &help_rules, deque).ok()?;
    }

    assert!(edge);
    assert!(minimal);
    assert_eq!(mirror_url, "https://alpine.mirror.com");
    assert_eq!(rootfs, PathBuf::from("/mnt/alpine_root"));
    Ok(())
}
