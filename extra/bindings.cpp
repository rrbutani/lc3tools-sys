#include "bindings.h"
#include "interface.h"
#include "simulator.h"
#include "shims.h"

extern "C" lc3::sim *new_sim(lc3::utils::IPrinter *printer, lc3::utils::IInputter *inputter) {
    return new lc3::sim(
        *printer,
        *inputter,
        false,
        0,
        false
    );
}

extern "C" lc3::utils::IPrinter *no_op_printer(void) {
    auto printer = new lc3::shims::PrinterShim(lc3::shims::noOpPrintShim());
    return (lc3::utils::IPrinter*)(printer);
}

extern "C" lc3::utils::IInputter *no_op_inputter(void) {
    auto inputter = new lc3::shims::InputterShim(lc3::shims::noOpInputShim());
    return (lc3::utils::IInputter*)(inputter);
}

extern "C" lc3::sim *new_sim_with_no_op_io(void) {
    return new_sim(no_op_printer(), no_op_inputter());
}

extern "C" lc3::utils::IPrinter *buffer_printer(
    size_t const len,
    unsigned char buffer[/*len*/]
) {
    auto printer = new lc3::shims::BufferPrinter(len, buffer);
    return (lc3::utils::IPrinter*)(printer);
}

extern "C" lc3::utils::IInputter *buffer_inputter(
    size_t const len,
    unsigned char const buffer[/*len*/]
) {
    auto inputter = new lc3::shims::BufferInputter(len, buffer);
    return (lc3::utils::IInputter*)(inputter);
}

extern "C" lc3::utils::IPrinter *callback_printer(
    void (*func)(unsigned char)
) {
    auto printer = new lc3::shims::CallbackPrinter(func);
    return (lc3::utils::IPrinter*)(printer);
}

extern "C" lc3::utils::IInputter *callback_inputter(
    unsigned char (*func)(void)
) {
    auto inputter = new lc3::shims::CallbackInputter(func);
    return (lc3::utils::IInputter*)(inputter);
}

extern "C" void load_program(
    lc3::sim* sim,
    uint16_t const len,
    uint16_t const addresses[/*len*/],
    uint16_t const words[/*len*/]
) {
    sim->reinitialize();

    for (auto i = 0; i < len; i++) {
        sim->setMem(addresses[i], words[i]);
    }
}

extern "C" uint16_t get_mem(lc3::sim* sim, uint16_t addr) {
    return sim->getMem(addr);
}

extern "C" State run_program(
    lc3::sim* sim,
    uint16_t const pc
) {
    sim->setPC(pc);

    auto success = sim->runUntilHalt();

    return State {
        .regs = {
            sim->getReg(0),
            sim->getReg(1),
            sim->getReg(2),
            sim->getReg(3),
            sim->getReg(4),
            sim->getReg(5),
            sim->getReg(6),
            sim->getReg(7)
        },
        .pc = sim->getPC(),
        .cc = sim->getCC(),
        .psr = sim->getPSR(),
        .mcr = sim->getMCR(),
        .success = success,
    };
}

extern "C" void free_sim(lc3::sim *sim) {
    // delete sim->inputter; // TODO: inputter!
    // delete sim->inputter; // TODO: printer!
    delete sim;
}
