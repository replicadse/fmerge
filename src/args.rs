use {
    anyhow::Result,
    clap::Arg,
    std::str::FromStr,
};

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum Privilege {
    Normal,
    Experimental,
}

#[derive(Debug)]
pub(crate) struct CallArgs {
    pub privileges: Privilege,
    pub command: Command,
}

impl CallArgs {
    pub fn validate(&self) -> Result<()> {
        if self.privileges == Privilege::Experimental {
            return Ok(());
        }

        match &self.command {
            | _ => Ok(()),
        }
    }
}

#[derive(Debug)]
pub(crate) enum ManualFormat {
    Manpages,
    Markdown,
}

#[derive(Debug)]
pub(crate) enum Command {
    Manual { path: String, format: ManualFormat },
    Autocomplete { path: String, shell: clap_complete::Shell },

    Merge { file: String, regex: String },
}

pub(crate) struct ClapArgumentLoader {}

impl ClapArgumentLoader {
    pub fn root_command() -> clap::Command {
        clap::Command::new("fmerge")
            .version(env!("CARGO_PKG_VERSION"))
            .about("A rusty text templating application for CLIs.")
            .author("replicadse <aw@voidpointergroup.com>")
            .propagate_version(true)
            .subcommand_required(true)
            .args([Arg::new("experimental")
                .short('e')
                .long("experimental")
                .help("enables experimental features")
                .num_args(0)])
            .subcommand(
                clap::Command::new("man")
                    .about("Renders the manual.")
                    .arg(clap::Arg::new("out").short('o').long("out").required(true))
                    .arg(
                        clap::Arg::new("format")
                            .short('f')
                            .long("format")
                            .value_parser(["manpages", "markdown"])
                            .required(true),
                    ),
            )
            .subcommand(
                clap::Command::new("autocomplete")
                    .about("Renders shell completion scripts.")
                    .arg(clap::Arg::new("out").short('o').long("out").required(true))
                    .arg(
                        clap::Arg::new("shell")
                            .short('s')
                            .long("shell")
                            .value_parser(["bash", "zsh", "fish", "elvish", "powershell"])
                            .required(true),
                    ),
            )
            .subcommand(
                clap::Command::new("merge")
                    .alias("m")
                    .about("Merge.")
                    .arg(clap::Arg::new("file").short('f').long("file").required(true))
                    .arg(
                        clap::Arg::new("regex")
                            .short('r')
                            .long("regex")
                            .default_value(r#"\{\{\s*([\w./]+)\s*\+?(\d+)?\s*\}\}"#)
                            .required(false),
                    ),
            )
    }

    pub fn load() -> Result<CallArgs> {
        let command = Self::root_command().get_matches();

        let privileges = if command.get_flag("experimental") {
            Privilege::Experimental
        } else {
            Privilege::Normal
        };

        let cmd = if let Some(subc) = command.subcommand_matches("man") {
            Command::Manual {
                path: subc.get_one::<String>("out").unwrap().into(),
                format: match subc.get_one::<String>("format").unwrap().as_str() {
                    | "manpages" => ManualFormat::Manpages,
                    | "markdown" => ManualFormat::Markdown,
                    | _ => return Err(anyhow::anyhow!("unknown format")),
                },
            }
        } else if let Some(subc) = command.subcommand_matches("autocomplete") {
            Command::Autocomplete {
                path: subc.get_one::<String>("out").unwrap().into(),
                shell: clap_complete::Shell::from_str(subc.get_one::<String>("shell").unwrap().as_str()).unwrap(),
            }
        } else if let Some(subc) = command.subcommand_matches("merge") {
            Command::Merge {
                file: subc.get_one::<String>("file").unwrap().into(),
                regex: subc.get_one::<String>("regex").unwrap().into(),
            }
        } else {
            return Err(anyhow::anyhow!("unknown command"));
        };

        let callargs = CallArgs {
            privileges,
            command: cmd,
        };

        callargs.validate()?;
        Ok(callargs)
    }
}
