// Copyright (c) Bao Project and Contributors. All rights reserved.
//          João Peixoto <joaopeixotooficial@gmail.com>
//
// SPDX-License-Identifier: Apache-2.0

//! Bao custom types.

#![allow(dead_code)]

use serde::{Deserialize, Serialize};

/// Struct representing a Bao I/O request.
///
/// # Attributes
///
/// * `virtio_id` - Virtio instance ID.
/// * `reg_off` - Register offset.
/// * `addr` - Address.
/// * `op` - Operation.
/// * `value` - Value.
/// * `access_width` - Access width.
/// * `cpu_id` - Frontend CPU ID of the I/O request.
/// * `vcpu_id` - Frontend vCPU ID of the I/O request.
/// * `ret` - Return value.
#[repr(C)]
#[derive(Debug)]
pub struct BaoIoRequest {
    pub virtio_id: u64,
    pub reg_off: u64,
    pub addr: u64,
    pub op: u64,
    pub value: u64,
    pub access_width: u64,
    pub cpu_id: u64,
    pub vcpu_id: u64,
    pub ret: u64,
}

/// Struct representing a Bao I/O event file descriptor.
///
/// # Attributes
///
/// * `fd` - File descriptor.
/// * `flags` - Flags.
/// * `addr` - Address.
/// * `len` - Length.
/// * `reserved` - Reserved.
/// * `data` - Datamatch.
#[repr(C)]
pub struct BaoIoEventFd {
    pub fd: u32,
    pub flags: u32,
    pub addr: u64,
    pub len: u32,
    pub reserved: u32,
    pub data: u64,
}

/// Struct representing a Bao IRQ file descriptor.
///
/// # Attributes
///
/// * `fd` - File descriptor.
/// * `flags` - Flags.
#[repr(C)]
pub struct BaoIrqFd {
    pub fd: i32,
    pub flags: u32,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
/// Struct representing a Bao device configuration.
///
/// # Attributes
///
/// * `name` - Device name.
/// * `id` - Device ID.
/// * `type` - Device type.
/// * `irq` - Device IRQ.
/// * `addr` - Device address.
pub struct ConfigDevice {
    pub name: String,
    pub id: u32,
    #[serde(rename = "type")]
    pub device_type: String,
    pub irq: u32,
    pub addr: u64,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
/// Struct representing a Bao guest configuration.
///
/// # Attributes
///
/// * `name` - Guest name.
/// * `id` - Guest ID.
/// * `ram_addr` - Guest RAM address.
/// * `ram_size` - Guest RAM size.
/// * `shmem_path` - Guest shared memory path.
/// * `socket_path` - Guest socket path.
/// * `devices` - Guest devices.
pub struct ConfigGuest {
    pub name: String,
    pub id: u32,
    pub ram_addr: u64,
    pub ram_size: u64,
    pub shmem_path: String,
    pub socket_path: String,
    pub devices: Vec<ConfigDevice>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
/// Struct representing a Bao frontend configuration.
///
/// # Attributes
///
/// * `name` - Frontend name.
/// * `id` - Frontend ID.
/// * `guests` - Frontend guests.
pub struct ConfigFrontend {
    pub name: String,
    pub id: u32,
    pub guests: Vec<ConfigGuest>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
/// Struct representing a Bao frontends configuration.
///
/// # Attributes
///
/// * `frontends` - Frontends.
pub struct ConfigFrontends {
    pub frontends: Vec<ConfigFrontend>,
}
