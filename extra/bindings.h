
#include <cstdint>

extern "C" {
    typedef struct State {
        uint32_t regs[8];
        uint32_t pc;
        char cc;
        uint32_t psr;
        uint32_t mcr;
        bool success;
    } State;

    void *load_program(
        uint16_t const len,
        uint16_t const addresses[/*len*/],
        uint16_t const words[/*len*/]
    );

    uint16_t get_mem(void* sim_ptr, uint16_t addr);

    State run_program(void* sim_ptr, uint16_t const pc);

    void free_sim(void *sim_ptr);
}
