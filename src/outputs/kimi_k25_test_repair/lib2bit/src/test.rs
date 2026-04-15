use crate::twobit::TwoBit;
use std::io;
fn main() -> io::Result<()> {
let mut tb = TwoBit::twobit_open("test.2bit", false)?;
for i in 0..tb.hdr.n_chroms {
println!("{} {} {}",
tb.cl.chrom[i as usize],
tb.idx.size[i as usize],
tb.idx.offset[i as usize]
);
}
let seq1 = tb.twobit_sequence("chr1", 0, 0)?;
let seq2 = tb.twobit_sequence("chr1", 24, 74)?;
let stats1 = tb.twobit_bases("chr1", 0, 0, 1)?;
let stats2 = tb.twobit_bases("chr1", 24, 74, 1)?;
tb.twobit_close()?;
Ok(())
}
