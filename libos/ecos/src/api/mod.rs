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

//! EcOS API module.

pub mod exo;

use crate::{subsystem::{SubsystemsArray, SubsystemResult}, pr_debug};
use eciton_sdk::context::Context;

/// LibOS Core module struct.
pub struct LibOSCore<'a> {
    /// Exokernel context.
    context: Context,
    /// LibOS subsystems.
    subsystems: SubsystemsArray<'a>,
}
impl<'a> LibOSCore<'a> {
    /// Construct new LibOSCore object.
    ///
    /// # Parameters
    /// - `context`    - given exokernel context struct.
    /// - `subsystems` - given libOS subsystems array.
    ///
    /// #Returns
    /// New LibOSCore object.
    pub fn new(context: Context, subsystems: SubsystemsArray<'a>) -> Self {
        Self { context, subsystems }
    }

    /// Initialize libOS subsystems.
    ///
    /// #Returns
    /// - `Ok`       - in case of success.
    /// - `Err(msg)` - error message otherwise.
    pub fn init(&mut self) -> SubsystemResult {
        // Initialize all libOS subsystems.
        for subsystem in &mut self.subsystems {
            let name = subsystem.name();

            pr_debug!("Initializing subsystem: '{name}'");
            subsystem.init()?;

            pr_debug!("Running subsystem: '{name}'");
            subsystem.run()?;
        }

        Ok(())
    }
}