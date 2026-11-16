// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use crate::instance_id::InstanceId;

// What a fresh instance is created with: its instance id. It holds no ballot,
// and its first receipt is number 1.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Fresh {
    pub instance: InstanceId,
}
