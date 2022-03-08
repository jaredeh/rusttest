// testing polars dataframe

use polars::prelude::*;
use polars::df;

pub fn ptest() -> Result<()> {
    let _df = df! [
        "names" => ["a", "b", "c"],
        "values" => [1, 2, 3],
        "values_nulls" => [Some(1), None, Some(3)]
    ]?;

    let s1 = Series::new("names", &["a", "b", "c"]);
    let s2 = Series::new("values", &[Some(1), None, Some(3)]);
    let df3 = DataFrame::new(vec![s1, s2])?;
    Ok(())
}