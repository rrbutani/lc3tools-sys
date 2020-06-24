use std::error::Error;
use std::ops::Deref;

use lc3tools_sys::root::lc3::sim as Sim;
use lc3tools_sys::root::lc3::shims::{
    noOpInputShim,
    noOpPrintShim,
    // testPrinter,
};
use lc3tools_sys::root::{free_sim, get_mem, load_program, run_program, State};

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

fn main() {
    std::thread::Builder::new()
        .stack_size(32 * 1024 * 1024)
        .spawn(inner_main)
        .unwrap()
        .join()
        .unwrap()
}

fn inner_main() {
    /*
    let mut printer = Box::new(unsafe { noOpPrintShim() });
    let mut input = Box::new(unsafe { noOpInputShim() });

    // unsafe { testPrinter(&mut printer._base as *mut _); }

    println!("printed");

    // let mut sim = Box::new(unsafe { Sim::new(
    //     dbg!(&mut printer._base as *mut _),
    //     dbg!(&mut input._base as *mut _),
    //     true,
    //     0,
    //     false,
    // )});
    */

    #[rustfmt::skip]
    let prog_gen = |foo: u16, bar: u16| program! {
        .ORIG #0x3000;
        BRnzp @START;

        // Calculates foo * bar
        @FOO .FILL #foo;
        @BAR .FILL #bar;

        @START
        AND R0, R0, #0; // R0 as acc
        LD R1, @FOO;    // R1 as inc
        LD R2, @BAR;    // R2 as count

        // TODO: if bar is negative, flip the signs of foo and bar.
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

// .ORIG x3000
// BRnzp START

// ; Calculates foo * bar
// RES .FILL x0
// FOO .FILL x2
// BAR .FILL x4

// START
// AND R0, R0, #0 ; R0 as acc
// LD R1, FOO     ; R1 as inc
// LD R2, BAR     ; R2 as count

// ; TODO: if bar is negative, flip the signs of foo and bar.
// ; i.e. 3 * -4 → -3 * 4
// ;
// ; For now, we'll just do unsigned numbers though.

// LOOP
//     BRz FIN

//     ADD R0, R0, R1
//     ADD R2, R2, #-1
//     BRnzp LOOP

// FIN
//     ST R0, RES
//     HALT

// .END


    /*
    let mut test = |foo: u16, bar: u16| {
        let mut sim = Box::new(unsafe { Sim::new(
            dbg!(&mut printer._base as *mut _),
            dbg!(&mut input._base as *mut _),
            true,
            0,
            false,
        )});

        let prog: AssembledProgram = prog_gen(foo, bar);
        let expected = foo
            .checked_mul(bar)
            .expect("multiplication does not overflow");

        println!("loading");
        sim.load::<AssembledProgram, _>(&prog).unwrap();
        println!("loaded");
        unsafe {
            sim.reinitialize();
        }
        println!("init-ed");
        assert!(unsafe { sim.runUntilHalt() });
        println!("ran");

        let got = unsafe { sim.getReg(0) };
        assert_eq!(expected, got, "Expected `{}`, got `{}`.", expected, got);
    };
    */

    let test = |foo: u16, bar: u16| {
        print!("{:5} x {:5}: ", foo, bar);
        let prog: AssembledProgram = prog_gen(foo, bar);
        let expected = foo
            .checked_mul(bar)
            .expect("multiplication does not overflow");

        let (mut addrs, mut words) = (Vec::new(), Vec::new());
        for (addr, word) in &prog {
            addrs.push(addr);
            words.push(word);

            // println!("[W] {:#06X} → {:#06X}", addr, word);
        }

        // If these were stable:
        // let (addrs, len, _) = addrs.into_raw_parts();
        // let (words, _, _) = words.into_raw_parts();

        let addrs_ptr = addrs.as_ptr();
        let words_ptr = words.as_ptr();
        let len = addrs.len();

        let sim = unsafe { load_program(len as u16, addrs_ptr, words_ptr) };

        // println!("loaded");

        drop(addrs);
        drop(words);

        let start = std::time::Instant::now();
        let state = unsafe { run_program(sim, 0x03000) };
        /*e*/println!("[in {:?}]", start.elapsed());
        let State {
            // regs: [r0, _, _, _, _, _, _, _],
            success,
            ..
        } = state.clone();

        // let got = unsafe { get_mem(sim, 0x3020) };
        let got = unsafe { get_mem(sim, 0x3020) };

        unsafe { free_sim(sim) };

        assert!(success);
        eq!(expected, got, "Expected `{}`, got `{:?}`.", expected, got);
        println!("\n               OK");
    };

    test(/*&mut sim,*/ 0, 0);
    // println!("k");
    test(/*&mut sim,*/ 0, 8);
    test(/*&mut sim,*/ 9, 0);
    test(/*&mut sim,*/ 1, 1);
    test(/*&mut sim,*/ 1, 50);
    test(/*&mut sim,*/ 30, 50);
    test(/*&mut sim,*/ 6, 7); // → 42
    test(/*&mut sim,*/ 1, 65535); // This one has the worst runtime.
}
