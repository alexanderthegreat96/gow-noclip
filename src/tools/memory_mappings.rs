pub const GOW_PROC_NAME: &str = "GoWR.exe";

// fully reversed the entity structs and the camera structs
// the older implementation was absolutely bad and did not offer
// the correct pitch / yaw.
#[allow(dead_code)]
pub mod entity {
    pub const BASE: usize = 0x29EBB00;

    // world-space position (vec4, w = 1.0 at +0x3EC).
    pub const POS_X_OFFSET: usize = 0x3E0;
    pub const POS_Y_OFFSET: usize = 0x3E4; // up axis
    pub const POS_Z_OFFSET: usize = 0x3E8;

    // twin vec4 at the top of the struct — role unclear, but tracks +0x3E0 with
    // a small offset in normal play. suspected streaming/physics anchor.
    // we apply the same delta to it as +0x3E0 to keep them in lockstep.
    pub const POS_MIRROR_OFFSET: usize = 0x00;

    pub const ACCEL_OFFSET: usize = 0x3F4;

    // updates when the player is physically turning
    pub const BODY_YAW_OFFSET: usize = 0x400;

    pub const POS_X: (usize, &[usize]) = (BASE, &[POS_X_OFFSET]);
    pub const POS_Y: (usize, &[usize]) = (BASE, &[POS_Y_OFFSET]);
    pub const POS_Z: (usize, &[usize]) = (BASE, &[POS_Z_OFFSET]);
    pub const POS_MIRROR: (usize, &[usize]) = (BASE, &[POS_MIRROR_OFFSET]);
    pub const ACCEL: (usize, &[usize]) = (BASE, &[ACCEL_OFFSET]);
    pub const BODY_YAW: (usize, &[usize]) = (BASE, &[BODY_YAW_OFFSET]);
}

#[allow(dead_code)]
pub mod camera {
    pub const BASE: usize = 0x4025928;

    pub const YAW_OFFSET: usize = 0x12DC;
    pub const PITCH_OFFSET: usize = 0x12E0;

    //   [+0xF00] = -left  = right
    //   [+0xF20] = -forward = backward (eye direction)
    pub const LEFT_VEC_OFFSET: usize = 0xF00;
    pub const UP_VEC_OFFSET: usize = 0xF10;
    pub const FORWARD_VEC_OFFSET: usize = 0xF20;

    pub const YAW: (usize, &[usize]) = (BASE, &[YAW_OFFSET]);
    pub const PITCH: (usize, &[usize]) = (BASE, &[PITCH_OFFSET]);
    pub const LEFT_VEC: (usize, &[usize]) = (BASE, &[LEFT_VEC_OFFSET]);
    pub const UP_VEC: (usize, &[usize]) = (BASE, &[UP_VEC_OFFSET]);
    pub const FORWARD_VEC: (usize, &[usize]) = (BASE, &[FORWARD_VEC_OFFSET]);
}
