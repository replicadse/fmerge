include!("check_features.rs");

use std::path::{Path, PathBuf};
use std::result::Result;

use args::ManualFormat;
use error::Error;

pub mod args;
pub mod error;
pub mod reference;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cmd = crate::args::ClapArgumentLoader::load()?;
    cmd.validate()?;

    match cmd.command {
        | crate::args::Command::Manual { path, format } => {
            let out_path = PathBuf::from(path);
            std::fs::create_dir_all(&out_path)?;
            match format {
                | ManualFormat::Manpages => {
                    reference::build_manpages(&out_path)?;
                },
                | ManualFormat::Markdown => {
                    reference::build_markdown(&out_path)?;
                },
            }
            Ok(())
        },
        | crate::args::Command::Autocomplete { path, shell } => {
            let out_path = PathBuf::from(path);
            std::fs::create_dir_all(&out_path)?;
            reference::build_shell_completion(&out_path, &shell)?;
            Ok(())
        },
        | crate::args::Command::Merge { file, pattern } => {
            merge(std::fs::canonicalize(file)?.as_path(), &pattern).await?;
            Ok(())
        },
    }
}

async fn merge(path: &Path, pattern: &str) -> Result<(), Error> {
    let pat_s = pattern.splitn(2, "%f").collect::<Vec<_>>();
    let regex = fancy_regex::Regex::new(&format!(
        "{}(.*?){}",
        fancy_regex::escape(&pat_s[0]),
        fancy_regex::escape(&pat_s[1])
    ))
    .or(Err(error::Error::Generic("failed to parse regex".to_owned())))?;
    let strip = |v: &str| -> String { return v.trim_start_matches(pat_s[0]).trim_end_matches(pat_s[1]).to_owned() };

    print!("{}", resolve_file(path, &regex, &strip)?);
    Ok(())
}

fn resolve_file(path: &Path, pattern: &fancy_regex::Regex, strip: &impl Fn(&str) -> String) -> Result<String, Error> {
    let mut content = std::fs::read_to_string(path).or(Err(Error::Generic(format!(
        "can not read file {}",
        path.to_str().unwrap()
    ))))?;
    let rel_dir = path.parent().unwrap();

    for m in pattern.find_iter(&content.clone()) {
        let mat = m.or(Err(error::Error::Generic("match error".to_owned())))?.as_str();

        let filename_arg = strip(mat).replace("/", &std::path::MAIN_SEPARATOR.to_string());
        let subf_content = resolve_file(&rel_dir.join(filename_arg), pattern, strip)?;

        content = content.replace(mat, &subf_content);
    }
    Ok(content)
}

#[cfg(test)]
mod tests {
    use std::{error::Error, process::Command};

    fn exec(command: &str) -> Result<String, Box<dyn Error>> {
        let output = Command::new("sh").arg("-c").arg(command).output()?;
        if output.status.code().unwrap() != 0 {
            return Err(Box::new(crate::error::Error::Generic(
                String::from_utf8(output.stderr).unwrap(),
            )));
        }
        Ok(String::from_utf8(output.stdout)?)
    }

    #[test]
    fn test_merge_mds() {
        assert!("# A\n# B\n# C\n# C" == exec(r#"cargo run -- m -f=./test/md/a.md -p="{{ _include \"%f\" }}""#).unwrap())
    }

    #[test]
    fn test_merge_yamls() {
        assert!(
            r#"root:
  childA:
    test: ok
  childB:
    test: ok
  childC:
    test: ok
"# == exec(r##"cargo run -- m -f=./test/yaml/a.yaml -p="#include %f!""##).unwrap()
        )
    }
}
