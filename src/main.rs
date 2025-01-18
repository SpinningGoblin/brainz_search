extern crate core;

use crate::cli::{Cli, Commands};
use crate::entities::ReleaseGroup;
use clap::Parser;
use rusqlite::{params, Connection, Transaction};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::time::Instant;

mod cli;
mod entities;

const BATCH_SIZE: usize = 2_500;

fn main() -> io::Result<()> {
    let args = Cli::parse();

    let conn = Connection::open(&args.output).unwrap();

    println!("{}", args.command.sql_create());

    conn.execute(&args.command.sql_create(), []).unwrap();

    conn.execute_batch("PRAGMA journal_mode = WAL;").unwrap(); // Write-Ahead Logging for better concurrency
    conn.execute("PRAGMA synchronous = NORMAL;", []).unwrap(); // Faster writes with reasonable safety
    conn.execute("PRAGMA cache_size = -2000000;", []).unwrap(); // Bigger cache size
    conn.execute("PRAGMA temp_store = MEMORY;", []).unwrap(); // Store any temp tables in memory

    let file = File::open(&args.input)?;

    match &args.command {
        Commands::ReleaseGroup(args) => {
            process_release_groups(file, conn, args.release_group_types())
        },
    }
}

fn process_release_groups(file: File, mut conn: Connection, release_group_types: Vec<String>) -> io::Result<()> {
    let mut count = 0;
    let mut inserted = 0;

    let reader = BufReader::new(file);

    let now = Instant::now();
    println!("{:?}", now);

    let mut batch: Vec<ReleaseGroup> = Vec::new();

    for line in reader.lines() {
        count += 1;
        let parse_result = serde_json::from_str::<ReleaseGroup>(&line?);

        match parse_result {
            Ok(release_group) => {
                if let Some(primary_type) = &release_group.primary_type {
                    if !release_group_types.contains(primary_type) {
                        continue;
                    }
                }

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
            insert_batch(&batch, conn.transaction().unwrap());
            inserted += batch.len();
            batch.clear();
        }

        if count % 10_000 == 0 {
            println!("Processed {count} lines")
        }
    }

    if batch.len() > 0 {
        insert_batch(&batch, conn.transaction().unwrap());
        inserted += batch.len();
        batch.clear();
    }

    let elapsed = now.elapsed();
    println!(
        "{}s for parsing file with {count} total lines, {inserted} inserted",
        elapsed.as_secs()
    );

    conn.cache_flush().unwrap();
    conn.close().unwrap();

    Ok(())
}

pub fn insert_batch(batch: &Vec<ReleaseGroup>, tx: Transaction)  {
    for release_group in batch.iter() {
        let data = serde_json::to_string(&release_group).unwrap();
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
}
