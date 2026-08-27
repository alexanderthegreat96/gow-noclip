pub const GOW_PROC_NAME: &str = "GoWR.exe";

#[allow(dead_code)]
pub mod entity {
    pub const SIG: &str = "30 24 1F 05 49 02 00 00 10 07 00 00";
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
}

#[allow(dead_code)]
pub mod camera {
    pub const SIG: &str = "48 8B 05 ?? ?? ?? ?? F3 0F 10 88 DC 12 00 00";
    pub const BASE: usize = 0x4025928;

    pub const YAW_OFFSET: usize = 0x12DC;
    pub const PITCH_OFFSET: usize = 0x12E0;

    //   [+0xF00] = -left  = right
    //   [+0xF20] = -forward = backward (eye direction)
    pub const LEFT_VEC_OFFSET: usize = 0xF00;
    pub const UP_VEC_OFFSET: usize = 0xF10;
    pub const FORWARD_VEC_OFFSET: usize = 0xF20;
}
