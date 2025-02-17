use crate::fp::Fp;
use crate::read_iop::ReadIOP;
use crate::sha::Digest;
use crate::util::to_po2;

pub struct MerkeTreeParams {
    pub row_size: usize,
    pub col_size: usize,
    pub queries: usize,
    pub layers: usize,
    pub top_layer: usize,
    pub top_size: usize,
}

impl MerkeTreeParams {
    pub fn new(row_size: usize, col_size: usize, queries: usize) -> Self {
        let layers: usize = to_po2(row_size);
        assert!(1 << layers == row_size);
        let mut top_layer = 0;
        for i in 1..layers {
            if (1 << i) > queries {
                break;
            }
            top_layer = i;
        }
        let top_size = 1 << top_layer;
        MerkeTreeParams {
            row_size,
            col_size,
            queries,
            layers,
            top_layer,
            top_size,
        }
    }
}

pub struct MerkleTreeVerifier {
    params: MerkeTreeParams,
    top: Vec<Digest>,
}

impl MerkleTreeVerifier {
    pub fn new(iop: &mut ReadIOP, row_size: usize, col_size: usize, queries: usize) -> Self {
        let params = MerkeTreeParams::new(row_size, col_size, queries);
        let mut top = vec![Digest::default(); params.top_size * 2];
        iop.read_digests(&mut top[params.top_size..]);
        for i in (1..params.top_size).rev() {
            top[i] = Digest::hash_pair(&top[2 * i], &top[2 * i + 1]);
        }
        iop.commit(&top[1]);
        return MerkleTreeVerifier { params, top };
    }
    pub fn root(&self) -> &Digest {
        return &self.top[1];
    }
    pub fn verify(&self, iop: &mut ReadIOP, mut idx: usize) -> Vec<Fp> {
        let col_size = self.params.col_size;
        let row_size = self.params.row_size;
        assert!(idx < row_size);
        let mut out: Vec<Fp> = vec![Fp::new(0); col_size];
        iop.read_fps(&mut out);
        let mut cur: Digest = Digest::hash_fps(&out);
        idx += row_size;
        while idx >= 2 * self.params.top_size {
            let low_bit = idx % 2;
            let mut other = Digest::default();
            iop.read_digests(std::slice::from_mut(&mut other));
            idx /= 2;
            if low_bit == 1 {
                cur = Digest::hash_pair(&other, &cur);
            } else {
                cur = Digest::hash_pair(&cur, &other);
            }
        }
        assert!(self.top[idx] == cur);
        out
    }
}
