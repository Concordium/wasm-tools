/* Copyright 2018 Mozilla Foundation
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

use super::{
    BinaryReader, BinaryReaderError, EventType, ExternalKind, GlobalType, LinkingType, MemoryType,
    NameType, Naming, Operator, Range, RelocType, Result, SectionCode, TableType, Type,
};

pub use self::{
    alias_section::*, code_section::*, data_section::*, element_section::*, event_section::*,
    export_section::*, function_section::*, global_section::*, import_section::*, init_expr::*,
    instance_section::*, linking_section::*, memory_section::*, module_section::*, name_section::*,
    operators::*, producers_section::*, reloc_section::*, section_reader::*, table_section::*,
    type_section::*,
};

mod alias_section;
mod code_section;
mod data_section;
mod element_section;
mod event_section;
mod export_section;
mod function_section;
mod global_section;
mod import_section;
mod init_expr;
mod instance_section;
mod linking_section;
mod memory_section;
mod module_section;
mod name_section;
mod operators;
mod producers_section;
mod reloc_section;
mod section_reader;
mod table_section;
mod type_section;
