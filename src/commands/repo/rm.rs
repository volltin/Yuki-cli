use clap::{App, Arg, ArgMatches, SubCommand};
use commands::{Commander, RespMsg};
use context::Context;
use reqwest::Response;

pub(crate) struct RepoRm;

impl Commander for RepoRm {
    fn build() -> App<'static, 'static> {
        SubCommand::with_name("rm")
            .about("Remove one or more repositories")
            .arg(
                Arg::with_name("NAMES")
                    .multiple(true)
                    .required(true)
                    .help("Repository names to remove"),
            )
    }

    fn exec(ctx: &Context, args: &ArgMatches) -> ::Result<()> {
        let remote = ctx.remote.join("repositories/")?;
        let names: Vec<_> = args.values_of("NAMES").unwrap_or_default().collect();

        let result = names.iter().map(|name| -> ::Result<Response> {
            let remote = remote.join(name)?;
            Ok(ctx.delete(remote).send()?)
        });

        for (name, rr) in names.iter().zip(result) {
            match rr {
                Ok(mut r) => if !r.status().is_success() {
                    match r.json::<RespMsg>() {
                        Ok(msg) => println!("{} error: {}", name, msg.message),
                        Err(e) => println!("{} error: {}", name, e),
                    }
                } else {
                    println!("{}", name);
                },
                Err(e) => println!("{} error: {}", name, e),
            }
        }

        Ok(())
    }
}
