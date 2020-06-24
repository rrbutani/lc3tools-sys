#pragma once

#include <functional>

#include "printer.h"
#include "inputter.h"

namespace lc3 { namespace shims
{
    class PrinterShim : public lc3::utils::IPrinter
    {
    public:
        PrinterShim(
            std::function<void(lc3::utils::PrintColor)> setColor,
            std::function<void(std::string const&)> print,
            std::function<void()> newline
        ): setColorFunc(setColor), printFunc(print), newlineFunc(newline) {}

        virtual void setColor(lc3::utils::PrintColor color) override;
        virtual void print(std::string const & string) override;
        virtual void newline(void) override;
    private:
        std::function<void(lc3::utils::PrintColor)> setColorFunc;
        std::function<void(std::string const&)> printFunc;
        std::function<void(void)> newlineFunc;
    };

    PrinterShim noOpPrintShim(void);

    void setColorNoOp(lc3::utils::PrintColor color);
    void printNoOp(std::string const & string);
    void newlineNoOp(void);

    class InputterShim: public lc3::utils::IInputter
    {
    public:
        InputterShim(
            std::function<void()> beginInput,
            std::function<bool(char & c)> getChar,
            std::function<void()> endInput
        ): beginInputFunc(beginInput), getCharFunc(getChar), endInputFunc(endInput) {}

        virtual void beginInput(void) override;
        virtual bool getChar(char & c) override;
        virtual void endInput(void) override;

    private:
        std::function<void(void)> beginInputFunc;
        std::function<bool(char & c)> getCharFunc;
        std::function<void(void)> endInputFunc;
    };

    InputterShim noOpInputShim(void);

    void beginInputNoOp(void);
    bool getCharNoOp(char & c);
    void endInputNoOp(void);
}; };
