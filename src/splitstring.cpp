#include "splitstring.h"
#include <iostream>

// Pictoral representation of the extraction process
// Original String: 1 2 3 , 4 5 6 , 7 8 9
//                  first POS
// Extracted value: 1 2 3
// Substring: 4 5 6 , 7 8 9
//                   secondPOS
// Extracted value: 4 5 6
// Final extraction: 7 8 9

// Takes a string in the form 123,456,789 and returns a tuple of three uint16_t's.
// The delimiter separates numbers.
// there will be exactly three numbers
// the numbers will be in the range 0-65535
// non-numbers (cannot be converted with stoi) will return nullopt
// numbers out of range will return nullopt
// missing numbers will return nullopt
ThreeValuesOpt NewSplitString(const std::string &str, const std::string &delimiter)
{
   // sized as ints because stoi returns an int, we will do range checking and
   // then static cast to uint16_t.
   int first;
   int second;
   int third;

   // Do each individually/explicitly (since there are only three)
   auto first_pos = str.find(delimiter); // find the index of the delimiter
   if (first_pos == std::string::npos)
   {
      // fail if didn't find
      return std::nullopt;
   }
   // Extract the substring and convert
   try
   {
      first = std::stoi(str.substr(0, first_pos));
   }
   catch (...)
   {
      // stoi failed (substr is going to succeed)
      return std::nullopt;
   }
   // Check range
   if (first > UINT16_MAX || first < 0)
   {
      return std::nullopt;
   }

   // Find the second position, find can search from a starting point, so we
   // calculate the starting point from where the delimiter was plus the length of delim
   auto second_pos = str.find(delimiter, first_pos + delimiter.length());
   if (second_pos == std::string::npos)
   {
      return std::nullopt;
   }
   try
   {
      // Extract the substring and convert to int all in one.
      second = std::stoi(str.substr(first_pos + delimiter.length(), second_pos - first_pos - 1));
   }
   catch (...)
   {
      // Failed to convert
      return std::nullopt;
   }
   // Check range
   if (second > UINT16_MAX || second < 0)
   {
      return std::nullopt;
   }

   // last one doesn't have a comma, so it's just the remaining string from the
   // second delimiter position plus its length
   auto entity_str = str.substr(second_pos + delimiter.length());
   try
   {
      third = std::stoi(entity_str);
   }
   catch (...)
   {
      // Failed to convert
      return std::nullopt;
   }
   // Check range
   if (third > UINT16_MAX || third < 0)
   {
      return std::nullopt;
   }

   // Range checking was done above, this static cast is valid
   return std::make_tuple(static_cast<uint16_t>(first), static_cast<uint16_t>(second), static_cast<uint16_t>(third));
}

void OldSplitString(std::string str, const std::string &delimiter, uint16_t result[3])
{
   // Special cases
   if (str.compare("") == 0 || str.compare("aaaa") == 0 || str.find(delimiter) == str.npos || str.compare("ZZZZ") == 0 ||
       str.compare("YYYY") == 0)
   {
      return;
   }

   int index = 0;
   size_t pos = 0;
   std::string token;
   while ((pos = str.find(delimiter)) != std::string::npos)
   {
      token = str.substr(0, pos);

      int convertedToken(std::stoi(token));
      uint16_t myInt16(0);
      if (convertedToken <= static_cast<int>(UINT16_MAX) && convertedToken >= 0)
      {
         myInt16 = static_cast<uint16_t>(convertedToken);
      }
      else
      {
         std::cerr << "invalid number found found in string: " << str << std::endl;
      }

      result[index] = myInt16;

      str.erase(0, pos + delimiter.length());
      index++;
   }
   token = str.substr(0, pos);

   int convertedToken(std::stoi(token));
   uint16_t myInt16(0);
   if (convertedToken <= static_cast<int>(UINT16_MAX) && convertedToken >= 0)
   {
      myInt16 = static_cast<uint16_t>(convertedToken);
   }
   else
   {
      std::cerr << "invalid number found found in string: " << str << std::endl;
   }

   result[index] = myInt16;
}
