use crate::fp::Fp;

pub enum PolyOp {
    Const {
        out: usize,
        val: Fp,
    },
    Get {
        out: usize,
        idx: usize,
    },
    GetGlobal {
        out: usize,
        idx: usize,
    },
    Begin {
        out: usize,
    },
    AssertZero {
        out: usize,
        orig: usize,
        val: usize,
    },
    Combine {
        out: usize,
        orig: usize,
        cond: usize,
        inner: usize,
    },
    Add {
        out: usize,
        a: usize,
        b: usize,
    },
    Sub {
        out: usize,
        a: usize,
        b: usize,
    },
    Mul {
        out: usize,
        a: usize,
        b: usize,
    },
    Result {
        val: usize,
    },
}
