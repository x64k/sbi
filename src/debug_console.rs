// SPDX-License-Identifier: MPL-2.0
// SPDX-FileCopyrightText: 2022 repnop
//
// This Source Code Form is subject to the terms of the Mozilla Public License,
// v. 2.0. If a copy of the MPL was not distributed with this file, You can
// obtain one at https://mozilla.org/MPL/2.0/.

use crate::{ecall1, ecall3, SbiError};

/// Debug Console Extension (DCE) ID
pub const EXTENSION_ID: usize = 0x4442434e;

/// Write a single byte to the debug console. This function will block until the
/// specified byte is written to the debug console.
/// 
/// ### Possible errors
/// 
/// [`SbiEffort::ErrFailed`]: Failed to write the byte due to I/O errors.
pub fn write_byte(byte: u8) -> Result<(), SbiError> {
    unsafe { 
        ecall1(byte as usize, EXTENSION_ID, 2).map(drop)
    }
}

/// Write a sequence of bytes from memory to the debug console. This function does
/// not block, and may write only part of the sequence, or even no bytes at all, if
/// the debug console is not able to accept more bytes. Upon success, it returns the
/// number of bytes written.
/// 
/// Note that the SBI interface does not take a raw address. The underlying interface
/// takes three arguments: the length of the byte sequence (in `a0`), and two XLEN
/// parameters in `a1` and `a2` representing, respectively, the the lower XLEN bits
/// and the higher XLEN bits of the physical address of the byte sequence. Errors
/// related to accessing the byte sequence (e.g. `SbiError::InvalidParam`) may refer
/// to any of these components.
/// 
/// ### Safety
/// 
/// The entire `bytes` slice must be accessible to the supervisor-level software and
/// the SBI implementation must be able to access the entire `bytes` slice using the
/// PMA attributes.
/// 
/// ### Possible errors
/// 
/// [`SbiError::InvalidParam`]: The byte sequence does not meet the required memory
/// access parameters.
/// [`SbiError::Failed`]: Write failed due to I/O errors.
pub fn write(bytes: &[u8]) -> Result<usize, SbiError> {
    unsafe {
        ecall3(bytes.len(),
        (&bytes[0] as *const u8) as usize, 0,
        EXTENSION_ID, 0)
    }
}

/// Read bytes from the debug console into output memory, up to the length of the
/// `bytes` slice. This function does not block and may not read any bytes if none
/// are available to be read in the debug console. Upon success, it returns the
/// number of bytes read.
/// 
/// Note that the SBI interface does not take a raw address. The underlying interface
/// takes three arguments: the length of the byte sequence (in `a0`), and two XLEN
/// parameters in `a1` and `a2` representing, respectively, the the lower XLEN bits
/// and the higher XLEN bits of the physical address of the byte sequence. Errors
/// related to accessing the byte sequence (e.g. `SbiError::InvalidParam`) may refer
/// to any of these components.
/// 
/// ### Safety
/// 
/// The entire `bytes` slice must be accessible to the supervisor-level software and
/// the SBI implementation must be able to access the entire `bytes` slice using the
/// PMA attributes.
/// 
/// This function will not request more bytes to be read than there is space available
/// in `bytes`. However, it makes no guarantees about how the underyling SBI implementation
/// will execute the call, and does not check for overflows.
pub fn read(bytes: &mut [u8]) -> Result<usize, SbiError> {
    unsafe {
        ecall3(bytes.len(),
               (&bytes[0] as *const u8) as usize, 0,
               EXTENSION_ID, 1)
    }
}