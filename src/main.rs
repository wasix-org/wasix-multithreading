use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::time::Instant;

fn count_words_sync(file_path: &str) -> HashMap<String, u32> {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    let mut word_counts = HashMap::new();
    for line in reader.lines() {
        let line = line.unwrap();
        for word in line.split_whitespace() {
            *word_counts.entry(word.to_string()).or_insert(0) += 1;
        }
    }

    word_counts
}

fn count_words_threaded(file_path: &str) -> HashMap<String, u32> {
    let file = File::open(file_path).unwrap();
    let reader = Arc::new(Mutex::new(BufReader::new(file)));

    let word_counts = std::sync::Arc::new(std::sync::Mutex::new(HashMap::new()));
    let mut threads = Vec::new();

    // get the line count in the file

    // set threading to 4
    const NUM_THREADS: usize = 5;
    // for each thread, get the start and end line and spawn a thread to count the words

    for thread_count in 1..NUM_THREADS {
        let word_counts = word_counts.clone();
        let reader = reader.clone();
        let thread = std::thread::spawn(move || {
            let mut reader = reader.lock().unwrap();
            for (i, line) in reader.by_ref().lines().enumerate() {
                if i % thread_count == 0 {
                    let line = line.unwrap();
                    for word in line.split_whitespace() {
                        *word_counts
                            .lock()
                            .unwrap()
                            .entry(word.to_string())
                            .or_insert(0) += 1;
                    }
                }
            }
        });
        threads.push(thread);
    }

    for thread in threads {
        thread.join().unwrap();
    }

    let word_counts = word_counts.lock().unwrap().clone();
    word_counts
}

fn main() {
    let file_path = std::env::args().nth(1).expect("no file path given");

    let start_sync = Instant::now();
    let word_counts_sync = count_words_sync(&file_path);
    let duration_sync = start_sync.elapsed();

    let start_threaded = Instant::now();
    let word_counts_threaded = count_words_threaded(&file_path);
    let duration_threaded = start_threaded.elapsed();

    // assert_eq!(word_counts_sync.len(), word_counts_threaded.len());

    println!("Synchronous: {:?}", duration_sync);
    println!("Threaded: {:?}", duration_threaded);
}
