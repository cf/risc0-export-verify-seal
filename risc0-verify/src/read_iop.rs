use crate::fp::Fp;
use crate::fp4::Fp4;
use crate::sha::{Digest, DIGEST_WORDS};
use crate::sha_rng::ShaRng;
use rand::{Error, RngCore};

#[derive(Clone, Debug)]
pub struct ReadIOP<'a> {
    proof: &'a [u32],
    rng: ShaRng,
}

impl<'a> ReadIOP<'a> {
    pub fn new(proof: &'a [u32]) -> Self {
        ReadIOP {
            proof,
            rng: ShaRng::default(),
        }
    }
    pub fn read_u32s(&mut self, x: &mut [u32]) {
        x.clone_from_slice(&self.proof[0..x.len()]);
        self.proof = &self.proof[x.len()..];
    }
    pub fn read_fps(&mut self, x: &mut [Fp]) {
        for i in 0..x.len() {
            x[i] = Fp::from(self.proof[i]);
        }
        self.proof = &self.proof[x.len()..];
    }
    pub fn read_fp4s(&mut self, x: &mut [Fp4]) {
        for i in 0..x.len() {
            x[i] = Fp4::new(
                Fp::from(self.proof[4 * i + 0]),
                Fp::from(self.proof[4 * i + 1]),
                Fp::from(self.proof[4 * i + 2]),
                Fp::from(self.proof[4 * i + 3]),
            )
        }
        self.proof = &self.proof[4 * x.len()..];
    }
    pub fn read_digests(&mut self, x: &mut [Digest]) {
        for i in 0..x.len() {
            x[i] = Digest::from_u32s(&self.proof[DIGEST_WORDS * i..DIGEST_WORDS * (i + 1)]);
        }
        self.proof = &self.proof[DIGEST_WORDS * x.len()..];
    }
    pub fn commit(&mut self, digest: &Digest) {
        self.rng.mix(digest);
    }
    pub fn verify_complete(&self) {
        assert!(self.proof.len() == 0);
    }
}

impl<'a> RngCore for ReadIOP<'a> {
    fn next_u32(&mut self) -> u32 {
        self.rng.next_u32()
    }
    fn next_u64(&mut self) -> u64 {
        self.rng.next_u64()
    }
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.rng.fill_bytes(dest)
    }
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        self.rng.try_fill_bytes(dest)
    }
}
