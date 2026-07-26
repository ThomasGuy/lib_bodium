use rand::seq::SliceRandom;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;

use crate::data_types::KvError;

/// A generic dataset buffer used to parse, randomize, and seed key-value structures.
#[derive(Debug, Clone)]
pub struct Seeder<K, V> {
    pairs: Vec<(K, V)>,
}

impl<K, V> Seeder<K, V> {
    /// Creates an empty data seeder container
    pub fn new() -> Self {
        Self { pairs: Vec::new() }
    }

    /// Adds a single key-value record to the tracking buffer
    pub fn add(&mut self, key: K, value: V) {
        self.pairs.push((key, value));
    }

    /// Randomizes the sequence of collected items to prevent tree clustering/degradation.
    pub fn shuffle(&mut self) {
        let mut rng = rand::rng();
        self.pairs.shuffle(&mut rng);
    }

    /// Moves the inner items out as an iterator, consuming the seeder
    pub fn into_pairs(self) -> std::vec::IntoIter<(K, V)> {
        self.pairs.into_iter()
    }

    pub fn len(&self) -> usize {
        self.pairs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

// Implement file ingestion strictly on types that know how to parse text strings
impl<K, V> Seeder<K, V>
where
    K: FromStr + Ord,
    V: FromStr, // Only binds to this specific function block
    K::Err: std::fmt::Debug,
    V::Err: std::fmt::Debug,
{
    /// Streams a flat text file where BOTH keys and values are plain strings.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, KvError> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut seeder = Self::new();

        for (index, line_res) in reader.lines().enumerate() {
            let current_line = index + 1;
            let line = line_res?;
            let trimmed = line.trim();

            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }

            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() != 2 {
                return Err(KvError::InvalidRow {
                    line: current_line,
                    found_tokens: parts.len(),
                });
            }

            let key = parts[0].parse::<K>().map_err(|e| KvError::InvalidKey {
                line: current_line,
                text: parts[0].to_string(),
                details: format!("{:?}", e),
            })?;

            let val = parts[1].parse::<V>().map_err(|e| KvError::InvalidValue {
                line: current_line,
                text: parts[1].to_string(),
                details: format!("{:?}", e),
            })?;

            seeder.add(key, val);
        }

        if seeder.pairs.is_empty() {
            return Err(KvError::NoData);
        }

        Ok(seeder)
    }
}

impl<K, V> Default for Seeder<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> IntoIterator for Seeder<K, V> {
    type Item = (K, V);
    type IntoIter = std::vec::IntoIter<(K, V)>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_pairs()
    }
}
