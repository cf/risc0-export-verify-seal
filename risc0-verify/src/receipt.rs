use crate::risc0_circuit::Risc0Circuit;
use crate::sha::Digest;
use arrayref::array_ref;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Receipt {
    journal: Vec<u8>,
    seal: Vec<u32>,
}

impl Receipt {
    pub fn verify(&self) {
        let mut circuit: Risc0Circuit = Risc0Circuit::default();
        crate::verify::verify(&mut circuit, &self.seal);
        assert!(self.journal.len() == (self.seal[8] as usize));
        if self.journal.len() > 32 {
            let digest = Digest::hash_bytes(&self.journal);
            assert!(digest == Digest::from_u32s(&self.seal[0..8]));
        } else {
            let mut vec = self.journal.clone();
            vec.resize(32, 0);
            for i in 0..8 {
                assert!(self.seal[i] == u32::from_le_bytes(*array_ref![&vec, i * 4, 4]));
            }
        }
    }
    pub fn get_journal_u32(&self) -> Vec<u32> {
        let mut as_words: Vec<u32> = vec![];
        assert!(self.journal.len() % 4 == 0);
        for i in 0..(self.journal.len() / 4) {
            as_words.push(u32::from_le_bytes(*array_ref![&self.journal, i * 4, 4]));
        }
        as_words
    }
}
