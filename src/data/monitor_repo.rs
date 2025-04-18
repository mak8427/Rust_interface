use polars::prelude::*;

pub async fn load_monitors(path: &str) -> PolarsResult<DataFrame> {
    CsvReadOptions::default()
        .with_has_header(true)
        .try_into_reader_with_file_path(Some(path.into()))?
        .finish()
}
