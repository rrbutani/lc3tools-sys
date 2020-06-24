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
    lc3::sim *new_sim(
        lc3::utils::IPrinter *printer,
        lc3::utils::IInputter *inputter
    );
    lc3::sim *new_sim_with_no_op_io(void);

    // No-op I/O constructors:
    lc3::utils::IPrinter *no_op_printer(void);
    lc3::utils::IInputter *no_op_inputter(void);

    // Buffer I/O constructors:
    lc3::utils::IPrinter *buffer_printer(
        size_t const len,
        char buffer[/*len*/]
    );
    lc3::utils::IInputter *buffer_inputter(
        size_t const len,
        char const buffer[/*len*/]
    );

    // Callback I/O constructors:
    lc3::utils::IPrinter *callback_printer(void (*func)(char));
    lc3::utils::IInputter *callback_inputter(char (*func)(void));

    // Sim functions:
    void load_program(
        lc3::sim *sim,
        uint16_t const len,
        uint16_t const addresses[/*len*/],
        uint16_t const words[/*len*/]
    );
    uint16_t get_mem(lc3::sim *sim, uint16_t addr);
    State run_program(lc3::sim *sim, uint16_t const pc);
    void free_sim(lc3::sim *sim);
}
