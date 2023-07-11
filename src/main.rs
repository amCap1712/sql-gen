use clap::{App, Arg, SubCommand};
use sqlx::{postgres::PgArguments, Executor, Postgres};

mod generate;
mod migrate;
mod models;
mod query_generate;
mod utils;

// async fn test_sqlx_query<'a, E: sqlx::Executor<'a, Database = Postgres>>(e: E) -> sqlx::Result<()> {
//     sqlx::query("SELECT * FROM Categories").fetch_all(e).await?;
//     Ok(())
// }

//   pub async fn insert<'e, E: Executor<'e, Database = Postgres>>(executor: E) -> Result<(), Error> {
//         sqlx::query("INSERT INTO Categories (id, created_at, label) VALUES ($1, $2, $3)")
//             .bind(1, 2, 3)
//             .execute(executor)
//             .await?;
//         Ok(())
//     }
// fn test_sqlx_query<'a>() -> sqlx::query::Query<'a, Postgres, PgArguments> {
//     let res = sqlx::query("SELECT * FROM Categories");
//     res
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("SQL Gen")
        .subcommand(
            SubCommand::with_name("generate")
                .about("Generate structs and queries for tables")
                .arg(
                    Arg::with_name("output")
                        .short('o')
                        .long("output")
                        .value_name("FOLDER")
                        .help("Sets the output folder")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("database")
                        .short('d')
                        .long("database")
                        .value_name("URL")
                        .help("Sets the database connection URL")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("context")
                        .short('c')
                        .long("context")
                        .value_name("Context name")
                        .help("The name of the context for calling functions. Defaults to DB name")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("migrate")
                .about("Generate SQL migrations based on struct differences")
                .arg(
                    Arg::with_name("include")
                        .short('i')
                        .long("include")
                        .value_name("FOLDER")
                        .help("Sets the folder containing existing struct files")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("output")
                        .short('o')
                        .long("output")
                        .value_name("FOLDER")
                        .help("Sets the output folder for migrations")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("database")
                        .short('d')
                        .long("database")
                        .value_name("URL")
                        .help("Sets the database connection URL")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("generate") {
        let output_folder = matches.value_of("output").unwrap();
        let context = matches.value_of("context");
        let database_url = matches.value_of("database").unwrap();
        generate::generate(output_folder, database_url, context).await?;
    } else if let Some(matches) = matches.subcommand_matches("migrate") {
        let include_folder = matches.value_of("include").unwrap();
        let output_folder = matches.value_of("output").unwrap();
        let database_url = matches.value_of("database").unwrap();
        migrate::migrate(include_folder, output_folder, database_url).await?;
    }
    Ok(())
}
