use ahash::AHashMap;
use itertools::Itertools;
use rayon::prelude::*;

fn main() {
  let data = DataStuff::new();
  let args = std::env::args().collect_vec();

  match args[1].as_str() {
    "part1" => data.part1(),
    "part2" => data.part2(),
    _ => {}
  }
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
      .filter_map(|pair| {
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
