extern crate core;

use crate::cli::Cli;
use crate::entities::ReleaseGroup;
use crate::entity_type::EntityType;
use clap::Parser;
use rusqlite::{params, Connection};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::time::Instant;

mod cli;
mod entities;
mod entity_type;

const BATCH_SIZE: usize = 2_500;

fn main() -> io::Result<()> {
    let args = Cli::parse();

    let conn = Connection::open(&args.output).unwrap();

    conn.execute(args.entity.sql_create(), []).unwrap();

    conn.execute_batch("PRAGMA journal_mode = WAL;").unwrap(); // Write-Ahead Logging for better concurrency
    conn.execute("PRAGMA synchronous = NORMAL;", []).unwrap(); // Faster writes with reasonable safety
    conn.execute("PRAGMA cache_size = -2000000;", []).unwrap(); // Bigger cache size
    conn.execute("PRAGMA temp_store = MEMORY;", []).unwrap(); // Store any temp tables in memory

    let file = File::open(&args.input)?;

    match &args.entity {
        &EntityType::ReleaseGroup => process_release_groups(file, conn),
    }
}

fn process_release_groups(file: File, mut conn: Connection) -> io::Result<()> {
    let mut count = 0;

    let reader = BufReader::new(file);

    let now = Instant::now();
    println!("{:?}", now);

    let mut batch: Vec<ReleaseGroup> = Vec::new();

    for line in reader.lines() {
        count += 1;
        let parse_result = serde_json::from_str::<ReleaseGroup>(&line?);

        match parse_result {
            Ok(release_group) => {
                if count % 100_000 == 0 {
                    println!("count {count} {}", &release_group.title);
                }

                batch.push(release_group);
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }

        if batch.len() >= BATCH_SIZE {
            let tx = conn.transaction().unwrap();
            for release_group in batch.iter() {
                let data = serde_json::to_string(&release_group)?;
                tx.execute(
                    "INSERT INTO release_groups (id, json, artists, title, genres) VALUES (?1, ?2, ?3, ?4, ?5);",
                    params![
                        release_group.id.clone(),
                        data,
                        release_group.artist_content().clone(),
                        release_group.title.clone(),
                        release_group.genres_content().clone()
                    ],
                )
                    .unwrap();
            }
            tx.commit().unwrap();

            batch.clear();
        }

        if count % 10_000 == 0 {
            println!("Processed {count} lines")
        }
    }

    let elapsed = now.elapsed();
    println!(
        "{}s for parsing file with {count} total lines",
        elapsed.as_secs()
    );

    conn.cache_flush().unwrap();
    conn.close().unwrap();

    Ok(())
}
