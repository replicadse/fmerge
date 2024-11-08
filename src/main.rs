use {
    anyhow::Result,
    args::ManualFormat,
    std::path::{
        Path,
        PathBuf,
    },
};

pub mod args;
pub mod reference;

#[tokio::main]
async fn main() -> Result<()> {
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
        | crate::args::Command::Merge { file, regex } => {
            merge(std::fs::canonicalize(file)?.as_path(), &regex).await?;
            Ok(())
        },
    }
}

async fn merge(path: &Path, regex: &str) -> Result<()> {
    let regex = fancy_regex::Regex::new(regex)?;
    print!("{}", resolve_file(path, &regex)?);
    Ok(())
}

fn resolve_file(path: &Path, regex: &fancy_regex::Regex) -> Result<String> {
    let mut content = std::fs::read_to_string(path)?;
    let rel_dir = path.parent().unwrap();

    let mut replacements = Vec::new();
    let captures = regex.captures_iter(&content);
    for cap in captures {
        let c = cap?;
        let file_path = c
            .get(1)
            .unwrap()
            .as_str()
            .replace("/", &std::path::MAIN_SEPARATOR.to_string());
        let indentation = match c.get(2) {
            | Some(v) => v.as_str().parse::<usize>()?,
            | None => 0_usize,
        };

        let file_content = resolve_file(&rel_dir.join(file_path), regex)?;
        let mut lines = file_content.lines();
        let indented_content: String = lines
            .next()
            .map(|first_line| {
                let mut result = first_line.to_string();
                for line in lines {
                    result.push('\n');
                    result.push_str(&" ".repeat(indentation));
                    result.push_str(line);
                }
                result
            })
            .unwrap_or_default();

        let start = c.get(0).unwrap().start();
        let end = c.get(0).unwrap().end();
        replacements.push((start, end, indented_content));
    }
    for (start, end, replacement) in replacements.into_iter().rev() {
        content.replace_range(start..end, &replacement);
    }

    Ok(content)
}

#[cfg(test)]
mod tests {
    use {
        anyhow::Result,
        std::process::Command,
    };

    fn exec(command: &str) -> Result<String> {
        let output = Command::new("sh").arg("-c").arg(command).output()?;
        if output.status.code().unwrap() != 0 {
            return Err(anyhow::anyhow!(String::from_utf8(output.stderr).unwrap()));
        }
        Ok(String::from_utf8(output.stdout)?)
    }

    #[test]
    fn test_merge_mds() {
        let res = exec(r#"cargo run -- m -f=./test/md/a.md"#).unwrap();
        assert_eq!("# A\n# B\n# C\n# C", res)
    }

    #[test]
    fn test_merge_yamls() {
        let res = exec(r##"cargo run -- m -f=./test/yaml/a.yaml.part"##).unwrap();

        assert_eq!(
            r#"root:
  childA:
    test: ok
  childB:
    test: ok
  childC:
    test: ok
"#,
            res
        )
    }
}
