// Eciton - experimental exokernel.
// Copyright (C) 2025 Alexander (@alkuzin).
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

//! LibOS subsystems main module.

pub mod graphics;

/// Return value of some subsystem methods.
pub type SubsystemResult = Result<(), &'static str>;

/// Subsystem array alias (used for `LibOSCore`).
pub type SubsystemsArray<'a> = [&'a mut dyn Subsystem;SUBSYSTEM_COUNT];

/// Number of libOS subsystems.
pub const SUBSYSTEM_COUNT: usize = 1;

/// LibOS subsystem trait.
pub trait Subsystem {
    /// Initialize subsystem.
    ///
    /// #Returns
    /// - `Ok`       - in case of success.
    /// - `Err(msg)` - error message otherwise.
    fn init(&self) -> SubsystemResult;

    /// Run subsystem.
    ///
    /// # Details
    /// Can also be useful for initializing mutable
    /// subsystem struct members.
    ///
    /// #Returns
    /// - `Ok`       - in case of success.
    /// - `Err(msg)` - error message otherwise.
    fn run(&mut self) -> SubsystemResult;

    /// Shutdown subsystem.
    ///
    /// #Returns
    /// - `Ok`       - in case of success.
    /// - `Err(msg)` - error message otherwise.
    fn exit(&self) -> SubsystemResult;

    /// Get subsystem name.
    ///
    /// #Returns
    /// - Subsystem name in string representation.
    fn name(&self) -> &'static str;
}