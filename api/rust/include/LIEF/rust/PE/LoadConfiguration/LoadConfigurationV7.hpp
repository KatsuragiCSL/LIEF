/* Copyright 2024 - 2025 R. Thomas
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
#pragma once
#include "LIEF/PE/LoadConfigurations/LoadConfigurationV7.hpp"
#include "LIEF/rust/PE/LoadConfiguration/LoadConfigurationV6.hpp"

class PE_LoadConfigurationV7 : public PE_LoadConfigurationV6 {
  public:
  using lief_t = LIEF::PE::LoadConfigurationV7;
  PE_LoadConfigurationV7(const lief_t& base) : PE_LoadConfigurationV6(base) {}

  static bool classof(const PE_LoadConfiguration& config) {
    return lief_t::classof(&config.get());
  }

  uint32_t reserved3() const { return impl().reserved3(); }
  uint64_t addressof_unicode_string() const { return impl().addressof_unicode_string(); }

  private:
  const lief_t& impl() const { return as<lief_t>(this); }
};
