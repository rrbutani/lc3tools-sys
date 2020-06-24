#include "shims.h"
#include <iostream>

#if !(defined(WIN32) || defined(_WIN32) || defined(__WIN32))
#define UNUSED __attribute((unused))
#else
#define UNUSED [[maybe_unused]]
#endif

// If we were okay with being C++17+:
// #define UNUSED [[maybe_unused]]

// PrinterShim:
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

void lc3::shims::setColorNoOp(UNUSED lc3::utils::PrintColor _color) {}
void lc3::shims::printNoOp(UNUSED std::string const & _string) {}
void lc3::shims::newlineNoOp(void) {}


// InputterShim:
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

void lc3::shims::beginInputNoOp(void) {}
bool lc3::shims::getCharNoOp(UNUSED char & __c) { return false; }
void lc3::shims::endInputNoOp(void) {}


// BufferPrinter:
bool lc3::shims::BufferPrinter::put(char c) {
    if (this->pos < this->len) { this->buffer[this->len++] = c; return true; }
    else { return false; }
}
void lc3::shims::BufferPrinter::setColor(UNUSED lc3::utils::PrintColor color) {}
void lc3::shims::BufferPrinter::print(std::string const & string) {
    for (auto& c: string) { this->put(c); }
}
void lc3::shims::BufferPrinter::newline(void) { this->put('\n'); }


// BufferInputter:
void lc3::shims::BufferInputter::beginInput(void) {}
bool lc3::shims::BufferInputter::getChar(char & c) {
    if (this->pos < this->len) { c = this->buffer[this->len++]; return true; }
    else { return false; }
}
void lc3::shims::BufferInputter::endInput(void) {}


// CallbackPrinter:
void lc3::shims::CallbackPrinter::setColor(UNUSED lc3::utils::PrintColor color) {}
void lc3::shims::CallbackPrinter::print(std::string const & string) {
    for (auto& c: string) { this->func(c); }
}
void lc3::shims::CallbackPrinter::newline(void) { this->func('\n'); }


// CallbackInputter:
void lc3::shims::CallbackInputter::beginInput(void) {}
bool lc3::shims::CallbackInputter::getChar(char & c) {
    c = this->func();
    return true;
}
void lc3::shims::CallbackInputter::endInput(void) {}
