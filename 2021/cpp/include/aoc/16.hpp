#pragma once

#include <fstream>
#include <iostream>
#include <iterator>
#include <span>
#include <stdexcept>
#include <string>
#include <variant>
#include <vector>

#include <fmt/format.h>

#define DEBUG 0

using u8 = std::uint8_t;
using u16 = std::uint16_t;
using u32 = std::uint32_t;
using u64 = std::uint64_t;
using usize = std::size_t;

using i8 = std::int8_t;
using i16 = std::int16_t;
using i32 = std::int32_t;
using i64 = std::int64_t;
using isize = std::ptrdiff_t;

struct Literal;
struct Operator;

using Packet = std::variant<Literal, Operator>;

enum class Type {
  Sum,
  Mul,
  Min,
  Max,
  Literal,
  GreaterThan,
  LessThan,
  EqualTo,
};

struct Literal {
  u64 version;
  Type type_id;
  u64 value;
};

struct Operator {
  u64 version;
  Type type_id;
  std::vector<Packet> packets;
};

inline auto print(std::span<const u8> span) {
  fmt::print("[");
  for (size_t i = 0; i < span.size(); ++i) {
    if (i > 0)
      fmt::print(", ");
    fmt::print("{}", +span[i]);
  }
  fmt::print("]\n");
}

auto debug([[maybe_unused]] auto&&... args) {
#if defined(DEBUG) && DEBUG == 1
  fmt::print(std::forward<decltype(args)>(args)...);
#endif
}

class Advent {
public:
  Advent(char const *path) : bits{} {
    auto file = std::ifstream(path);
    auto line = std::string(std::istreambuf_iterator<char>(file),
                            std::istreambuf_iterator<char>());

    bits.reserve(4 * line.size());
    for (auto c : line) {
      int n = parse_hex(c);
      if (n == -1) break;
      bits.push_back((n & 8) > 0);
      bits.push_back((n & 4) > 0);
      bits.push_back((n & 2) > 0);
      bits.push_back((n & 1) > 0);
    }
  }

  auto part_one() {
    auto [packet, _] = parse_packet(bits);
    fmt::print("{}\n", version_sum(packet));
  }

  auto part_two() {
    auto [packet, _] = parse_packet(bits);
    auto result = evaluate(packet);
    fmt::print("{}\n", result);
  }

private:
  static constexpr u64 literal_packet_type = 4;
  static constexpr u64 version_bits = 3;
  static constexpr u64 type_id_bits = 3;
  std::vector<u8> bits;

  auto evaluate(Packet const& packet) -> u64 {
    if (auto *ptr = std::get_if<Literal>(&packet)) {
      return ptr->value;
    } else if (auto *ptr = std::get_if<Operator>(&packet)) {
      auto result = u64{};
      switch (ptr->type_id) {
        case Type::Sum:
          result = 0;
          for (auto packet : ptr->packets) {
            result += evaluate(packet);
          }
          break;
        case Type::Mul:
          result = 1;
          for (auto packet : ptr->packets) {
            result *= evaluate(packet);
          }
          break;
        case Type::Min:
          result = u64{} - 1;
          for (auto packet : ptr->packets) {
            auto value = evaluate(packet);
            result = value < result ? value : result;
          }
          break;
        case Type::Max:
          result = 0;
          for (auto packet : ptr->packets) {
            auto value = evaluate(packet);
            result = value > result ? value : result;
          }
          break;
        case Type::GreaterThan:
          result = evaluate(ptr->packets[0]) > evaluate(ptr->packets[1]);
          break;
        case Type::LessThan:
          result = evaluate(ptr->packets[0]) < evaluate(ptr->packets[1]); 
          break;
        case Type::EqualTo:
          result = evaluate(ptr->packets[0]) == evaluate(ptr->packets[1]); 
          break;
        default:
          throw std::runtime_error{"Invalid operation in packet"};
      }
      return result;
    } else {
      throw std::runtime_error{"Something went very wrong!"};
    }
  }

  auto version_sum(Packet const &packet) -> u64 {
    if (auto *ptr = std::get_if<Literal>(&packet)) {
      return ptr->version;
    } else if (auto *ptr = std::get_if<Operator>(&packet)) {
      auto sum = ptr->version;
      for (auto &packet : ptr->packets) {
        sum += version_sum(packet);
      }
      return sum;
    } else {
      throw std::runtime_error{"Something went very wrong!"};
    }
  }

  constexpr auto parse_hex(char c) -> i32 {
    if ('0' <= c && c <= '9')
      return c - '0';
    if ('a' <= c && c <= 'f')
      return c - 'a' + 10;
    if ('A' <= c && c <= 'F')
      return c - 'A' + 10;
    return -1;
  }

  static constexpr auto bits_to_value(std::span<const u8> bits) -> u64 {
    u64 result = 0;
    for (auto i = bits.size(), j = decltype(i)(0); i-- > 0; j++) {
      result |= bits[i] << j;
    }
    return result;
  }

  auto parse_packet(std::span<const u8> bits) -> std::tuple<Packet, size_t> {
    auto version = bits_to_value(bits.subspan(0, version_bits));
    auto type_id = static_cast<Type>(bits_to_value(bits.subspan(version_bits, type_id_bits)));
    auto packet_len = 6;
    bits = bits.subspan(packet_len);

    if (type_id == Type::Literal) {
      debug("... Parsing literal packet\n");
      u64 value = 0;
      bool done = false;
      while (!done) {
        value = (value << 4) | bits_to_value(bits.subspan(1, 4));
        done = bits[0] == 0;
        bits = bits.subspan(5);
        packet_len += 5;
      }
      debug("Found literal packet: (version = {}, type_id = {}, value = {}, size = {})\n", version, type_id, value, packet_len);
      return {Literal{version, type_id, value}, packet_len};
    } else {
      auto packets = std::vector<Packet>{};
      auto kind = bits[0];
      bits = bits.subspan(1);
      packet_len += 1;
      debug("... Parsing operator packet (length type = {})\n", +kind);

      if (kind) {
        auto num_packets = bits_to_value(bits.subspan(0, 11));
        bits = bits.subspan(11);
        packet_len += 11;
        debug("... Expecting {} sub-packets\n", num_packets);
        for (auto i = size_t{}; i < num_packets; ++i) {
          auto [packet, size] = parse_packet(bits);
          packets.push_back(packet);
          packet_len += size;
          bits = bits.subspan(size);
        }
      } else {
        auto bit_len = bits_to_value(bits.subspan(0, 15));
        bits = bits.subspan(15);
        packet_len += 15;
        debug("... Expecting {} bits in payload\n", bit_len);
        while (bit_len != 0) {
          auto [packet, size] = parse_packet(bits);
          packets.push_back(packet);
          packet_len += size;
          
          if (size <= bits.size()) {
            bits = bits.subspan(size);
          } else {
            throw std::runtime_error{"Too few remaining bits!"};
          }
          
          if (bit_len >= size) {
            bit_len -= size;
          } else {
            throw std::runtime_error{fmt::format("Remaining payload bits are too small! (bits: {}, size: {})", bit_len, size)};
          }
        }
      }
      debug("Found operator packet: (version = {}, type_id = {}, packets = [{}], size = {})\n", version, type_id, packets.size(), packet_len);
      return {Operator{version, type_id, std::move(packets)}, packet_len};
    }
  }
};
