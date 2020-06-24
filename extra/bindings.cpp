#include "bindings.h"
#include "interface.h"
#include "simulator.h"
#include "shims.h"

// TODO: these require the frontend feature...
#include "console_printer.h"
#include "console_inputter.h"

extern "C" void *load_program(
    uint16_t const len,
    uint16_t const addresses[len],
    uint16_t const words[len]
) {
    // lc3::ConsolePrinter printer;
    // lc3::ConsoleInputter inputter;
    // lc3::sim simulator(printer, inputter, true, args.print_level, false);

    // auto printer = lc3::shims::noOpPrintShim();
    // auto inputter = lc3::shims::noOpInputShim();

    auto printer = new lc3::shims::PrinterShim(lc3::shims::noOpPrintShim());
    auto inputter = new lc3::shims::InputterShim(lc3::shims::noOpInputShim());

    auto sim = new lc3::sim(
        *printer,
        *inputter,
        false,
        0,
        false
    );

    sim->reinitialize();

    for (auto i = 0; i < len; i++) {
        sim->setMem(addresses[i], words[i]);
    }

    return sim;
}

extern "C" uint16_t get_mem(void* sim_ptr, uint16_t addr) {
    auto sim = static_cast<lc3::sim*>(sim_ptr);

    return sim->getMem(addr);
}

extern "C" State run_program(
    void* sim_ptr,
    uint16_t const pc
) {
    auto sim = static_cast<lc3::sim*>(sim_ptr);

    sim->setPC(pc);

    auto success = sim->runUntilHalt();
    // auto success = sim->run();

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

extern "C" void free_sim(void *sim_ptr) {
    auto sim = static_cast<lc3::sim*>(sim_ptr);
    // delete sim->inputter; // TODO: inputter
    // delete sim->inputter; // TODO: printer
    delete sim;
}
