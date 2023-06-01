use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    sync::{Arc, Mutex},
    time::Instant,
};

fn count_words_sync(file_path: &str) -> HashMap<String, u32> {
    let file = File::open(file_path).unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();

    let mut word_counts = HashMap::new();
    while reader.read_line(&mut line).unwrap() != 0 {
        for word in line.split_whitespace() {
            *word_counts.entry(word.to_string()).or_insert(0) += 1;
        }
        line.clear();
    }

    word_counts
}

fn count_words_threaded(file_path: &str) -> HashMap<String, u32> {
    let file = File::open(file_path).unwrap();
    let reader = Arc::new(Mutex::new(BufReader::new(file)));

    let word_counts = Arc::new(Mutex::new(HashMap::new()));
    let mut threads = Vec::new();

    // get the line count in the file

    // set threading to 4
    const NUM_THREADS: usize = 5;
    // for each thread, get the start and end line and spawn a thread to count the words

    // first thread will do all the work and then the rest will do nothing
    for _ in 0..NUM_THREADS {
        let word_counts = word_counts.clone();
        let reader = reader.clone();

        let thread = std::thread::spawn(move || {
            let mut line = String::new();

            loop {
                line.clear();
                reader.lock().unwrap().read_line(&mut line).unwrap();

                if line.is_empty() {
                    break;
                }

                let mut word_counts = word_counts.lock().unwrap();

                for word in line.split_whitespace() {
                    *word_counts.entry(word.to_string()).or_insert(0) += 1;
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

    assert_eq!(word_counts_sync.len(), word_counts_threaded.len());

    println!("Synchronous: {:?}", duration_sync);
    println!("Threaded: {:?}", duration_threaded);
}
