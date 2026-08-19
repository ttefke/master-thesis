pub const GLB_EM_TYPE_GLB_EM_0_KB: GlbEmType = 0;
pub const GLB_EM_TYPE_GLB_EM_8_KB: GlbEmType = 3;
pub const GLB_EM_TYPE_GLB_EM_16_KB: GlbEmType = 15;
pub type GlbEmType = u8;

pub fn early_init() {
    let hbn = unsafe { &*pac::Hbn::ptr() };
    // Set HBN AON PAD SMT bit
    hbn.hbn_irq_mode()
        .modify(|_r, w| w.reg_aon_pad_ie_smt().set_bit());
}

pub fn sys_init() {
    let channel: &pac::glb::RegisterBlock = unsafe { &*pac::Glb::ptr() };

    unsafe {
        // write em sel value
        channel.seam_misc().modify(|_r, w| w.em_sel().bits(0));
    }
}
