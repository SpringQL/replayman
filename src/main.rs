use chrono::DateTime;
use springql_foreign_service::source::source_input::{
    timed_stream::{file_type::FileType, TimedStream},
    ForeignSourceInput,
};

fn main() {
    let timed_stream = TimedStream::new(
        FileType::Tsv,
        "/Users/sho.nakatani/.ghq/src/github.com/SpringQL/dataset/pseudo-in-vehicle/AirConditioner-30sec.tsv",
        "Time".to_string(),
        DateTime::parse_from_rfc3339("2020-10-21T10:37:56.000+09:00").unwrap(),
    )
    .unwrap();

    let input = ForeignSourceInput::new_timed_stream(timed_stream);

    for j in input {
        println!("{}", j.unwrap());
    }
}
