#pragma once

#include <functional>

#include "printer.h"
#include "inputter.h"

namespace lc3 { namespace shims
{
    class PrinterShim: public lc3::utils::IPrinter
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

    /// Prints to a buffer.
    class BufferPrinter: public lc3::utils::IPrinter
    {
    public:
        BufferPrinter(size_t len, unsigned char buffer[/*len*/]): len(len), buffer(buffer) {}

        virtual void setColor(lc3::utils::PrintColor color) override;
        virtual void print(std::string const & string) override;
        virtual void newline(void) override;

    private:
        bool put(unsigned char c);

        size_t pos = 0;
        size_t len;
        unsigned char* buffer;
    };

    /// Gets inputs from a buffer.
    class BufferInputter: public lc3::utils::IInputter
    {
    public:
        BufferInputter(size_t len, unsigned char const buffer[/*len*/]):
            len(len), buffer(buffer) {}

        virtual void beginInput(void) override;
        virtual bool getChar(char & c) override;
        virtual void endInput(void) override;

    private:
        size_t pos = 0;
        size_t len;
        unsigned char const* buffer;
    };

    /// Calls a function on output.
    class CallbackPrinter: public lc3::utils::IPrinter
    {
    public:
        CallbackPrinter(void(*func)(unsigned char)): func(func) {}

        virtual void setColor(lc3::utils::PrintColor color) override;
        virtual void print(std::string const & string) override;
        virtual void newline(void) override;

    private:
        void (*func)(unsigned char);
    };

    /// Calls a function to get an input.
    ///
    /// Note: the function must ultimately produce a character but *can* block.
    class CallbackInputter: public lc3::utils::IInputter
    {
    public:
        CallbackInputter(unsigned char (*func)(void)): func(func) {}

        virtual void beginInput(void) override;
        virtual bool getChar(char & c) override;
        virtual void endInput(void) override;

    private:
        unsigned char (*func)(void);
    };
}; };
