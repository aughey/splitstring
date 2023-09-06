#include <sstream>
#include "splitstring_new.h"

std::vector<uint16_t> vecFromSAE(std::string entitySAE)
{
   // Parse custom SAE identifiers
   if (entitySAE == "NOONE")
   {
      return {0, 0, 0};
   }

   if (entitySAE == "EVERYONE")
      return {65535, 65535, 65535};

   // TODO: for now we will be treating all other custom SAE identifiers as NOONE
   // TODO: We need to throw exception
   if (entitySAE.find('#') == std::string::npos)
      return {0, 0, 0};

   // Parse the "Site#App#Entity" string
   std::stringstream ss(entitySAE);
   std::vector<uint16_t> SAE;

   while (!ss.eof())
   {
      std::string buffer;
      std::getline(ss, buffer, '#');
      // TODO: raise exception if SAE is larger than uint16_t
      SAE.push_back(static_cast<uint16_t>(std::stoi(buffer)));
   }
   return SAE;
}
