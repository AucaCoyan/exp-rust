use polars::prelude::*;

fn main() {
    let df = CsvReader::from_path("./survey_results_public.csv")
        .unwrap()
        .finish()
        .unwrap();

    let dato = df
        .clone()
        .select(&col("DatabaseHaveWorkedWith").n_unique())
        .sort(false)
        .head(Some(10));

    println!("{}", dato);
}
