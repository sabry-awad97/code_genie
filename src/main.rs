use generator::{CodeGenerator, Generator};
use sql_formatter::SqlFormatter;

mod generator;
mod model;
mod openai;
mod sql_formatter;

struct SqlGenerator;

impl CodeGenerator for SqlGenerator {
    fn name() -> &'static str {
        "Sql"
    }

    fn format(&self, sql_code: &str) -> String {
        SqlFormatter::format(sql_code)
    }
}

#[tokio::main]
async fn main() -> Result<(), String> {
    let mut sql_generator = Generator::new(SqlGenerator)?;
    sql_generator.run().await?;
    Ok(())
}
