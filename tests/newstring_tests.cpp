// use google tests
#include <gtest/gtest.h>
#include "splitstring_new.h"

TEST(DISABLED_DISABLED_TheirNewStringSplit, HappyPath)
{
    auto onetwothree = vecFromSAE("1#2#3");
    ASSERT_EQ(onetwothree.size(), 3);
    ASSERT_EQ(onetwothree[0], 1);
    ASSERT_EQ(onetwothree[1], 2);
    ASSERT_EQ(onetwothree[2], 3);
}

TEST(DISABLED_DISABLED_TheirNewStringSplit, MissingOne)
{
    auto onetwothree = vecFromSAE("1#2");
    ASSERT_EQ(onetwothree.size(), 0);
}

TEST(DISABLED_DISABLED_TheirNewStringSplit, Hole)
{
    auto onetwothree = vecFromSAE("1##3");
    ASSERT_EQ(onetwothree.size(), 0);
}

TEST(DISABLED_DISABLED_TheirNewStringSplit, NotNumber)
{
    auto onetwothree = vecFromSAE("A#2#3");
    ASSERT_EQ(onetwothree.size(), 0);
}

TEST(DISABLED_DISABLED_TheirNewStringSplit, OutOfRange3)
{
    auto onetwothree = vecFromSAE("1#2#65536");
    ASSERT_EQ(onetwothree.size(), 0);
}

TEST(DISABLED_DISABLED_TheirNewStringSplit, OutOfRange2)
{
    auto onetwothree = vecFromSAE("1#65536#3");
    ASSERT_EQ(onetwothree.size(), 0);
}

TEST(DISABLED_DISABLED_TheirNewStringSplit, OutOfRange1)
{
    auto onetwothree = vecFromSAE("65536#2#3");
    ASSERT_EQ(onetwothree.size(), 0);
}

TEST(DISABLED_DISABLED_TheirNewStringSplit, LowRangePass)
{
    auto onetwothree = vecFromSAE("0#0#0");
    ASSERT_EQ(onetwothree.size(), 3);
    ASSERT_EQ(onetwothree[0], 0);
    ASSERT_EQ(onetwothree[1], 0);
    ASSERT_EQ(onetwothree[2], 0);
}

TEST(DISABLED_DISABLED_TheirNewStringSplit, LowRangeFail1)
{
    auto onetwothree = vecFromSAE("-1#0#0");
    ASSERT_EQ(onetwothree.size(), 0);
}

TEST(DISABLED_DISABLED_TheirNewStringSplit, LowRangeFail2)
{
    auto onetwothree = vecFromSAE("0#-1#0");
    ASSERT_EQ(onetwothree.size(), 0);
}

TEST(DISABLED_DISABLED_TheirNewStringSplit, LowRangeFail3)
{
    auto onetwothree = vecFromSAE("0#0#-1");
    ASSERT_EQ(onetwothree.size(), 0);
}

TEST(DISABLED_DISABLED_TheirNewStringSplit, HighRangePass)
{
    auto onetwothree = vecFromSAE("65535,65535,65535");
    ASSERT_EQ(onetwothree.size(), 3);
    ASSERT_EQ(onetwothree[0], 65535);
    ASSERT_EQ(onetwothree[1], 65535);
    ASSERT_EQ(onetwothree[2], 65535);
}