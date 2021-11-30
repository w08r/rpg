mod pg;
mod reader;

extern crate clap;

use clap::{App, SubCommand};

fn main() {
    const NAME: &'static str = env!("CARGO_PKG_NAME");
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");
    const DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");
    let matches = App::new(NAME)
        .version(VERSION)
        .author(AUTHORS)
        .about(DESCRIPTION)
        .args_from_usage(
            "-c, --config=[FILE] 'Sets a custom config file'
             -i, --input=[FILE] 'Use [FILE] as input',
             -t, --type=[FILETYPE] 'Use [FILETYPE] as input file type (default: infer)'",
        )
        .subcommand(
            SubCommand::with_name("pg")
                .about("uploads to postgresql")
                .args_from_usage(
                    "-d, --dbname=[DBNAME]     'database name to connect to'
                     -h, --host=[HOSTNAME]     'database server host or socket directory (default: \"local socket\")'
                     -p, --port=[PORT]         'database server port (default: \"5432\")'
                     -U, --username=[USERNAME] 'database user name (default: \"postgres\")'
                     -w, --no-password         'never prompt for password'
                     -W, --password            'force password prompt (should happen automatically)'"
                )
        )
        .get_matches();

    let reader = reader::Reader::new(matches.value_of("input"), matches.value_of("type"));

    if let Some(pg) = matches.subcommand_matches("pg") {
        let pg_opts = pg::PostgresOpts {
            dbname: pg.value_of("dbname").unwrap_or("postgres"),
        };
        pg::upload_to_postgresql(reader, pg_opts)
    }
}

