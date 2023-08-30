use chrono::format::ParseError;
use chrono::{NaiveDateTime, TimeZone, Utc};
use std::env;

/** epoch - Small utility for dealing with conversions between human readable dates
 *  and Unix Timestamps.
 *
 * This program takes a single command line argument, which is expected
 * to either be a Unix Timestamp or a date/time string. It then either converts the 
 * timestamp to a human-readable format, or if the input is a date/time string, converts
 * it to a Unix Timestamp. If the input is not recognized, the program prints an error 
 * message and exits with a non-zero status code.
 */

enum InputKind {
    UnixTimestamp,
    HumanReadable
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: epoch <timestamp>\nOr:    epoch \"<month> <day> <year> <hour><minute>\" (all integer arguments, quotes required)");
        std::process::exit(1);
    }

    let input = &args[1];
    let parsed_date = parse_input(input);

    match parsed_date {
        Ok((epoch, InputKind::UnixTimestamp)) => {
            let human_readable = Utc
                .timestamp_opt(epoch, 0)
                .unwrap()
                .format("%B %e, %Y --> %H:%M Hours");
            println!("Human Readable: {}", human_readable);
        }
        Ok((epoch, InputKind::HumanReadable)) => {
            println!("Unix Timestamp: {}", epoch);
        }
        Err(_) => {
            eprintln!("Unrecognized Input");
            std::process::exit(1);
        }
    }
}

fn parse_input(input: &str) -> Result<(i64, InputKind), ParseError> {
    // Try to parse as UNIX timestamp
    if let Ok(epoch) = input.parse::<i64>() {
        return Ok((epoch, InputKind::UnixTimestamp));
    }

    // Try to parse as custom string format
    let format = "%m %d %Y %H%M";
    let parsed_date = NaiveDateTime::parse_from_str(input, format)?;
    let epoch = Utc.from_utc_datetime(&parsed_date).timestamp();
    Ok((epoch, InputKind::HumanReadable))
}
