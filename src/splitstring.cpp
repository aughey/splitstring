#include "splitstring.h"
#include <iostream>

SiteAppEntityOpt NewSplitString(const std::string &str, const std::string &delimiter) {
   uint16_t site;
   uint16_t app;
   uint16_t entity;

   // Do each individually/explicitly (since there are only three)
   auto site_pos = str.find(delimiter);
   if (site_pos == std::string::npos) {
      return std::nullopt;
   }
   try {
      site = std::stoi(str.substr(0, site_pos));
   } catch(...) {
      //std::cerr << "invalid number found found in string: " << str << std::endl;
      return std::nullopt;
   }
   
   auto app_pos = str.find(delimiter, site_pos + 1);
   if (app_pos == std::string::npos) {
      return std::nullopt;
   }
   try {
      app = std::stoi(str.substr(site_pos + 1, app_pos - site_pos - 1));
   } catch(...) {
      //std::cerr << "invalid number found found in string: " << str << std::endl;
      return std::nullopt;
   }

   // last one doesn't have a comma, so it's just the remaining string
   auto entity_str = str.substr(app_pos + 1);
   try {
      entity = std::stoi(entity_str);
   } catch(...) {
      //std::cerr << "invalid number found found in string: " << str << std::endl;
      return std::nullopt;
   }

   return std::make_tuple(site, app, entity);
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

/*
int main(int argc, char *argv[]) {
   std::string test_string_1 = "1,2,3";

   uint16_t result[3];
   OldSplitString(test_string_1, ",", result);
   assert(result[0] == 1);
   assert(result[1] == 2);
   assert(result[2] == 3);

   auto sae = NewSplitString(test_string_1, ",");
   assert(sae.has_value());
   assert(std::get<0>(sae.value()) == 1);
   assert(std::get<1>(sae.value()) == 2);
   assert(std::get<2>(sae.value()) == 3);

   // FAILS!
   // std::string test_string_2 = "A,2,3,4";
   // OldSplitString(test_string_2, ",", result);
   // assert(result[0] == 0);
   // assert(result[1] == 2);
   // assert(result[2] == 3);
}
*/