#pragma once

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
            std::function<void()> newline,
        ): setColor(setColor), print(print), newline(newline) {}

        virtual void setColor(lc3::utils::PrintColor color) override;
        virtual void print(std::string const & string) override;
        virtual void newline(void) override;
    private:
        std::function<void(lc3::utils::PrintColor)> setColor;
        std::function<void(std::string const&)> print;
        std::function<void()> newline;
    };

    void setColorNoOp(lc3::utils::PrintColor color);
    void printNoOp(std::string const & string);
    void newlineNoOp(void);

    class InputterShim: public lc3::utils::IInputter
    {
    public:
        InputterShim(
            std::function<void()>() beginInput,
            std::function<bool(char & c)>() getChar,
            std::function<void()>() endInput,
        ): beginInput(beginInput), getChar(getChar), endInput(endInput) {}

        virtual void beginInput(void) override;
        virtual bool getChar(char & c) override;
        virtual void endInput(void) override;

    private:
        std::function<void()>() beginInput;
        std::function<bool(char & c)>() getChar;
        std::function<void()>() endInput;
    }

    void beginInputNoOp(void);
    bool getCharNoOp(char & c);
    void endInputNoOp(void);
}; };
