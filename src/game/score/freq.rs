#![doc = include_str!("../../../docs/game/score/freq.md")]
use std::collections::HashMap;

pub struct ScoreFrequencyLookup {
    freq_lookup: HashMap<u32, u32>,
}

impl ScoreFrequencyLookup {
    /// Constructor for the ScoreFrequencyLookup struct
    pub fn new() -> ScoreFrequencyLookup {
        ScoreFrequencyLookup{
            freq_lookup: HashMap::new()
        }
    }

    /// Insert a score into the lookup table
    pub fn insert(&mut self, score: u32, freq: u32) {
        self.freq_lookup.insert(
            score, freq
        );
    }

    /// Populate the lookup table
    pub fn create(&mut self) {
        self.insert(0, 443);
        self.insert(1, 0);
        self.insert(2, 7);
        self.insert(3, 547);
        self.insert(4, 1);
        self.insert(5, 14);
        self.insert(6, 431);
        self.insert(7, 850);
        self.insert(8, 220);
        self.insert(9, 312);
        self.insert(10, 1271);
        self.insert(11, 50);
        self.insert(12, 204);
        self.insert(13, 1059);
        self.insert(14, 1084);
        self.insert(15, 171);
        self.insert(16, 758);
        self.insert(17, 1617);
        self.insert(18, 105);
        self.insert(19, 438);
        self.insert(20, 1427);
        self.insert(21, 949);
        self.insert(22, 255);
        self.insert(23, 865);
        self.insert(24, 1354);
        self.insert(25, 163);
        self.insert(26, 427);
        self.insert(27, 1115);
        self.insert(28, 687);
        self.insert(29, 188);
        self.insert(30, 579);
        self.insert(31, 899);
        self.insert(32, 99);
        self.insert(33, 241);
        self.insert(34, 621);
        self.insert(35, 305);
        self.insert(36, 105);
        self.insert(37, 289);
        self.insert(38, 406);
        self.insert(39, 42);
        self.insert(40, 103);
        self.insert(41, 225);
        self.insert(42, 154);
        self.insert(43, 47);
        self.insert(44, 95);
        self.insert(45, 140);
        self.insert(46, 13);
        self.insert(47, 26);
        self.insert(48, 63);
        self.insert(49, 44);
        self.insert(50, 10);
        self.insert(51, 30);
        self.insert(52, 35);
        self.insert(53, 2);
        self.insert(54, 8);
        self.insert(55, 16);
        self.insert(56, 12);
        self.insert(57, 2);
        self.insert(58, 4);
        self.insert(59, 7);
        self.insert(60, 1);
        self.insert(61, 1);
        self.insert(62, 5);
    }

    /// Get the frequency of a score
    pub fn frequency(&self, score: u32) -> Result<u32, String> {
        match self.freq_lookup.get(&score) {
            Some(freq) => return Ok(*freq),
            None       => return Ok(1),
        }
    }
}
