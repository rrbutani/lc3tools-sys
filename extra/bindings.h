#include <cstdint>

#include "simulator.h"

extern "C" {
    typedef struct State {
        uint32_t regs[8];
        uint32_t pc;
        char cc;
        uint32_t psr;
        uint32_t mcr;
        bool success;
    } State;

    // Sim constructors:
    /// Creates a new [`sim`] with the given `Printer` and `Inputter`.
    ///
    /// [`sim`]: crate::root::lc3::sim
    lc3::sim *new_sim(
        lc3::utils::IPrinter *printer,
        lc3::utils::IInputter *inputter,
        lc3::utils::PrintType print_level
    );
    /// Creates a new [`sim`] with the no-op `Printer` and `Inputter`.
    ///
    /// [`sim`]: crate::root::lc3::sim
    lc3::sim *new_sim_with_no_op_io(lc3::utils::PrintType print_level);

    // No-op I/O constructors:
    /// Creates a no-op `Printer`.
    lc3::utils::IPrinter *no_op_printer(void);
    /// Creates a no-op `Inputter`.
    lc3::utils::IInputter *no_op_inputter(void);

    // Buffer I/O constructors:
    /// Creates a `Printer` that's backed by a buffer.
    lc3::utils::IPrinter *buffer_printer(
        size_t const len,
        unsigned char buffer[/*len*/]
    );
    /// Creates an `Inputter` that's backed by a buffer.
    lc3::utils::IInputter *buffer_inputter(
        size_t const len,
        unsigned char const buffer[/*len*/]
    );

    // Callback I/O constructors:
    /// Creates a `Printer` that calls a function for every `char` that's
    /// emitted.
    lc3::utils::IPrinter *callback_printer(void (*func)(unsigned char));
    /// Creates an `Inputter` that calls a function to get input `char`s.
    ///
    /// The function provided must eventually produce a `char` but is allowed
    /// to block.
    lc3::utils::IInputter *callback_inputter(unsigned char (*func)(void));

    // Sim functions:
    /// Loads a program into memory (and resets memory, probably).
    void load_program(
        lc3::sim *sim,
        uint16_t const len,
        uint16_t const addresses[/*len*/],
        uint16_t const words[/*len*/]
    );
    /// Gets the value of a memory address.
    uint16_t get_mem(lc3::sim *sim, uint16_t addr);
    /// Runs the program starting at the given PC.
    ///
    /// Returns the machine state when the program halts (or raises an
    /// exception).
    State run_program(lc3::sim *sim, uint16_t const pc);
    /// Frees the memory allocated to the given [`sim`] instance.
    ///
    /// [`sim`]: crate::root::lc3::sim
    void free_sim(lc3::sim *sim);
}
