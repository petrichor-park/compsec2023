use std::sync::atomic::{AtomicU64, Ordering};

use ahash::AHashMap;
use itertools::Itertools;
use rayon::prelude::*;

/// yes, this is the only way to get that information to the ctrl-c
/// handler i think
static HASHES_COMPUTED: AtomicU64 = AtomicU64::new(0);

fn print_hashes() {
  eprintln!(
    "\nExiting! Hashes computed : {}",
    HASHES_COMPUTED.load(Ordering::SeqCst)
  );
}

fn main() {
  ctrlc::set_handler(move || {
    print_hashes();
    std::process::exit(0)
  })
  .unwrap();

  let data = DataStuff::new();
  let args = std::env::args().collect_vec();

  match args[1].as_str() {
    "part1" => data.part1(),
    "part2" => data.part2(),
    "part3" => data.part3(),
    _ => {}
  }

  print_hashes();
}

struct DataStuff {
  words: Vec<String>,
}

impl DataStuff {
  fn new() -> Self {
    let words = std::fs::read_to_string("data/words.txt")
      .unwrap()
      .lines()
      .map(str::to_ascii_lowercase)
      .collect();
    Self { words }
  }

  fn part1(&self) {
    let hashhashmap = read_passwords("data/passwords1.txt");
    // `par_iter` is the secret sauce;
    // everything here is automatically parallelized
    let output: Vec<_> = self
      .words
      .par_iter()
      .filter_map(|word| {
        // I do not actually know what to do for atomics,
        // but given i want this to be fast and don't care if I miss, relaxed??
        HASHES_COMPUTED.fetch_add(1, Ordering::Relaxed);

        let hash = sha256::digest(word);
        let vs = hashhashmap.get(&hash)?;
        Some((word.clone(), vs.clone()))
      })
      .collect();

    for (word, users) in output {
      for user in users {
        println!("{}:{}", user, word);
      }
    }
  }

  fn part2(&self) {
    let hashhashmap = read_passwords("data/passwords2.txt");

    let words = &self.words;
    // let words = &self.words[..1000];

    words
      .par_iter()
      .flat_map(|word1| {
        words.par_iter().map(|word2| {
          let v: String = [word1.as_str(), word2.as_str()].concat();
          v
        })
      })
      // ^^ this and above iterates over each pair of words, concated.
      // rayon does not have a cartesian product iterator so I had to do
      // it myself
      .filter_map(|pair| {
        HASHES_COMPUTED.fetch_add(1, Ordering::Relaxed);

        let hash = sha256::digest(&pair);
        let vs = hashhashmap.get(&hash)?;
        Some((pair.clone(), vs.clone()))
      })
      .for_each(|(pw, names)| {
        for name in names {
          println!("{}:{}", name, pw);
        }
      });
  }

  fn part3(&self) {
    // let test_words = vec!["marmot".to_string()];

    let hash_salts = {
      let map = read_passwords("data/passwords3.txt");
      map
        .into_iter()
        .map(|(hash_salt, usernames)| {
          // Not sure what the 5 means. But the split returns like:
          // "", 5, salt, hash
          // so, idxes 2 and 3
          let splitted = hash_salt.split('$').collect_vec();
          ((splitted[2].to_owned(), splitted[3].to_owned()), usernames)
        })
        .collect_vec()
    };
    hash_salts
      .par_iter()
      .filter_map(|((salt, hash), users)| {
        let words = self.words.par_iter();
        // let words = test_words.par_iter();
        let word = words.find_map_any(|word| {
          HASHES_COMPUTED.fetch_add(1, Ordering::Relaxed);

          let this_hash = sha256::digest(format!("{salt}{word}"));
          (&this_hash == hash).then_some(word)
        })?;
        Some((word, users))
      })
      .for_each(|(word, users)| {
        for user in users {
          println!("{user}:{word}");
        }
      });
  }
}

fn read_passwords(path: &str) -> AHashMap<String, Vec<String>> {
  let mut map = AHashMap::<String, Vec<String>>::new();
  for line in std::fs::read_to_string(path).unwrap().lines() {
    let splitted = line.split(":").collect_vec();
    // hash, username
    let hash = splitted[1].to_owned();
    let username = splitted[0].to_owned();
    map.entry(hash).or_default().push(username);
  }
  map
}
