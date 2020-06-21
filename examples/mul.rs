use std::error::Error;
use std::ops::Deref;

use lc3tools_sys::root::lc3::sim as Sim;

use lc3_isa::{
    program,
    util::{AssembledProgram, LoadableIterator},
};

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
        for (addr, word) in prog.into_iter() {
            unsafe { self.setMem(addr, word) }
        }

        Ok(())
    }
}

fn main() {
    // let mut sim = unsafe { Sim::new(
    //     todo!(),
    //     todo!(),
    //     true,
    //     0,
    //     true,
    // )};

    let mut sim = Sim::default();

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
            HALT;
    }.into();

    let test = |sim: &mut Sim, foo: u16, bar: u16| {
        let prog: AssembledProgram = prog_gen(foo, bar);
        let expected = foo
            .checked_mul(bar)
            .expect("multiplication does not overflow");

        sim.load::<AssembledProgram, _>(&prog).unwrap();
        unsafe {
            sim.reinitialize();
        }
        assert!(unsafe { sim.runUntilHalt() });

        let got = unsafe { sim.getReg(0) };
        assert_eq!(expected, got, "Expected `{}`, got `{}`.", expected, got);
    };

    test(&mut sim, 0, 0);
    test(&mut sim, 0, 8);
    test(&mut sim, 9, 0);
    test(&mut sim, 1, 1);
    test(&mut sim, 1, 50);
    test(&mut sim, 30, 50);
    test(&mut sim, 6, 7); // → 42
}
