
#include "shims.h"

void lc3::shims::PrinterShim::setColor(lc3::utils::PrintColor color) { this.setColor(color); }
void lc3::shims::PrinterShim::print(std::string const & string) { this.print(string); }
void lc3::shims::PrinterShim::newline(void) { this.newline(); }

void lc3::shims::setColorNoOp(lc3::utils::PrintColor color) { }
void lc3::shims::printNoOp(std::string const & string) { }
void lc3::shims::newlineNoOp(void) { }


void lc3::shims::InputterShim::beginInput(void) { this.beginInput(); }
void lc3::shims::InputterShim::getChar(char & c) { this.getChar(); }
void lc3::shims::InputterShim::endInput(void) { this.endInput(); }

void beginInputNoOp(void) { }
bool getCharNoOp(char & c) { return false; }
void endInputNoOp(void) { }
