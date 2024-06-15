#ifndef REGISTERS_HPP
#define REGISTERS_HPP

#include <array>
#include <cstdint>

struct Registers {
    std::array<std::uint8_t, 8> registers{};
    std::array<std::uint16_t, 2> special_registers{};
};
#endif //REGISTERS_HPP
