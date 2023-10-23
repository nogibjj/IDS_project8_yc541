use polars::prelude::*;
use std::error::Error as StdError;
use std::result::Result as StdResult;

fn main() -> StdResult<(), Box<dyn StdError>> {
    let file_path = "spotify-2023.csv";

    let mut df = CsvReader::from_path(file_path)?
        .infer_schema(None)
        .has_header(true)
        .finish()?;

    // 打印数据集的前十行
    println!("=== Dataset Overview ===");
    println!("{:?}", df.head(Some(10)));

    // 对于streams列，先检查它是否存在，然后尝试转换和过滤操作
    if let Ok(streams_column) = df.column("streams") {
        let streams_column = streams_column.clone();
        let boolean_array = streams_column.is_not_null();
        df.filter(&boolean_array)?;
        if let Ok(streams_as_uint) = streams_column.cast::<UInt32Type>() {
            df.with_column(streams_as_uint)?;
        }
    }

    // 获取前10首歌曲
    let sorted_df = df.sort("streams", false)?;
    let top_10_songs = sorted_df.head(Some(10));

    
    println!("=== Top 10 Songs by Streams ===");
    println!("{:?}", top_10_songs.select(vec!["track_name", "artist(s)_name", "streams"]));

    Ok(())
}
