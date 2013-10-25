extern mod extra;
use extra::future::Future;

/*
Iterative Fibonacci numbers calculation.
*/
fn fib_iter(limit: int) -> int {
    let mut i = limit;
    let mut a = 0;
    let mut b = 1;
    let mut c = a + b;
    while i >= 0 {
        c = a;
        a = a + b;
        b = c;
        i -= 1;
    };
    c
}

fn fib_inner(i: int, a: int, b: int) -> int {
    if i > 0 {
        fib_inner(i - 1, a + b, a)
    }
    else {
        a
    }
}

/*
Recursive Fibonacci numbers calculation.
*/
fn fib_rec(limit: int) -> int {
    fib_inner(limit, 0, 1)
}

/*
Concurrent Fibonacci numbers calculation.
*/
fn fib_conc(limit: int) -> int {
    let (port, chan): (Port<int>, Chan<int>) = stream();
    do spawn {
        let result = fib_inner(limit, 0, 1);
        chan.send(result);
    };
    port.recv()
}

/*
Concurrent Fibonacci numbers calculation with futures.
*/
fn fib_futures(limit: int) -> int {
    let mut delayed_fib = Future::spawn (|| fib_inner(limit, 0, 1));
    delayed_fib.get()
}

fn main() {
    println(fmt!("Iterative: %d", fib_iter(100)));
    println(fmt!("Recursive: %d", fib_rec(100)));
    println(fmt!("Concurrent: %d", fib_conc(100)));
    println(fmt!("Futures: %d", fib_futures(100)));
}
