use std::error::Error;
use std::ops::Deref;
use std::time::{Duration, Instant};

use lc3tools_sys::root::lc3::sim as Sim;
use lc3tools_sys::root::{
    free_sim, get_mem, load_program, new_sim_with_no_op_io, run_program, State,
};

use lc3_isa::{
    program,
    util::{AssembledProgram, LoadableIterator},
};
use pretty_assertions::assert_eq as eq;

trait LoadProgram {
    fn load<L, P>(&mut self, prog: P) -> Result<(), Box<dyn Error>>
    where
        P: Deref<Target = L>,
        for<'l> &'l L: LoadableIterator;
}

impl LoadProgram for Sim {
    fn load<L, P>(&mut self, prog: P) -> Result<(), Box<dyn Error>>
    where
        P: Deref<Target = L>,
        for<'l> &'l L: LoadableIterator,
    {
        for (addr, word) in &*prog {
            unsafe { self.setMem(addr, word) }
        }

        Ok(())
    }
}

fn time<R>(func: impl FnOnce() -> R) -> (R, Duration) {
    let start = Instant::now();
    let res = func();
    (res, start.elapsed())
}

fn main() {
    let builder = std::thread::Builder::new().stack_size(4 * 1024 * 1024);

    builder.spawn(inner_main).unwrap().join().unwrap()
}

fn inner_main() {
    #[rustfmt::skip]
    let prog_gen = |a: u16, b: u16| program! {
        .ORIG #0x3000;
        BRnzp @START;

        // Calculates a * b
        @A .FILL #a;
        @B .FILL #b;

        @START
        AND R0, R0, #0; // R0 as acc
        LD R1, @A;      // R1 as inc
        LD R2, @B;      // R2 as count

        // TODO: if b is negative, flip the signs of a and b.
        // i.e. 3 * -4 → -3 * 4
        //
        // For now, we'll just do unsigned numbers though.

        @LOOP
            BRz @END;

            ADD R0, R0, R1;
            ADD R2, R2, #-1;
            BRnzp @LOOP;

        @END
            ST R0, @RES;
            HALT;

        .ORIG #0x3020;
        @RES .FILL #0;
    }.into();

    c_interface(&prog_gen);

    println!();

    #[cfg(feature = "cpp-interface-example")]
    cpp_interface(&prog_gen);
}

fn c_interface(prog_gen: &impl Fn(u16, u16) -> AssembledProgram) {
    let test = |a: u16, b: u16| {
        print!("{:5} x {:5}: ", a, b);
        let prog: AssembledProgram = prog_gen(a, b);
        let expected =
            a.checked_mul(b).expect("multiplication does not overflow");

        let (mut addrs, mut words) = (Vec::new(), Vec::new());
        for (addr, word) in &prog {
            addrs.push(addr);
            words.push(word);
        }

        // If these were stable:
        // let (addrs, len, _) = addrs.into_raw_parts();
        // let (words, _, _) = words.into_raw_parts();

        let addrs_ptr = addrs.as_ptr();
        let words_ptr = words.as_ptr();
        let len = addrs.len();

        let sim = unsafe { new_sim_with_no_op_io() };
        unsafe { load_program(sim, len as u16, addrs_ptr, words_ptr) };

        drop((addrs, words));

        let (state, elapsed) = time(|| unsafe { run_program(sim, 0x3000) });
        println!("[in {:?}]", elapsed);

        let State { success, .. } = state;

        let got = unsafe { get_mem(sim, 0x3020) };
        unsafe { free_sim(sim) };

        assert!(success);
        eq!(expected, got, "Expected `{}`, got `{:?}`.", expected, state);
    };

    test(0, 0);
    test(0, 8);
    test(9, 0);
    test(1, 1);
    test(1, 50);
    test(30, 50);
    test(6, 7); // → 42
    test(1, 65535); // This one has the worst runtime.
}

#[cfg(feature = "cpp-interface-example")]
fn cpp_interface(prog_gen: &impl Fn(u16, u16) -> AssembledProgram) {
    use lc3tools_sys::root::lc3::shims::{noOpInputShim, noOpPrintShim};

    let mut printer = Box::new(unsafe { noOpPrintShim() });
    let mut input = Box::new(unsafe { noOpInputShim() });

    let mut test = |a: u16, b: u16| {
        print!("{:5} x {:5}: ", a, b);

        let mut sim = Box::new(unsafe {
            Sim::new(
                &mut printer._base as *mut _,
                &mut input._base as *mut _,
                true,
                0,
                false,
            )
        });

        let prog: AssembledProgram = prog_gen(a, b);
        let expected =
            a.checked_mul(b).expect("multiplication does not overflow");

        unsafe {
            sim.reinitialize();
        }
        sim.load::<AssembledProgram, _>(&prog).unwrap();

        unsafe {
            sim.setPC(0x3000);
        }
        let (success, elapsed) = time(|| unsafe { sim.runUntilHalt() });

        assert!(success);
        println!("[in {:?}]", elapsed);

        let got = unsafe { sim.getMem(0x3020) };
        eq!(expected, got, "Expected `{}`, got `{}`.", expected, got);
    };

    test(0, 0);
    test(0, 8);
    test(9, 0);
    test(1, 1);
    test(1, 50);
    test(30, 50);
    test(6, 7); // → 42
    test(1, 65535); // This one has the worst runtime.
}
