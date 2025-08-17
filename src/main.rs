use fast_cards::cards::{isomorphisms::IsomorphismIterator, Street};
use std::env;
use std::fs::File;
use std::io::Write;

fn parse_street(arg: Option<String>) -> Street {
    match arg
        .unwrap_or_else(|| "pref".to_string())
        .to_lowercase()
        .chars()
        .next()
    {
        Some('p') => Street::Pref,
        Some('f') => Street::Flop,
        Some('t') => Street::Turn,
        Some('r') => Street::Rive,
        _ => Street::Pref,
    }
}

fn file_stem(street: Street) -> &'static str {
    match street {
        Street::Pref => "pref",
        Street::Flop => "flop",
        Street::Turn => "turn",
        Street::Rive => "river",
    }
}

fn save_isomorphisms(street: Street, limit: Option<u64>) -> std::io::Result<()> {
    let count = street.n_isomorphisms() as u64;
    let stem = file_stem(street);

    // Binary: [u64 count][i64 obs]...
    let mut bin = File::create(format!("isomorphisms_{}.bin", stem))?;
    let header_count = limit.map(|m| m.min(count)).unwrap_or(count);
    bin.write_all(&header_count.to_be_bytes())?;
    let mut written: u64 = 0;
    for iso in IsomorphismIterator::from(street) {
        let v: i64 = i64::from(iso);
        bin.write_all(&v.to_be_bytes())?;
        written += 1;
        if let Some(max) = limit { if written >= max { break; } }
        if written % 1_000_000 == 0 { println!("... {}M written", written / 1_000_000); }
    }

    // Text: one per line (Display format)
    if header_count <= 1_000_000 {
        let mut txt = File::create(format!("isomorphisms_{}.txt", stem))?;
        let mut written_txt: u64 = 0;
        for iso in IsomorphismIterator::from(street) {
            writeln!(txt, "{}", iso)?;
            written_txt += 1;
            if let Some(max) = limit { if written_txt >= max { break; } }
        }
    }
    Ok(())
}

fn main() {
    let street = parse_street(env::args().nth(1));
    let limit = env::args().nth(2).and_then(|s| s.parse::<u64>().ok());
    println!(
        "Deriving canonical isomorphisms for {} (expected: {})",
        street,
        street.n_isomorphisms()
    );
    match save_isomorphisms(street, limit) {
        Ok(()) => {
            let stem = file_stem(street);
            if street.n_isomorphisms() as u64 <= 1_000_000 && limit.unwrap_or(u64::MAX) <= 1_000_000 {
                println!("Saved files: isomorphisms_{}.bin and isomorphisms_{}.txt", stem, stem);
            } else {
                println!("Saved file: isomorphisms_{}.bin (text suppressed for large sets)", stem);
            }
        },
        Err(e) => eprintln!("Error: {}", e),
    }
}

