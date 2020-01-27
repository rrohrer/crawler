use rs_nds as nds;

pub fn init_video() {
    unsafe {
        nds::powerOn(nds::POWER_ALL_2D as i32);

        nds::vramSetMainBanks(
            nds::VRAM_A_MAIN_BG_0x06000000,
            nds::VRAM_B_MAIN_BG_0x06020000,
            nds::VRAM_C_SUB_BG_0x06200000,
            nds::VRAM_D_LCD,
        );

        nds::videoSetMode(nds::MODE_5_2D | nds::DISPLAY_BG2_ACTIVE | nds::DISPLAY_BG3_ACTIVE);
        nds::videoSetModeSub(nds::MODE_5_2D | nds::DISPLAY_BG3_ACTIVE);
    }
}
