#pragma once
#include <string>
#include <optional>
#include <tuple>

typedef std::tuple<uint16_t, uint16_t, uint16_t> ThreeValues;
typedef std::optional<ThreeValues> ThreeValuesOpt;

ThreeValuesOpt NewSplitString(const std::string &str, const std::string &delimiter);
void OldSplitString(std::string str, const std::string &delimiter, uint16_t result[3]);
