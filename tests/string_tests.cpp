// use google tests
#include <gtest/gtest.h>
#include "splitstring.h"

TEST(OldStringSplit, HappyPath)
{
    uint16_t result[3];
    OldSplitString("1,2,3", ",", result);
    ASSERT_EQ(result[0], 1);
    ASSERT_EQ(result[1], 2);
    ASSERT_EQ(result[2], 3);
}

TEST(NewStringSplit, HappyPath)
{
    auto onetwothree = NewSplitString("1,2,3", ",");
    assert(onetwothree.has_value());
    assert(std::get<0>(onetwothree.value()) == 1);
    assert(std::get<1>(onetwothree.value()) == 2);
    assert(std::get<2>(onetwothree.value()) == 3);
}

TEST(NewStringSplit, MissingOne)
{
    auto onetwothree = NewSplitString("1,2", ",");
    assert(!onetwothree.has_value());
}

TEST(NewStringSplit, Hole)
{
    auto onetwothree = NewSplitString("1,,3", ",");
    assert(!onetwothree.has_value());
}

TEST(NewStringSplit, NotNumber)
{
    auto onetwothree = NewSplitString("A,2,3", ",");
    assert(!onetwothree.has_value());
}

TEST(NewStringSplit, OutOfRange3)
{
    auto onetwothree = NewSplitString("1,2,65536", ",");
    assert(!onetwothree.has_value());
}

TEST(NewStringSplit, OutOfRange2)
{
    auto onetwothree = NewSplitString("1,65536,3", ",");
    assert(!onetwothree.has_value());
}

TEST(NewStringSplit, OutOfRange1)
{
    auto onetwothree = NewSplitString("65536,2,3", ",");
    assert(!onetwothree.has_value());
}

TEST(NewStringSplit, LowRangePass) {
    auto onetwothree = NewSplitString("0,0,0", ",");
    assert(onetwothree.has_value());
    assert(std::get<0>(onetwothree.value()) == 0);
    assert(std::get<1>(onetwothree.value()) == 0);
    assert(std::get<2>(onetwothree.value()) == 0);
}

TEST(NewStringSplit, LowRangeFail1) {
    auto onetwothree = NewSplitString("-1,0,0", ",");
    assert(!onetwothree.has_value());
}

TEST(NewStringSplit, LowRangeFail2) {
    auto onetwothree = NewSplitString("0,-1,0", ",");
    assert(!onetwothree.has_value());
}

TEST(NewStringSplit, LowRangeFail3) {
    auto onetwothree = NewSplitString("0,0,-1", ",");
    assert(!onetwothree.has_value());
}

TEST(NewStringSplit, HighRangePass) {
    auto onetwothree = NewSplitString("65535,65535,65535", ",");
    assert(onetwothree.has_value());
    assert(std::get<0>(onetwothree.value()) == 65535);
    assert(std::get<1>(onetwothree.value()) == 65535);
    assert(std::get<2>(onetwothree.value()) == 65535);
}