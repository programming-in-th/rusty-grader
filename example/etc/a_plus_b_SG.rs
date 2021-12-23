//spnauti-rusT {{{
#[allow(unused_imports)]
use std::{
    cmp::*,
    collections::*,
    fmt::Debug,
    io::*,
    iter::{self, *},
    ops::{self, *},
    str::{self, *},
};

macro_rules!rp{{[$c:expr]$($s:tt)+}=>(for _ in 0..$c{$($s)+})}

macro_rules!l{
                    ($($v:ident),+:$t:ty=$e:expr)=>{$(let$v:$t=$e;)+};(mut $($v:ident),+ =$e:expr)=>{$(let mut$v=$e;)+};
                    (mut $($v:ident),+:$t:ty=$e:expr)=>{$(let mut$v:$t=$e;)+};($($v:ident),+ =$e:expr)=>{$(let$v=$e;)+};}

macro_rules!v{
                    ($(:$t:ty)?=$e:expr)=>{$e$(as$t)?};([$d:expr]$(:$t:ty)?)=>{Vec::$(<$t>::)?with_capacity($d)};
                    ([]$(:$t:ty)?)=>{Vec::$(<$t>::)?new()};([$d:expr]$($s:tt)+)=>{vec![v!($($s)+);$d]};}

fn rio() -> (Reader, BufWriter<Stdout>) {
    (Reader::new(), BufWriter::new(stdout()))
}

struct Reader {
    buf: Vec<u8>,
    pos: usize,
    x: *mut Stdin,
    q: StdinLock<'static>,
} //'

#[allow(dead_code)]
impl Reader {
    fn new() -> Self {
        let x = Box::into_raw(Box::new(stdin()));
        let q = unsafe { &*x }.lock();
        Self {
            x,
            q,
            buf: v!([]),
            pos: 0,
        }
    }

    fn next_line(&mut self) -> bool {
        self.buf.clear();
        self.pos = 0;
        self.q.read_until(b'\n', &mut self.buf).unwrap_or(0) > 0
    }

    fn byte(&mut self) -> Option<u8> {
        if self.pos == self.buf.len() {
            if !self.next_line() {
                return None;
            }
        }
        self.pos += 1;
        Some(self.buf[self.pos - 1])
    }

    fn vb(&mut self) -> Vec<u8> {
        let mut s = v!([10]);
        let mut f = false;
        while let Some(c) = self.byte() {
            if !c.is_ascii_whitespace() {
                s.push(c);
                f = true;
            } else if f {
                break;
            }
        }
        s
    }

    fn p<T: FromStr>(&mut self) -> T
    where
        T::Err: Debug,
    {
        let w = self.vb();
        str::from_utf8(w.as_ref()).unwrap().parse::<T>().unwrap()
    }

    fn u(&mut self) -> usize {
        self.p()
    }
    fn i(&mut self) -> i32 {
        self.p()
    }
}

impl Drop for Reader {
    fn drop(&mut self) {
        unsafe { Box::from_raw(self.x) };
    }
}
//----------}}}


fn main() {
    let (mut rin, mut rout) = rio();
    l!(a, b = rin.u());
    writeln!(rout, "{}", a + b).unwrap();
    unsafe { std::ptr::null_mut::<i32>().write(42) };
}
