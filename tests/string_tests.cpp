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
    auto sae = NewSplitString("1,2,3", ",");
    assert(sae.has_value());
    assert(std::get<0>(sae.value()) == 1);
    assert(std::get<1>(sae.value()) == 2);
    assert(std::get<2>(sae.value()) == 3);
}

TEST(NewStringSplit, MissineOne)
{
    auto sae = NewSplitString("1,2", ",");
    assert(!sae.has_value());
}