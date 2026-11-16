// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use crate::bounds::Bounds;
use crate::instance_id::InstanceId;

// What a fresh instance is created with: its instance id and its bounds. It
// holds no ballot, and its first receipt is number 1.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Fresh {
    pub instance: InstanceId,
    pub bounds: Bounds,
}
