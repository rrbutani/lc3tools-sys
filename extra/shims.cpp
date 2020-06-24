#include "shims.h"
#include <iostream>

#if !(defined(WIN32) || defined(_WIN32) || defined(__WIN32))
#define UNUSED __attribute((unused))
#else
#define UNUSED [[maybe_unused]]
#endif

// If we were okay with being C++17+:
// #define UNUSED [[maybe_unused]]

void lc3::shims::PrinterShim::setColor(lc3::utils::PrintColor color) { this->setColorFunc(color); }
void lc3::shims::PrinterShim::print(std::string const & string) { this->printFunc(string); }
void lc3::shims::PrinterShim::newline(void) { this->newlineFunc(); }

lc3::shims::PrinterShim lc3::shims::noOpPrintShim(void) {
    return lc3::shims::PrinterShim(
        lc3::shims::setColorNoOp,
        lc3::shims::printNoOp,
        lc3::shims::newlineNoOp
    );
}

void lc3::shims::setColorNoOp(UNUSED lc3::utils::PrintColor _color) { }
void lc3::shims::printNoOp(UNUSED std::string const & _string) { }
void lc3::shims::newlineNoOp(void) { }


void lc3::shims::InputterShim::beginInput(void) { this->beginInputFunc(); }
bool lc3::shims::InputterShim::getChar(char & c) { return this->getCharFunc(c); }
void lc3::shims::InputterShim::endInput(void) { this->endInputFunc(); }

lc3::shims::InputterShim lc3::shims::noOpInputShim(void) {
    return lc3::shims::InputterShim(
        lc3::shims::beginInputNoOp,
        lc3::shims::getCharNoOp,
        lc3::shims::endInputNoOp
    );
}

void lc3::shims::beginInputNoOp(void) { }
bool lc3::shims::getCharNoOp(UNUSED char & __c) { return false; }
void lc3::shims::endInputNoOp(void) { }
