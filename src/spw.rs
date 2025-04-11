#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    router_sts: RouterSts,
    router_cfg: RouterCfg,
    router_timeout: RouterTimeout,
    _reserved3: [u8; 0x74],
    router_table: [RouterTable; 224],
    link1_pi_rm: Link1PiRm,
    link1_pi_rcm: Link1PiRcm,
    link1_pi_r: Link1PiR,
    link1_pi_rcs: Link1PiRcs,
    link1_im: Link1Im,
    link1_pi_c: Link1PiC,
    link1_im_s: Link1ImS,
    link1_im_c: Link1ImC,
    link1_cfg: Link1Cfg,
    link1_clkdiv: Link1Clkdiv,
    link1_status: Link1Status,
    link1_swreset: Link1Swreset,
    link1_esccharevent0: Link1Esccharevent0,
    link1_esccharevent1: Link1Esccharevent1,
    link1_esccharsts: Link1Esccharsts,
    link1_transesc: Link1Transesc,
    link1_distintpi_rm: Link1DistintpiRm,
    link1_distintpi_rcm: Link1DistintpiRcm,
    link1_distintpi_r: Link1DistintpiR,
    link1_distintpi_rcs: Link1DistintpiRcs,
    link1_distintim: Link1Distintim,
    link1_distintpi_c: Link1DistintpiC,
    link1_distintim_s: Link1DistintimS,
    link1_distintim_c: Link1DistintimC,
    link1_distackpi_rm: Link1DistackpiRm,
    link1_distackpi_rcm: Link1DistackpiRcm,
    link1_distackpi_r: Link1DistackpiR,
    link1_distackpi_rcs: Link1DistackpiRcs,
    link1_distackim: Link1Distackim,
    link1_distackpi_c: Link1DistackpiC,
    link1_distackim_s: Link1DistackimS,
    link1_distackim_c: Link1DistackimC,
    link2_pi_rm: Link2PiRm,
    link2_pi_rcm: Link2PiRcm,
    link2_pi_r: Link2PiR,
    link2_pi_rcs: Link2PiRcs,
    link2_im: Link2Im,
    link2_pi_c: Link2PiC,
    link2_im_s: Link2ImS,
    link2_im_c: Link2ImC,
    link2_cfg: Link2Cfg,
    link2_clkdiv: Link2Clkdiv,
    link2_status: Link2Status,
    link2_swreset: Link2Swreset,
    link2_esccharevent0: Link2Esccharevent0,
    link2_esccharevent1: Link2Esccharevent1,
    link2_esccharsts: Link2Esccharsts,
    link2_transesc: Link2Transesc,
    link2_distintpi_rm: Link2DistintpiRm,
    link2_distintpi_rcm: Link2DistintpiRcm,
    link2_distintpi_r: Link2DistintpiR,
    link2_distintpi_rcs: Link2DistintpiRcs,
    link2_distintim: Link2Distintim,
    link2_distintpi_c: Link2DistintpiC,
    link2_distintim_s: Link2DistintimS,
    link2_distintim_c: Link2DistintimC,
    link2_distackpi_rm: Link2DistackpiRm,
    link2_distackpi_rcm: Link2DistackpiRcm,
    link2_distackpi_r: Link2DistackpiR,
    link2_distackpi_rcs: Link2DistackpiRcs,
    link2_distackim: Link2Distackim,
    link2_distackpi_c: Link2DistackpiC,
    link2_distackim_s: Link2DistackimS,
    link2_distackim_c: Link2DistackimC,
    _reserved68: [u8; 0x0300],
    pktrx1_pi_rm: Pktrx1PiRm,
    pktrx1_pi_rcm: Pktrx1PiRcm,
    pktrx1_pi_r: Pktrx1PiR,
    pktrx1_pi_rcs: Pktrx1PiRcs,
    pktrx1_im: Pktrx1Im,
    pktrx1_pi_c: Pktrx1PiC,
    pktrx1_im_s: Pktrx1ImS,
    pktrx1_im_c: Pktrx1ImC,
    pktrx1_cfg: Pktrx1Cfg,
    pktrx1_status: Pktrx1Status,
    _reserved78: [u8; 0x08],
    pktrx1_nxtbufdataaddr: Pktrx1Nxtbufdataaddr,
    pktrx1_nxtbufdatalen: Pktrx1Nxtbufdatalen,
    pktrx1_nxtbufpktaddr: Pktrx1Nxtbufpktaddr,
    pktrx1_nxtbufcfg: Pktrx1Nxtbufcfg,
    pktrx1_curbufdataaddr: Pktrx1Curbufdataaddr,
    pktrx1_curbufdatalen: Pktrx1Curbufdatalen,
    pktrx1_curbufpktaddr: Pktrx1Curbufpktaddr,
    pktrx1_curbufcfg: Pktrx1Curbufcfg,
    pktrx1_prevbufdatalen: Pktrx1Prevbufdatalen,
    pktrx1_prevbufsts: Pktrx1Prevbufsts,
    _reserved88: [u8; 0x24],
    pktrx1_swreset: Pktrx1Swreset,
    _reserved89: [u8; 0x0380],
    pkttx1_pi_rm: Pkttx1PiRm,
    pkttx1_pi_rcm: Pkttx1PiRcm,
    pkttx1_pi_r: Pkttx1PiR,
    pkttx1_pi_rcs: Pkttx1PiRcs,
    pkttx1_im: Pkttx1Im,
    pkttx1_pi_c: Pkttx1PiC,
    pkttx1_im_s: Pkttx1ImS,
    pkttx1_im_c: Pkttx1ImC,
    pkttx1_status: Pkttx1Status,
    pkttx1_nxtsendrout: Pkttx1Nxtsendrout,
    pkttx1_nxtsendaddr: Pkttx1Nxtsendaddr,
    pkttx1_nxtsendcfg: Pkttx1Nxtsendcfg,
    pkttx1_cursendrout: Pkttx1Cursendrout,
    pkttx1_cursendaddr: Pkttx1Cursendaddr,
    pkttx1_cursendcfg: Pkttx1Cursendcfg,
    pkttx1_swreset: Pkttx1Swreset,
    _reserved105: [u8; 0x01c0],
    rmap1_cfg: Rmap1Cfg,
    rmap1_sts_rc: Rmap1StsRc,
    rmap1_sts: Rmap1Sts,
    _reserved108: [u8; 0x74],
    tch_pi_rm: TchPiRm,
    tch_pi_rcm: TchPiRcm,
    tch_pi_r: TchPiR,
    tch_pi_rcs: TchPiRcs,
    tch_im: TchIm,
    tch_pi_c: TchPiC,
    tch_im_s: TchImS,
    tch_im_c: TchImC,
    tch_cfglisten: TchCfglisten,
    tch_cfgsend: TchCfgsend,
    tch_cfg: TchCfg,
    tch_cfgrestart: TchCfgrestart,
    tch_cfgtcevent: TchCfgtcevent,
    tch_cfgwd: TchCfgwd,
    tch_lasttimecode: TchLasttimecode,
    tch_swreset: TchSwreset,
    _reserved124: [u8; 0x40],
    group_irqsts1: GroupIrqsts1,
    group_irqsts2: GroupIrqsts2,
    _reserved126: [u8; 0x04],
    group_edacsts: GroupEdacsts,
}
impl RegisterBlock {
    #[doc = "0x00 - SpW Router Status"]
    #[inline(always)]
    pub const fn router_sts(&self) -> &RouterSts {
        &self.router_sts
    }
    #[doc = "0x04 - SpW Router Config"]
    #[inline(always)]
    pub const fn router_cfg(&self) -> &RouterCfg {
        &self.router_cfg
    }
    #[doc = "0x08 - SpW Router Timeout"]
    #[inline(always)]
    pub const fn router_timeout(&self) -> &RouterTimeout {
        &self.router_timeout
    }
    #[doc = "0x80..0x400 - SpW Router Table (Logical addresses 32 to 255, index 0 for logical address 32)"]
    #[inline(always)]
    pub const fn router_table(&self, n: usize) -> &RouterTable {
        &self.router_table[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x80..0x400 - SpW Router Table (Logical addresses 32 to 255, index 0 for logical address 32)"]
    #[inline(always)]
    pub fn router_table_iter(&self) -> impl Iterator<Item = &RouterTable> {
        self.router_table.iter()
    }
    #[doc = "0x400 - SpW Link 1 Pending Read Masked Interrupt"]
    #[inline(always)]
    pub const fn link1_pi_rm(&self) -> &Link1PiRm {
        &self.link1_pi_rm
    }
    #[doc = "0x404 - SpW Link 1 Pending Read and Clear Masked Interrupt"]
    #[inline(always)]
    pub const fn link1_pi_rcm(&self) -> &Link1PiRcm {
        &self.link1_pi_rcm
    }
    #[doc = "0x408 - SpW Link 1 Pending Read Interrupt"]
    #[inline(always)]
    pub const fn link1_pi_r(&self) -> &Link1PiR {
        &self.link1_pi_r
    }
    #[doc = "0x40c - SpW Link 1 Pending Read, Clear and Enabed Interrupt"]
    #[inline(always)]
    pub const fn link1_pi_rcs(&self) -> &Link1PiRcs {
        &self.link1_pi_rcs
    }
    #[doc = "0x410 - SpW Link 1 Interrupt Mask"]
    #[inline(always)]
    pub const fn link1_im(&self) -> &Link1Im {
        &self.link1_im
    }
    #[doc = "0x414 - SpW Link 1 Clear Pending Interrupt"]
    #[inline(always)]
    pub const fn link1_pi_c(&self) -> &Link1PiC {
        &self.link1_pi_c
    }
    #[doc = "0x418 - SpW Link 1 Interrupt Set Mask"]
    #[inline(always)]
    pub const fn link1_im_s(&self) -> &Link1ImS {
        &self.link1_im_s
    }
    #[doc = "0x41c - SpW Link 1 Interrupt Clear Mask"]
    #[inline(always)]
    pub const fn link1_im_c(&self) -> &Link1ImC {
        &self.link1_im_c
    }
    #[doc = "0x420 - SpW Link 1 Config"]
    #[inline(always)]
    pub const fn link1_cfg(&self) -> &Link1Cfg {
        &self.link1_cfg
    }
    #[doc = "0x424 - SpW Link 1 Clock Division"]
    #[inline(always)]
    pub const fn link1_clkdiv(&self) -> &Link1Clkdiv {
        &self.link1_clkdiv
    }
    #[doc = "0x428 - SpW Link 1 Status"]
    #[inline(always)]
    pub const fn link1_status(&self) -> &Link1Status {
        &self.link1_status
    }
    #[doc = "0x42c - SpW Link 1 Software Reset"]
    #[inline(always)]
    pub const fn link1_swreset(&self) -> &Link1Swreset {
        &self.link1_swreset
    }
    #[doc = "0x430 - SpW Link 1 Escape Character Event 0"]
    #[inline(always)]
    pub const fn link1_esccharevent0(&self) -> &Link1Esccharevent0 {
        &self.link1_esccharevent0
    }
    #[doc = "0x434 - SpW Link 1 Escape Character Event 1"]
    #[inline(always)]
    pub const fn link1_esccharevent1(&self) -> &Link1Esccharevent1 {
        &self.link1_esccharevent1
    }
    #[doc = "0x438 - SpW Link 1 Escape Character Status"]
    #[inline(always)]
    pub const fn link1_esccharsts(&self) -> &Link1Esccharsts {
        &self.link1_esccharsts
    }
    #[doc = "0x43c - SpW Link 1 Transmit Escape Character"]
    #[inline(always)]
    pub const fn link1_transesc(&self) -> &Link1Transesc {
        &self.link1_transesc
    }
    #[doc = "0x440 - SpW Link 1 Distributed Interrupt Pending Read Masked Interrupt"]
    #[inline(always)]
    pub const fn link1_distintpi_rm(&self) -> &Link1DistintpiRm {
        &self.link1_distintpi_rm
    }
    #[doc = "0x444 - SpW Link 1 Distributed Interrupt Pending Read and Clear Masked Interrupt"]
    #[inline(always)]
    pub const fn link1_distintpi_rcm(&self) -> &Link1DistintpiRcm {
        &self.link1_distintpi_rcm
    }
    #[doc = "0x448 - SpW Link 1 Distributed Interrupt Pending Read Interrupt"]
    #[inline(always)]
    pub const fn link1_distintpi_r(&self) -> &Link1DistintpiR {
        &self.link1_distintpi_r
    }
    #[doc = "0x44c - SpW Link 1 Distributed Interrupt Pending Read and Clear Interrupt"]
    #[inline(always)]
    pub const fn link1_distintpi_rcs(&self) -> &Link1DistintpiRcs {
        &self.link1_distintpi_rcs
    }
    #[doc = "0x450 - SpW Link 1 Distributed Interrupt Mask"]
    #[inline(always)]
    pub const fn link1_distintim(&self) -> &Link1Distintim {
        &self.link1_distintim
    }
    #[doc = "0x454 - SpW Link 1 Distributed Interrupt Clear Pending Interrupt"]
    #[inline(always)]
    pub const fn link1_distintpi_c(&self) -> &Link1DistintpiC {
        &self.link1_distintpi_c
    }
    #[doc = "0x458 - SpW Link 1 Distributed Interrupt Set Mask"]
    #[inline(always)]
    pub const fn link1_distintim_s(&self) -> &Link1DistintimS {
        &self.link1_distintim_s
    }
    #[doc = "0x45c - SpW Link 1 Distributed Interrupt Clear Mask"]
    #[inline(always)]
    pub const fn link1_distintim_c(&self) -> &Link1DistintimC {
        &self.link1_distintim_c
    }
    #[doc = "0x460 - SpW Link 1 Distributed Interrupt Acknowledge Pending Read Masked Interrupt"]
    #[inline(always)]
    pub const fn link1_distackpi_rm(&self) -> &Link1DistackpiRm {
        &self.link1_distackpi_rm
    }
    #[doc = "0x464 - SpW Link 1 Distributed Interrupt Acknowledge Pending Read and Clear Masked Interrupt"]
    #[inline(always)]
    pub const fn link1_distackpi_rcm(&self) -> &Link1DistackpiRcm {
        &self.link1_distackpi_rcm
    }
    #[doc = "0x468 - SpW Link 1 Distributed Interrupt Acknowledge Pending Read Interrupt"]
    #[inline(always)]
    pub const fn link1_distackpi_r(&self) -> &Link1DistackpiR {
        &self.link1_distackpi_r
    }
    #[doc = "0x46c - SpW Link 1 Distributed Interrupt Acknowledge Pending Read and Clear Interrupt"]
    #[inline(always)]
    pub const fn link1_distackpi_rcs(&self) -> &Link1DistackpiRcs {
        &self.link1_distackpi_rcs
    }
    #[doc = "0x470 - SpW Link 1 Distributed Interrupt Acknowledge Mask"]
    #[inline(always)]
    pub const fn link1_distackim(&self) -> &Link1Distackim {
        &self.link1_distackim
    }
    #[doc = "0x474 - SpW Link 1 Distributed Interrupt Acknowledge Clear Pending Interrupt"]
    #[inline(always)]
    pub const fn link1_distackpi_c(&self) -> &Link1DistackpiC {
        &self.link1_distackpi_c
    }
    #[doc = "0x478 - SpW Link 1 Distributed Interrupt Acknowledge Set Mask"]
    #[inline(always)]
    pub const fn link1_distackim_s(&self) -> &Link1DistackimS {
        &self.link1_distackim_s
    }
    #[doc = "0x47c - SpW Link 1 Distributed Interrupt Acknowledge Clear Mask"]
    #[inline(always)]
    pub const fn link1_distackim_c(&self) -> &Link1DistackimC {
        &self.link1_distackim_c
    }
    #[doc = "0x480 - SpW Link 2 Pending Read Masked Interrupt"]
    #[inline(always)]
    pub const fn link2_pi_rm(&self) -> &Link2PiRm {
        &self.link2_pi_rm
    }
    #[doc = "0x484 - SpW Link 2 Pending Read and Clear Masked Interrupt"]
    #[inline(always)]
    pub const fn link2_pi_rcm(&self) -> &Link2PiRcm {
        &self.link2_pi_rcm
    }
    #[doc = "0x488 - SpW Link 2 Pending Read Interrupt"]
    #[inline(always)]
    pub const fn link2_pi_r(&self) -> &Link2PiR {
        &self.link2_pi_r
    }
    #[doc = "0x48c - SpW Link 2 Pending Read, Clear and Enabled Interrupt"]
    #[inline(always)]
    pub const fn link2_pi_rcs(&self) -> &Link2PiRcs {
        &self.link2_pi_rcs
    }
    #[doc = "0x490 - SpW Link 2 Interrupt Mask"]
    #[inline(always)]
    pub const fn link2_im(&self) -> &Link2Im {
        &self.link2_im
    }
    #[doc = "0x494 - SpW Link 2 Clear Pending Interrupt"]
    #[inline(always)]
    pub const fn link2_pi_c(&self) -> &Link2PiC {
        &self.link2_pi_c
    }
    #[doc = "0x498 - SpW Link 2 Interrupt Set Mask"]
    #[inline(always)]
    pub const fn link2_im_s(&self) -> &Link2ImS {
        &self.link2_im_s
    }
    #[doc = "0x49c - SpW Link 2 Interrupt Clear Mask"]
    #[inline(always)]
    pub const fn link2_im_c(&self) -> &Link2ImC {
        &self.link2_im_c
    }
    #[doc = "0x4a0 - SpW Link 2 Config"]
    #[inline(always)]
    pub const fn link2_cfg(&self) -> &Link2Cfg {
        &self.link2_cfg
    }
    #[doc = "0x4a4 - SpW Link 2 Clock Division"]
    #[inline(always)]
    pub const fn link2_clkdiv(&self) -> &Link2Clkdiv {
        &self.link2_clkdiv
    }
    #[doc = "0x4a8 - SpW Link 2 Status"]
    #[inline(always)]
    pub const fn link2_status(&self) -> &Link2Status {
        &self.link2_status
    }
    #[doc = "0x4ac - SpW Link 2 Software Reset"]
    #[inline(always)]
    pub const fn link2_swreset(&self) -> &Link2Swreset {
        &self.link2_swreset
    }
    #[doc = "0x4b0 - SpW Link 2 Escape Character Event 0"]
    #[inline(always)]
    pub const fn link2_esccharevent0(&self) -> &Link2Esccharevent0 {
        &self.link2_esccharevent0
    }
    #[doc = "0x4b4 - SpW Link 2 Escape Character Event 1"]
    #[inline(always)]
    pub const fn link2_esccharevent1(&self) -> &Link2Esccharevent1 {
        &self.link2_esccharevent1
    }
    #[doc = "0x4b8 - SpW Link 2 Escape Character Status"]
    #[inline(always)]
    pub const fn link2_esccharsts(&self) -> &Link2Esccharsts {
        &self.link2_esccharsts
    }
    #[doc = "0x4bc - SpW Link 2 Transmit Escape Character"]
    #[inline(always)]
    pub const fn link2_transesc(&self) -> &Link2Transesc {
        &self.link2_transesc
    }
    #[doc = "0x4c0 - SpW Link 2 Distributed Interrupt Pending Read Masked Interrupt"]
    #[inline(always)]
    pub const fn link2_distintpi_rm(&self) -> &Link2DistintpiRm {
        &self.link2_distintpi_rm
    }
    #[doc = "0x4c4 - SpW Link 2 Distributed Interrupt Pending Read and Clear Masked Interrupt"]
    #[inline(always)]
    pub const fn link2_distintpi_rcm(&self) -> &Link2DistintpiRcm {
        &self.link2_distintpi_rcm
    }
    #[doc = "0x4c8 - SpW Link 2 Distributed Interrupt Pending Read Interrupt"]
    #[inline(always)]
    pub const fn link2_distintpi_r(&self) -> &Link2DistintpiR {
        &self.link2_distintpi_r
    }
    #[doc = "0x4cc - SpW Link 2 Distributed Interrupt Pending Read and Clear Interrupt"]
    #[inline(always)]
    pub const fn link2_distintpi_rcs(&self) -> &Link2DistintpiRcs {
        &self.link2_distintpi_rcs
    }
    #[doc = "0x4d0 - SpW Link 2 Distributed Interrupt Mask"]
    #[inline(always)]
    pub const fn link2_distintim(&self) -> &Link2Distintim {
        &self.link2_distintim
    }
    #[doc = "0x4d4 - SpW Link 2 Distributed Interrupt Clear Pending Interrupt"]
    #[inline(always)]
    pub const fn link2_distintpi_c(&self) -> &Link2DistintpiC {
        &self.link2_distintpi_c
    }
    #[doc = "0x4d8 - SpW Link 2 Distributed Interrupt Set Mask"]
    #[inline(always)]
    pub const fn link2_distintim_s(&self) -> &Link2DistintimS {
        &self.link2_distintim_s
    }
    #[doc = "0x4dc - SpW Link 2 Distributed Interrupt Clear Mask"]
    #[inline(always)]
    pub const fn link2_distintim_c(&self) -> &Link2DistintimC {
        &self.link2_distintim_c
    }
    #[doc = "0x4e0 - SpW Link 2 Distributed Interrupt Acknowledge Pending Read Masked Interrupt"]
    #[inline(always)]
    pub const fn link2_distackpi_rm(&self) -> &Link2DistackpiRm {
        &self.link2_distackpi_rm
    }
    #[doc = "0x4e4 - SpW Link 2 Distributed Interrupt Acknowledge Pending Read and Clear Masked Interrupt"]
    #[inline(always)]
    pub const fn link2_distackpi_rcm(&self) -> &Link2DistackpiRcm {
        &self.link2_distackpi_rcm
    }
    #[doc = "0x4e8 - SpW Link 2 Distributed Interrupt Acknowledge Pending Read Interrupt"]
    #[inline(always)]
    pub const fn link2_distackpi_r(&self) -> &Link2DistackpiR {
        &self.link2_distackpi_r
    }
    #[doc = "0x4ec - SpW Link 2 Distributed Interrupt Acknowledge Pending Read and Clear Interrupt"]
    #[inline(always)]
    pub const fn link2_distackpi_rcs(&self) -> &Link2DistackpiRcs {
        &self.link2_distackpi_rcs
    }
    #[doc = "0x4f0 - SpW Link 2 Distributed Interrupt Acknowledge Mask"]
    #[inline(always)]
    pub const fn link2_distackim(&self) -> &Link2Distackim {
        &self.link2_distackim
    }
    #[doc = "0x4f4 - SpW Link 2 Distributed Interrupt Acknowledge Clear Pending Interrupt"]
    #[inline(always)]
    pub const fn link2_distackpi_c(&self) -> &Link2DistackpiC {
        &self.link2_distackpi_c
    }
    #[doc = "0x4f8 - SpW Link 2 Distributed Interrupt Acknowledge Set Mask"]
    #[inline(always)]
    pub const fn link2_distackim_s(&self) -> &Link2DistackimS {
        &self.link2_distackim_s
    }
    #[doc = "0x4fc - SpW Link 2 Distributed Interrupt Acknowledge Clear Mask"]
    #[inline(always)]
    pub const fn link2_distackim_c(&self) -> &Link2DistackimC {
        &self.link2_distackim_c
    }
    #[doc = "0x800 - PktRx Pending Read Masked Interrupt"]
    #[inline(always)]
    pub const fn pktrx1_pi_rm(&self) -> &Pktrx1PiRm {
        &self.pktrx1_pi_rm
    }
    #[doc = "0x804 - PktRx Pending Read and Clear Masked Interrupt"]
    #[inline(always)]
    pub const fn pktrx1_pi_rcm(&self) -> &Pktrx1PiRcm {
        &self.pktrx1_pi_rcm
    }
    #[doc = "0x808 - PktRx Pending Read Interrupt"]
    #[inline(always)]
    pub const fn pktrx1_pi_r(&self) -> &Pktrx1PiR {
        &self.pktrx1_pi_r
    }
    #[doc = "0x80c - PktRx Pending Read, Clear and Enabled Interrupt"]
    #[inline(always)]
    pub const fn pktrx1_pi_rcs(&self) -> &Pktrx1PiRcs {
        &self.pktrx1_pi_rcs
    }
    #[doc = "0x810 - PktRx Interrupt Mask"]
    #[inline(always)]
    pub const fn pktrx1_im(&self) -> &Pktrx1Im {
        &self.pktrx1_im
    }
    #[doc = "0x814 - PktRx Clear Pending Interrupt"]
    #[inline(always)]
    pub const fn pktrx1_pi_c(&self) -> &Pktrx1PiC {
        &self.pktrx1_pi_c
    }
    #[doc = "0x818 - PktRx Interrupt Set Mask"]
    #[inline(always)]
    pub const fn pktrx1_im_s(&self) -> &Pktrx1ImS {
        &self.pktrx1_im_s
    }
    #[doc = "0x81c - PktRx Interrupt Clear Mask"]
    #[inline(always)]
    pub const fn pktrx1_im_c(&self) -> &Pktrx1ImC {
        &self.pktrx1_im_c
    }
    #[doc = "0x820 - PktRx Config"]
    #[inline(always)]
    pub const fn pktrx1_cfg(&self) -> &Pktrx1Cfg {
        &self.pktrx1_cfg
    }
    #[doc = "0x824 - PktRx Status"]
    #[inline(always)]
    pub const fn pktrx1_status(&self) -> &Pktrx1Status {
        &self.pktrx1_status
    }
    #[doc = "0x830 - PktRx Next Buffer Data Address"]
    #[inline(always)]
    pub const fn pktrx1_nxtbufdataaddr(&self) -> &Pktrx1Nxtbufdataaddr {
        &self.pktrx1_nxtbufdataaddr
    }
    #[doc = "0x834 - PktRx Next Buffer Data Length"]
    #[inline(always)]
    pub const fn pktrx1_nxtbufdatalen(&self) -> &Pktrx1Nxtbufdatalen {
        &self.pktrx1_nxtbufdatalen
    }
    #[doc = "0x838 - PktRx Next Buffer Packet Address"]
    #[inline(always)]
    pub const fn pktrx1_nxtbufpktaddr(&self) -> &Pktrx1Nxtbufpktaddr {
        &self.pktrx1_nxtbufpktaddr
    }
    #[doc = "0x83c - PktRx Next Buffer Config"]
    #[inline(always)]
    pub const fn pktrx1_nxtbufcfg(&self) -> &Pktrx1Nxtbufcfg {
        &self.pktrx1_nxtbufcfg
    }
    #[doc = "0x840 - PktRx Current Buffer Data Address"]
    #[inline(always)]
    pub const fn pktrx1_curbufdataaddr(&self) -> &Pktrx1Curbufdataaddr {
        &self.pktrx1_curbufdataaddr
    }
    #[doc = "0x844 - PktRx Current Buffer Data Length"]
    #[inline(always)]
    pub const fn pktrx1_curbufdatalen(&self) -> &Pktrx1Curbufdatalen {
        &self.pktrx1_curbufdatalen
    }
    #[doc = "0x848 - PktRx Current Buffer Packet Address"]
    #[inline(always)]
    pub const fn pktrx1_curbufpktaddr(&self) -> &Pktrx1Curbufpktaddr {
        &self.pktrx1_curbufpktaddr
    }
    #[doc = "0x84c - PktRx Current Buffer Config"]
    #[inline(always)]
    pub const fn pktrx1_curbufcfg(&self) -> &Pktrx1Curbufcfg {
        &self.pktrx1_curbufcfg
    }
    #[doc = "0x850 - PktRx Previous Buffer Data Length"]
    #[inline(always)]
    pub const fn pktrx1_prevbufdatalen(&self) -> &Pktrx1Prevbufdatalen {
        &self.pktrx1_prevbufdatalen
    }
    #[doc = "0x854 - PktRx Previous Buffer Status"]
    #[inline(always)]
    pub const fn pktrx1_prevbufsts(&self) -> &Pktrx1Prevbufsts {
        &self.pktrx1_prevbufsts
    }
    #[doc = "0x87c - PktRx Software Reset"]
    #[inline(always)]
    pub const fn pktrx1_swreset(&self) -> &Pktrx1Swreset {
        &self.pktrx1_swreset
    }
    #[doc = "0xc00 - PktTx Pending Read Masked Interrupt"]
    #[inline(always)]
    pub const fn pkttx1_pi_rm(&self) -> &Pkttx1PiRm {
        &self.pkttx1_pi_rm
    }
    #[doc = "0xc04 - PktTx Pending Read and Clear Masked Interrupt"]
    #[inline(always)]
    pub const fn pkttx1_pi_rcm(&self) -> &Pkttx1PiRcm {
        &self.pkttx1_pi_rcm
    }
    #[doc = "0xc08 - PktTx Pending Read Interrupt"]
    #[inline(always)]
    pub const fn pkttx1_pi_r(&self) -> &Pkttx1PiR {
        &self.pkttx1_pi_r
    }
    #[doc = "0xc0c - PktTx Pending Read, Clear and Enabled Interrupt"]
    #[inline(always)]
    pub const fn pkttx1_pi_rcs(&self) -> &Pkttx1PiRcs {
        &self.pkttx1_pi_rcs
    }
    #[doc = "0xc10 - PktTx Interrupt Mask"]
    #[inline(always)]
    pub const fn pkttx1_im(&self) -> &Pkttx1Im {
        &self.pkttx1_im
    }
    #[doc = "0xc14 - PktTx Clear Pending Interrupt"]
    #[inline(always)]
    pub const fn pkttx1_pi_c(&self) -> &Pkttx1PiC {
        &self.pkttx1_pi_c
    }
    #[doc = "0xc18 - PktTx Interrupt Set Mask"]
    #[inline(always)]
    pub const fn pkttx1_im_s(&self) -> &Pkttx1ImS {
        &self.pkttx1_im_s
    }
    #[doc = "0xc1c - PktTx Interrupt Clear Mask"]
    #[inline(always)]
    pub const fn pkttx1_im_c(&self) -> &Pkttx1ImC {
        &self.pkttx1_im_c
    }
    #[doc = "0xc20 - PktTx Status"]
    #[inline(always)]
    pub const fn pkttx1_status(&self) -> &Pkttx1Status {
        &self.pkttx1_status
    }
    #[doc = "0xc24 - PktTx Next Send List Router Bytes"]
    #[inline(always)]
    pub const fn pkttx1_nxtsendrout(&self) -> &Pkttx1Nxtsendrout {
        &self.pkttx1_nxtsendrout
    }
    #[doc = "0xc28 - PktTx Next Send List Address"]
    #[inline(always)]
    pub const fn pkttx1_nxtsendaddr(&self) -> &Pkttx1Nxtsendaddr {
        &self.pkttx1_nxtsendaddr
    }
    #[doc = "0xc2c - PktTx Next Send List Config"]
    #[inline(always)]
    pub const fn pkttx1_nxtsendcfg(&self) -> &Pkttx1Nxtsendcfg {
        &self.pkttx1_nxtsendcfg
    }
    #[doc = "0xc30 - PktTx Current Send List Router Bytes"]
    #[inline(always)]
    pub const fn pkttx1_cursendrout(&self) -> &Pkttx1Cursendrout {
        &self.pkttx1_cursendrout
    }
    #[doc = "0xc34 - PktTx Current Send List Address"]
    #[inline(always)]
    pub const fn pkttx1_cursendaddr(&self) -> &Pkttx1Cursendaddr {
        &self.pkttx1_cursendaddr
    }
    #[doc = "0xc38 - PktTx Current Send List Config"]
    #[inline(always)]
    pub const fn pkttx1_cursendcfg(&self) -> &Pkttx1Cursendcfg {
        &self.pkttx1_cursendcfg
    }
    #[doc = "0xc3c - PktTx Software Reset"]
    #[inline(always)]
    pub const fn pkttx1_swreset(&self) -> &Pkttx1Swreset {
        &self.pkttx1_swreset
    }
    #[doc = "0xe00 - SpW RMAP 1 Config"]
    #[inline(always)]
    pub const fn rmap1_cfg(&self) -> &Rmap1Cfg {
        &self.rmap1_cfg
    }
    #[doc = "0xe04 - SpW RMAP 1 Read and Clear Status"]
    #[inline(always)]
    pub const fn rmap1_sts_rc(&self) -> &Rmap1StsRc {
        &self.rmap1_sts_rc
    }
    #[doc = "0xe08 - SpW RMAP 1 Read Status"]
    #[inline(always)]
    pub const fn rmap1_sts(&self) -> &Rmap1Sts {
        &self.rmap1_sts
    }
    #[doc = "0xe80 - SpW Tch Pending Read Masked Interrupt"]
    #[inline(always)]
    pub const fn tch_pi_rm(&self) -> &TchPiRm {
        &self.tch_pi_rm
    }
    #[doc = "0xe84 - SpW Tch Pending Read and Clear Masked Interrupt"]
    #[inline(always)]
    pub const fn tch_pi_rcm(&self) -> &TchPiRcm {
        &self.tch_pi_rcm
    }
    #[doc = "0xe88 - SpW Tch Pending Read Interrupt"]
    #[inline(always)]
    pub const fn tch_pi_r(&self) -> &TchPiR {
        &self.tch_pi_r
    }
    #[doc = "0xe8c - SpW Tch Pending Read, Clear and Enabled Interrupt"]
    #[inline(always)]
    pub const fn tch_pi_rcs(&self) -> &TchPiRcs {
        &self.tch_pi_rcs
    }
    #[doc = "0xe90 - SpW Tch Interrupt Mask"]
    #[inline(always)]
    pub const fn tch_im(&self) -> &TchIm {
        &self.tch_im
    }
    #[doc = "0xe94 - SpW Tch Clear Pending Interrupt"]
    #[inline(always)]
    pub const fn tch_pi_c(&self) -> &TchPiC {
        &self.tch_pi_c
    }
    #[doc = "0xe98 - SpW Tch Interrupt Set Mask"]
    #[inline(always)]
    pub const fn tch_im_s(&self) -> &TchImS {
        &self.tch_im_s
    }
    #[doc = "0xe9c - SpW Tch Interrupt Clear Mask"]
    #[inline(always)]
    pub const fn tch_im_c(&self) -> &TchImC {
        &self.tch_im_c
    }
    #[doc = "0xea0 - SpW Tch Config Listener"]
    #[inline(always)]
    pub const fn tch_cfglisten(&self) -> &TchCfglisten {
        &self.tch_cfglisten
    }
    #[doc = "0xea4 - SpW Tch Config Sender"]
    #[inline(always)]
    pub const fn tch_cfgsend(&self) -> &TchCfgsend {
        &self.tch_cfgsend
    }
    #[doc = "0xea8 - SpW Tch Config"]
    #[inline(always)]
    pub const fn tch_cfg(&self) -> &TchCfg {
        &self.tch_cfg
    }
    #[doc = "0xeac - SpW Tch Config Restart"]
    #[inline(always)]
    pub const fn tch_cfgrestart(&self) -> &TchCfgrestart {
        &self.tch_cfgrestart
    }
    #[doc = "0xeb0 - SpW Tch Config Tc Event"]
    #[inline(always)]
    pub const fn tch_cfgtcevent(&self) -> &TchCfgtcevent {
        &self.tch_cfgtcevent
    }
    #[doc = "0xeb4 - SpW Tch Config Watchdog"]
    #[inline(always)]
    pub const fn tch_cfgwd(&self) -> &TchCfgwd {
        &self.tch_cfgwd
    }
    #[doc = "0xeb8 - SpW Tch Last Time Code"]
    #[inline(always)]
    pub const fn tch_lasttimecode(&self) -> &TchLasttimecode {
        &self.tch_lasttimecode
    }
    #[doc = "0xebc - SpW Tch Software Reset"]
    #[inline(always)]
    pub const fn tch_swreset(&self) -> &TchSwreset {
        &self.tch_swreset
    }
    #[doc = "0xf00 - SpW Group Interrupt status 1"]
    #[inline(always)]
    pub const fn group_irqsts1(&self) -> &GroupIrqsts1 {
        &self.group_irqsts1
    }
    #[doc = "0xf04 - SpW Group Interrupt status 2"]
    #[inline(always)]
    pub const fn group_irqsts2(&self) -> &GroupIrqsts2 {
        &self.group_irqsts2
    }
    #[doc = "0xf0c - SpW Group Interrupt status"]
    #[inline(always)]
    pub const fn group_edacsts(&self) -> &GroupEdacsts {
        &self.group_edacsts
    }
}
#[doc = "ROUTER_STS (r) register accessor: SpW Router Status\n\nYou can [`read`](crate::Reg::read) this register and get [`router_sts::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@router_sts`] module"]
#[doc(alias = "ROUTER_STS")]
pub type RouterSts = crate::Reg<router_sts::RouterStsSpec>;
#[doc = "SpW Router Status"]
pub mod router_sts;
#[doc = "ROUTER_CFG (rw) register accessor: SpW Router Config\n\nYou can [`read`](crate::Reg::read) this register and get [`router_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`router_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@router_cfg`] module"]
#[doc(alias = "ROUTER_CFG")]
pub type RouterCfg = crate::Reg<router_cfg::RouterCfgSpec>;
#[doc = "SpW Router Config"]
pub mod router_cfg;
#[doc = "ROUTER_TIMEOUT (r) register accessor: SpW Router Timeout\n\nYou can [`read`](crate::Reg::read) this register and get [`router_timeout::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@router_timeout`] module"]
#[doc(alias = "ROUTER_TIMEOUT")]
pub type RouterTimeout = crate::Reg<router_timeout::RouterTimeoutSpec>;
#[doc = "SpW Router Timeout"]
pub mod router_timeout;
#[doc = "ROUTER_TABLE (rw) register accessor: SpW Router Table (Logical addresses 32 to 255, index 0 for logical address 32)\n\nYou can [`read`](crate::Reg::read) this register and get [`router_table::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`router_table::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@router_table`] module"]
#[doc(alias = "ROUTER_TABLE")]
pub type RouterTable = crate::Reg<router_table::RouterTableSpec>;
#[doc = "SpW Router Table (Logical addresses 32 to 255, index 0 for logical address 32)"]
pub mod router_table;
#[doc = "LINK1_PI_RM (r) register accessor: SpW Link 1 Pending Read Masked Interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`link1_pi_rm::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@link1_pi_rm`] module"]
#[doc(alias = "LINK1_PI_RM")]
pub type Link1PiRm = crate::Reg<link1_pi_rm::Link1PiRmSpec>;
#[doc = "SpW Link 1 Pending Read Masked Interrupt"]
pub mod link1_pi_rm;
#[doc = "LINK1_PI_RCM (r) register accessor: SpW Link 1 Pending Read and Clear Masked Interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`link1_pi_rcm::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@link1_pi_rcm`] module"]
#[doc(alias = "LINK1_PI_RCM")]
pub type Link1PiRcm = crate::Reg<link1_pi_rcm::Link1PiRcmSpec>;
#[doc = "SpW Link 1 Pending Read and Clear Masked Interrupt"]
pub mod link1_pi_rcm;
#[doc = "LINK1_PI_R (r) register accessor: SpW Link 1 Pending Read Interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`link1_pi_r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@link1_pi_r`] module"]
#[doc(alias = "LINK1_PI_R")]
pub type Link1PiR = crate::Reg<link1_pi_r::Link1PiRSpec>;
#[doc = "SpW Link 1 Pending Read Interrupt"]
pub mod link1_pi_r;
#[doc = "LINK1_PI_RCS (rw) register accessor: SpW Link 1 Pending Read, Clear and Enabed Interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`link1_pi_rcs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`link1_pi_rcs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@link1_pi_rcs`] module"]
#[doc(alias = "LINK1_PI_RCS")]
pub type Link1PiRcs = crate::Reg<link1_pi_rcs::Link1PiRcsSpec>;
#[doc = "SpW Link 1 Pending Read, Clear and Enabed Interrupt"]
pub mod link1_pi_rcs;
#[doc = "LINK1_IM (rw) register accessor: SpW Link 1 Interrupt Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`link1_im::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`link1_im::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@link1_im`] module"]
#[doc(alias = "LINK1_IM")]
pub type Link1Im = crate::Reg<link1_im::Link1ImSpec>;
#[doc = "SpW Link 1 Interrupt Mask"]
pub mod link1_im;
#[doc = "LINK1_PI_C (w) register accessor: SpW Link 1 Clear Pending Interrupt\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`link1_pi_c::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@link1_pi_c`] module"]
#[doc(alias = "LINK1_PI_C")]
pub type Link1PiC = crate::Reg<link1_pi_c::Link1PiCSpec>;
#[doc = "SpW Link 1 Clear Pending Interrupt"]
pub mod link1_pi_c;
#[doc = "LINK1_IM_S (w) register accessor: SpW Link 1 Interrupt Set Mask\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`link1_im_s::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@link1_im_s`] module"]
#[doc(alias = "LINK1_IM_S")]
pub type Link1ImS = crate::Reg<link1_im_s::Link1ImSSpec>;
#[doc = "SpW Link 1 Interrupt Set Mask"]
pub mod link1_im_s;
#[doc = "LINK1_IM_C (w) register accessor: SpW Link 1 Interrupt Clear Mask\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`link1_im_c::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@link1_im_c`] module"]
#[doc(alias = "LINK1_IM_C")]
pub type Link1ImC = crate::Reg<link1_im_c::Link1ImCSpec>;
#[doc = "SpW Link 1 Interrupt Clear Mask"]
pub mod link1_im_c;
#[doc = "LINK1_CFG (rw) register accessor: SpW Link 1 Config\n\nYou can [`read`](crate::Reg::read) this register and get [`link1_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`link1_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@link1_cfg`] module"]
#[doc(alias = "LINK1_CFG")]
pub type Link1Cfg = crate::Reg<link1_cfg::Link1CfgSpec>;
#[doc = "SpW Link 1 Config"]
pub mod link1_cfg;
#[doc = "LINK1_CLKDIV (rw) register accessor: SpW Link 1 Clock Division\n\nYou can [`read`](crate::Reg::read) this register and get [`link1_clkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`link1_clkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@link1_clkdiv`] module"]
#[doc(alias = "LINK1_CLKDIV")]
pub type Link1Clkdiv = crate::Reg<link1_clkdiv::Link1ClkdivSpec>;
#[doc = "SpW Link 1 Clock Division"]
pub mod link1_clkdiv;
#[doc = "LINK1_STATUS (r) register accessor: SpW Link 1 Status\n\nYou can [`read`](crate::Reg::read) this register and get [`link1_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@link1_status`] module"]
#[doc(alias = "LINK1_STATUS")]
pub type Link1Status = crate::Reg<link1_status::Link1StatusSpec>;
#[doc = "SpW Link 1 Status"]
pub mod link1_status;
#[doc = "LINK1_SWRESET (rw) register accessor: SpW Link 1 Software Reset\n\nYou can [`read`](crate::Reg::read) this register and get [`link1_swreset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`link1_swreset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@link1_swreset`] module"]
#[doc(alias = "LINK1_SWRESET")]
pub type Link1Swreset = crate::Reg<link1_swreset::Link1SwresetSpec>;
#[doc = "SpW Link 1 Software Reset"]
pub mod link1_swreset;
#[doc = "LINK1_ESCCHAREVENT0 (rw) register accessor: SpW Link 1 Escape Character Event 0\n\nYou can [`read`](crate::Reg::read) this register and get [`link1_esccharevent0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`link1_esccharevent0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@link1_esccharevent0`] module"]
#[doc(alias = "LINK1_ESCCHAREVENT0")]
pub type Link1Esccharevent0 = crate::Reg<link1_esccharevent0::Link1Esccharevent0Spec>;
#[doc = "SpW Link 1 Escape Character Event 0"]
pub mod link1_esccharevent0;
#[doc = "LINK1_ESCCHAREVENT1 (rw) register accessor: SpW Link 1 Escape Character Event 1\n\nYou can [`read`](crate::Reg::read) this register and get [`link1_esccharevent1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`link1_esccharevent1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@link1_esccharevent1`] module"]
#[doc(alias = "LINK1_ESCCHAREVENT1")]
pub type Link1Esccharevent1 = crate::Reg<link1_esccharevent1::Link1Esccharevent1Spec>;
#[doc = "SpW Link 1 Escape Character Event 1"]
pub mod link1_esccharevent1;
#[doc = "LINK1_ESCCHARSTS (r) register accessor: SpW Link 1 Escape Character Status\n\nYou can [`read`](crate::Reg::read) this register and get [`link1_esccharsts::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@link1_esccharsts`] module"]
#[doc(alias = "LINK1_ESCCHARSTS")]
pub type Link1Esccharsts = crate::Reg<link1_esccharsts::Link1EsccharstsSpec>;
#[doc = "SpW Link 1 Escape Character Status"]
pub mod link1_esccharsts;
#[doc = "LINK1_TRANSESC (rw) register accessor: SpW Link 1 Transmit Escape Character\n\nYou can [`read`](crate::Reg::read) this register and get [`link1_transesc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`link1_transesc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@link1_transesc`] module"]
#[doc(alias = "LINK1_TRANSESC")]
pub type Link1Transesc = crate::Reg<link1_transesc::Link1TransescSpec>;
#[doc = "SpW Link 1 Transmit Escape Character"]
pub mod link1_transesc;
#[doc = "LINK1_DISTINTPI_RM (r) register accessor: SpW Link 1 Distributed Interrupt Pending Read Masked Interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`link1_distintpi_rm::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@link1_distintpi_rm`] module"]
#[doc(alias = "LINK1_DISTINTPI_RM")]
pub type Link1DistintpiRm = crate::Reg<link1_distintpi_rm::Link1DistintpiRmSpec>;
#[doc = "SpW Link 1 Distributed Interrupt Pending Read Masked Interrupt"]
pub mod link1_distintpi_rm;
#[doc = "LINK1_DISTINTPI_RCM (r) register accessor: SpW Link 1 Distributed Interrupt Pending Read and Clear Masked Interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`link1_distintpi_rcm::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@link1_distintpi_rcm`] module"]
#[doc(alias = "LINK1_DISTINTPI_RCM")]
pub type Link1DistintpiRcm = crate::Reg<link1_distintpi_rcm::Link1DistintpiRcmSpec>;
#[doc = "SpW Link 1 Distributed Interrupt Pending Read and Clear Masked Interrupt"]
pub mod link1_distintpi_rcm;
#[doc = "LINK1_DISTINTPI_R (r) register accessor: SpW Link 1 Distributed Interrupt Pending Read Interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`link1_distintpi_r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@link1_distintpi_r`] module"]
#[doc(alias = "LINK1_DISTINTPI_R")]
pub type Link1DistintpiR = crate::Reg<link1_distintpi_r::Link1DistintpiRSpec>;
#[doc = "SpW Link 1 Distributed Interrupt Pending Read Interrupt"]
pub mod link1_distintpi_r;
#[doc = "LINK1_DISTINTPI_RCS (rw) register accessor: SpW Link 1 Distributed Interrupt Pending Read and Clear Interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`link1_distintpi_rcs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`link1_distintpi_rcs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@link1_distintpi_rcs`] module"]
#[doc(alias = "LINK1_DISTINTPI_RCS")]
pub type Link1DistintpiRcs = crate::Reg<link1_distintpi_rcs::Link1DistintpiRcsSpec>;
#[doc = "SpW Link 1 Distributed Interrupt Pending Read and Clear Interrupt"]
pub mod link1_distintpi_rcs;
#[doc = "LINK1_DISTINTIM (rw) register accessor: SpW Link 1 Distributed Interrupt Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`link1_distintim::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`link1_distintim::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@link1_distintim`] module"]
#[doc(alias = "LINK1_DISTINTIM")]
pub type Link1Distintim = crate::Reg<link1_distintim::Link1DistintimSpec>;
#[doc = "SpW Link 1 Distributed Interrupt Mask"]
pub mod link1_distintim;
#[doc = "LINK1_DISTINTPI_C (w) register accessor: SpW Link 1 Distributed Interrupt Clear Pending Interrupt\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`link1_distintpi_c::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@link1_distintpi_c`] module"]
#[doc(alias = "LINK1_DISTINTPI_C")]
pub type Link1DistintpiC = crate::Reg<link1_distintpi_c::Link1DistintpiCSpec>;
#[doc = "SpW Link 1 Distributed Interrupt Clear Pending Interrupt"]
pub mod link1_distintpi_c;
#[doc = "LINK1_DISTINTIM_S (w) register accessor: SpW Link 1 Distributed Interrupt Set Mask\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`link1_distintim_s::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@link1_distintim_s`] module"]
#[doc(alias = "LINK1_DISTINTIM_S")]
pub type Link1DistintimS = crate::Reg<link1_distintim_s::Link1DistintimSSpec>;
#[doc = "SpW Link 1 Distributed Interrupt Set Mask"]
pub mod link1_distintim_s;
#[doc = "LINK1_DISTINTIM_C (w) register accessor: SpW Link 1 Distributed Interrupt Clear Mask\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`link1_distintim_c::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@link1_distintim_c`] module"]
#[doc(alias = "LINK1_DISTINTIM_C")]
pub type Link1DistintimC = crate::Reg<link1_distintim_c::Link1DistintimCSpec>;
#[doc = "SpW Link 1 Distributed Interrupt Clear Mask"]
pub mod link1_distintim_c;
#[doc = "LINK1_DISTACKPI_RM (r) register accessor: SpW Link 1 Distributed Interrupt Acknowledge Pending Read Masked Interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`link1_distackpi_rm::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@link1_distackpi_rm`] module"]
#[doc(alias = "LINK1_DISTACKPI_RM")]
pub type Link1DistackpiRm = crate::Reg<link1_distackpi_rm::Link1DistackpiRmSpec>;
#[doc = "SpW Link 1 Distributed Interrupt Acknowledge Pending Read Masked Interrupt"]
pub mod link1_distackpi_rm;
#[doc = "LINK1_DISTACKPI_RCM (r) register accessor: SpW Link 1 Distributed Interrupt Acknowledge Pending Read and Clear Masked Interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`link1_distackpi_rcm::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@link1_distackpi_rcm`] module"]
#[doc(alias = "LINK1_DISTACKPI_RCM")]
pub type Link1DistackpiRcm = crate::Reg<link1_distackpi_rcm::Link1DistackpiRcmSpec>;
#[doc = "SpW Link 1 Distributed Interrupt Acknowledge Pending Read and Clear Masked Interrupt"]
pub mod link1_distackpi_rcm;
#[doc = "LINK1_DISTACKPI_R (r) register accessor: SpW Link 1 Distributed Interrupt Acknowledge Pending Read Interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`link1_distackpi_r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@link1_distackpi_r`] module"]
#[doc(alias = "LINK1_DISTACKPI_R")]
pub type Link1DistackpiR = crate::Reg<link1_distackpi_r::Link1DistackpiRSpec>;
#[doc = "SpW Link 1 Distributed Interrupt Acknowledge Pending Read Interrupt"]
pub mod link1_distackpi_r;
#[doc = "LINK1_DISTACKPI_RCS (rw) register accessor: SpW Link 1 Distributed Interrupt Acknowledge Pending Read and Clear Interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`link1_distackpi_rcs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`link1_distackpi_rcs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@link1_distackpi_rcs`] module"]
#[doc(alias = "LINK1_DISTACKPI_RCS")]
pub type Link1DistackpiRcs = crate::Reg<link1_distackpi_rcs::Link1DistackpiRcsSpec>;
#[doc = "SpW Link 1 Distributed Interrupt Acknowledge Pending Read and Clear Interrupt"]
pub mod link1_distackpi_rcs;
#[doc = "LINK1_DISTACKIM (rw) register accessor: SpW Link 1 Distributed Interrupt Acknowledge Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`link1_distackim::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`link1_distackim::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@link1_distackim`] module"]
#[doc(alias = "LINK1_DISTACKIM")]
pub type Link1Distackim = crate::Reg<link1_distackim::Link1DistackimSpec>;
#[doc = "SpW Link 1 Distributed Interrupt Acknowledge Mask"]
pub mod link1_distackim;
#[doc = "LINK1_DISTACKPI_C (w) register accessor: SpW Link 1 Distributed Interrupt Acknowledge Clear Pending Interrupt\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`link1_distackpi_c::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@link1_distackpi_c`] module"]
#[doc(alias = "LINK1_DISTACKPI_C")]
pub type Link1DistackpiC = crate::Reg<link1_distackpi_c::Link1DistackpiCSpec>;
#[doc = "SpW Link 1 Distributed Interrupt Acknowledge Clear Pending Interrupt"]
pub mod link1_distackpi_c;
#[doc = "LINK1_DISTACKIM_S (w) register accessor: SpW Link 1 Distributed Interrupt Acknowledge Set Mask\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`link1_distackim_s::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@link1_distackim_s`] module"]
#[doc(alias = "LINK1_DISTACKIM_S")]
pub type Link1DistackimS = crate::Reg<link1_distackim_s::Link1DistackimSSpec>;
#[doc = "SpW Link 1 Distributed Interrupt Acknowledge Set Mask"]
pub mod link1_distackim_s;
#[doc = "LINK1_DISTACKIM_C (w) register accessor: SpW Link 1 Distributed Interrupt Acknowledge Clear Mask\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`link1_distackim_c::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@link1_distackim_c`] module"]
#[doc(alias = "LINK1_DISTACKIM_C")]
pub type Link1DistackimC = crate::Reg<link1_distackim_c::Link1DistackimCSpec>;
#[doc = "SpW Link 1 Distributed Interrupt Acknowledge Clear Mask"]
pub mod link1_distackim_c;
#[doc = "LINK2_PI_RM (r) register accessor: SpW Link 2 Pending Read Masked Interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`link2_pi_rm::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@link2_pi_rm`] module"]
#[doc(alias = "LINK2_PI_RM")]
pub type Link2PiRm = crate::Reg<link2_pi_rm::Link2PiRmSpec>;
#[doc = "SpW Link 2 Pending Read Masked Interrupt"]
pub mod link2_pi_rm;
#[doc = "LINK2_PI_RCM (r) register accessor: SpW Link 2 Pending Read and Clear Masked Interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`link2_pi_rcm::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@link2_pi_rcm`] module"]
#[doc(alias = "LINK2_PI_RCM")]
pub type Link2PiRcm = crate::Reg<link2_pi_rcm::Link2PiRcmSpec>;
#[doc = "SpW Link 2 Pending Read and Clear Masked Interrupt"]
pub mod link2_pi_rcm;
#[doc = "LINK2_PI_R (r) register accessor: SpW Link 2 Pending Read Interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`link2_pi_r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@link2_pi_r`] module"]
#[doc(alias = "LINK2_PI_R")]
pub type Link2PiR = crate::Reg<link2_pi_r::Link2PiRSpec>;
#[doc = "SpW Link 2 Pending Read Interrupt"]
pub mod link2_pi_r;
#[doc = "LINK2_PI_RCS (rw) register accessor: SpW Link 2 Pending Read, Clear and Enabled Interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`link2_pi_rcs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`link2_pi_rcs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@link2_pi_rcs`] module"]
#[doc(alias = "LINK2_PI_RCS")]
pub type Link2PiRcs = crate::Reg<link2_pi_rcs::Link2PiRcsSpec>;
#[doc = "SpW Link 2 Pending Read, Clear and Enabled Interrupt"]
pub mod link2_pi_rcs;
#[doc = "LINK2_IM (rw) register accessor: SpW Link 2 Interrupt Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`link2_im::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`link2_im::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@link2_im`] module"]
#[doc(alias = "LINK2_IM")]
pub type Link2Im = crate::Reg<link2_im::Link2ImSpec>;
#[doc = "SpW Link 2 Interrupt Mask"]
pub mod link2_im;
#[doc = "LINK2_PI_C (w) register accessor: SpW Link 2 Clear Pending Interrupt\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`link2_pi_c::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@link2_pi_c`] module"]
#[doc(alias = "LINK2_PI_C")]
pub type Link2PiC = crate::Reg<link2_pi_c::Link2PiCSpec>;
#[doc = "SpW Link 2 Clear Pending Interrupt"]
pub mod link2_pi_c;
#[doc = "LINK2_IM_S (w) register accessor: SpW Link 2 Interrupt Set Mask\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`link2_im_s::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@link2_im_s`] module"]
#[doc(alias = "LINK2_IM_S")]
pub type Link2ImS = crate::Reg<link2_im_s::Link2ImSSpec>;
#[doc = "SpW Link 2 Interrupt Set Mask"]
pub mod link2_im_s;
#[doc = "LINK2_IM_C (w) register accessor: SpW Link 2 Interrupt Clear Mask\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`link2_im_c::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@link2_im_c`] module"]
#[doc(alias = "LINK2_IM_C")]
pub type Link2ImC = crate::Reg<link2_im_c::Link2ImCSpec>;
#[doc = "SpW Link 2 Interrupt Clear Mask"]
pub mod link2_im_c;
#[doc = "LINK2_CFG (rw) register accessor: SpW Link 2 Config\n\nYou can [`read`](crate::Reg::read) this register and get [`link2_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`link2_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@link2_cfg`] module"]
#[doc(alias = "LINK2_CFG")]
pub type Link2Cfg = crate::Reg<link2_cfg::Link2CfgSpec>;
#[doc = "SpW Link 2 Config"]
pub mod link2_cfg;
#[doc = "LINK2_CLKDIV (rw) register accessor: SpW Link 2 Clock Division\n\nYou can [`read`](crate::Reg::read) this register and get [`link2_clkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`link2_clkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@link2_clkdiv`] module"]
#[doc(alias = "LINK2_CLKDIV")]
pub type Link2Clkdiv = crate::Reg<link2_clkdiv::Link2ClkdivSpec>;
#[doc = "SpW Link 2 Clock Division"]
pub mod link2_clkdiv;
#[doc = "LINK2_STATUS (r) register accessor: SpW Link 2 Status\n\nYou can [`read`](crate::Reg::read) this register and get [`link2_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@link2_status`] module"]
#[doc(alias = "LINK2_STATUS")]
pub type Link2Status = crate::Reg<link2_status::Link2StatusSpec>;
#[doc = "SpW Link 2 Status"]
pub mod link2_status;
#[doc = "LINK2_SWRESET (rw) register accessor: SpW Link 2 Software Reset\n\nYou can [`read`](crate::Reg::read) this register and get [`link2_swreset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`link2_swreset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@link2_swreset`] module"]
#[doc(alias = "LINK2_SWRESET")]
pub type Link2Swreset = crate::Reg<link2_swreset::Link2SwresetSpec>;
#[doc = "SpW Link 2 Software Reset"]
pub mod link2_swreset;
#[doc = "LINK2_ESCCHAREVENT0 (rw) register accessor: SpW Link 2 Escape Character Event 0\n\nYou can [`read`](crate::Reg::read) this register and get [`link2_esccharevent0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`link2_esccharevent0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@link2_esccharevent0`] module"]
#[doc(alias = "LINK2_ESCCHAREVENT0")]
pub type Link2Esccharevent0 = crate::Reg<link2_esccharevent0::Link2Esccharevent0Spec>;
#[doc = "SpW Link 2 Escape Character Event 0"]
pub mod link2_esccharevent0;
#[doc = "LINK2_ESCCHAREVENT1 (rw) register accessor: SpW Link 2 Escape Character Event 1\n\nYou can [`read`](crate::Reg::read) this register and get [`link2_esccharevent1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`link2_esccharevent1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@link2_esccharevent1`] module"]
#[doc(alias = "LINK2_ESCCHAREVENT1")]
pub type Link2Esccharevent1 = crate::Reg<link2_esccharevent1::Link2Esccharevent1Spec>;
#[doc = "SpW Link 2 Escape Character Event 1"]
pub mod link2_esccharevent1;
#[doc = "LINK2_ESCCHARSTS (r) register accessor: SpW Link 2 Escape Character Status\n\nYou can [`read`](crate::Reg::read) this register and get [`link2_esccharsts::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@link2_esccharsts`] module"]
#[doc(alias = "LINK2_ESCCHARSTS")]
pub type Link2Esccharsts = crate::Reg<link2_esccharsts::Link2EsccharstsSpec>;
#[doc = "SpW Link 2 Escape Character Status"]
pub mod link2_esccharsts;
#[doc = "LINK2_TRANSESC (rw) register accessor: SpW Link 2 Transmit Escape Character\n\nYou can [`read`](crate::Reg::read) this register and get [`link2_transesc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`link2_transesc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@link2_transesc`] module"]
#[doc(alias = "LINK2_TRANSESC")]
pub type Link2Transesc = crate::Reg<link2_transesc::Link2TransescSpec>;
#[doc = "SpW Link 2 Transmit Escape Character"]
pub mod link2_transesc;
#[doc = "LINK2_DISTINTPI_RM (r) register accessor: SpW Link 2 Distributed Interrupt Pending Read Masked Interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`link2_distintpi_rm::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@link2_distintpi_rm`] module"]
#[doc(alias = "LINK2_DISTINTPI_RM")]
pub type Link2DistintpiRm = crate::Reg<link2_distintpi_rm::Link2DistintpiRmSpec>;
#[doc = "SpW Link 2 Distributed Interrupt Pending Read Masked Interrupt"]
pub mod link2_distintpi_rm;
#[doc = "LINK2_DISTINTPI_RCM (r) register accessor: SpW Link 2 Distributed Interrupt Pending Read and Clear Masked Interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`link2_distintpi_rcm::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@link2_distintpi_rcm`] module"]
#[doc(alias = "LINK2_DISTINTPI_RCM")]
pub type Link2DistintpiRcm = crate::Reg<link2_distintpi_rcm::Link2DistintpiRcmSpec>;
#[doc = "SpW Link 2 Distributed Interrupt Pending Read and Clear Masked Interrupt"]
pub mod link2_distintpi_rcm;
#[doc = "LINK2_DISTINTPI_R (r) register accessor: SpW Link 2 Distributed Interrupt Pending Read Interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`link2_distintpi_r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@link2_distintpi_r`] module"]
#[doc(alias = "LINK2_DISTINTPI_R")]
pub type Link2DistintpiR = crate::Reg<link2_distintpi_r::Link2DistintpiRSpec>;
#[doc = "SpW Link 2 Distributed Interrupt Pending Read Interrupt"]
pub mod link2_distintpi_r;
#[doc = "LINK2_DISTINTPI_RCS (rw) register accessor: SpW Link 2 Distributed Interrupt Pending Read and Clear Interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`link2_distintpi_rcs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`link2_distintpi_rcs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@link2_distintpi_rcs`] module"]
#[doc(alias = "LINK2_DISTINTPI_RCS")]
pub type Link2DistintpiRcs = crate::Reg<link2_distintpi_rcs::Link2DistintpiRcsSpec>;
#[doc = "SpW Link 2 Distributed Interrupt Pending Read and Clear Interrupt"]
pub mod link2_distintpi_rcs;
#[doc = "LINK2_DISTINTIM (rw) register accessor: SpW Link 2 Distributed Interrupt Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`link2_distintim::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`link2_distintim::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@link2_distintim`] module"]
#[doc(alias = "LINK2_DISTINTIM")]
pub type Link2Distintim = crate::Reg<link2_distintim::Link2DistintimSpec>;
#[doc = "SpW Link 2 Distributed Interrupt Mask"]
pub mod link2_distintim;
#[doc = "LINK2_DISTINTPI_C (w) register accessor: SpW Link 2 Distributed Interrupt Clear Pending Interrupt\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`link2_distintpi_c::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@link2_distintpi_c`] module"]
#[doc(alias = "LINK2_DISTINTPI_C")]
pub type Link2DistintpiC = crate::Reg<link2_distintpi_c::Link2DistintpiCSpec>;
#[doc = "SpW Link 2 Distributed Interrupt Clear Pending Interrupt"]
pub mod link2_distintpi_c;
#[doc = "LINK2_DISTINTIM_S (w) register accessor: SpW Link 2 Distributed Interrupt Set Mask\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`link2_distintim_s::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@link2_distintim_s`] module"]
#[doc(alias = "LINK2_DISTINTIM_S")]
pub type Link2DistintimS = crate::Reg<link2_distintim_s::Link2DistintimSSpec>;
#[doc = "SpW Link 2 Distributed Interrupt Set Mask"]
pub mod link2_distintim_s;
#[doc = "LINK2_DISTINTIM_C (w) register accessor: SpW Link 2 Distributed Interrupt Clear Mask\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`link2_distintim_c::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@link2_distintim_c`] module"]
#[doc(alias = "LINK2_DISTINTIM_C")]
pub type Link2DistintimC = crate::Reg<link2_distintim_c::Link2DistintimCSpec>;
#[doc = "SpW Link 2 Distributed Interrupt Clear Mask"]
pub mod link2_distintim_c;
#[doc = "LINK2_DISTACKPI_RM (r) register accessor: SpW Link 2 Distributed Interrupt Acknowledge Pending Read Masked Interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`link2_distackpi_rm::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@link2_distackpi_rm`] module"]
#[doc(alias = "LINK2_DISTACKPI_RM")]
pub type Link2DistackpiRm = crate::Reg<link2_distackpi_rm::Link2DistackpiRmSpec>;
#[doc = "SpW Link 2 Distributed Interrupt Acknowledge Pending Read Masked Interrupt"]
pub mod link2_distackpi_rm;
#[doc = "LINK2_DISTACKPI_RCM (r) register accessor: SpW Link 2 Distributed Interrupt Acknowledge Pending Read and Clear Masked Interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`link2_distackpi_rcm::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@link2_distackpi_rcm`] module"]
#[doc(alias = "LINK2_DISTACKPI_RCM")]
pub type Link2DistackpiRcm = crate::Reg<link2_distackpi_rcm::Link2DistackpiRcmSpec>;
#[doc = "SpW Link 2 Distributed Interrupt Acknowledge Pending Read and Clear Masked Interrupt"]
pub mod link2_distackpi_rcm;
#[doc = "LINK2_DISTACKPI_R (r) register accessor: SpW Link 2 Distributed Interrupt Acknowledge Pending Read Interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`link2_distackpi_r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@link2_distackpi_r`] module"]
#[doc(alias = "LINK2_DISTACKPI_R")]
pub type Link2DistackpiR = crate::Reg<link2_distackpi_r::Link2DistackpiRSpec>;
#[doc = "SpW Link 2 Distributed Interrupt Acknowledge Pending Read Interrupt"]
pub mod link2_distackpi_r;
#[doc = "LINK2_DISTACKPI_RCS (rw) register accessor: SpW Link 2 Distributed Interrupt Acknowledge Pending Read and Clear Interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`link2_distackpi_rcs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`link2_distackpi_rcs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@link2_distackpi_rcs`] module"]
#[doc(alias = "LINK2_DISTACKPI_RCS")]
pub type Link2DistackpiRcs = crate::Reg<link2_distackpi_rcs::Link2DistackpiRcsSpec>;
#[doc = "SpW Link 2 Distributed Interrupt Acknowledge Pending Read and Clear Interrupt"]
pub mod link2_distackpi_rcs;
#[doc = "LINK2_DISTACKIM (rw) register accessor: SpW Link 2 Distributed Interrupt Acknowledge Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`link2_distackim::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`link2_distackim::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@link2_distackim`] module"]
#[doc(alias = "LINK2_DISTACKIM")]
pub type Link2Distackim = crate::Reg<link2_distackim::Link2DistackimSpec>;
#[doc = "SpW Link 2 Distributed Interrupt Acknowledge Mask"]
pub mod link2_distackim;
#[doc = "LINK2_DISTACKPI_C (w) register accessor: SpW Link 2 Distributed Interrupt Acknowledge Clear Pending Interrupt\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`link2_distackpi_c::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@link2_distackpi_c`] module"]
#[doc(alias = "LINK2_DISTACKPI_C")]
pub type Link2DistackpiC = crate::Reg<link2_distackpi_c::Link2DistackpiCSpec>;
#[doc = "SpW Link 2 Distributed Interrupt Acknowledge Clear Pending Interrupt"]
pub mod link2_distackpi_c;
#[doc = "LINK2_DISTACKIM_S (w) register accessor: SpW Link 2 Distributed Interrupt Acknowledge Set Mask\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`link2_distackim_s::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@link2_distackim_s`] module"]
#[doc(alias = "LINK2_DISTACKIM_S")]
pub type Link2DistackimS = crate::Reg<link2_distackim_s::Link2DistackimSSpec>;
#[doc = "SpW Link 2 Distributed Interrupt Acknowledge Set Mask"]
pub mod link2_distackim_s;
#[doc = "LINK2_DISTACKIM_C (w) register accessor: SpW Link 2 Distributed Interrupt Acknowledge Clear Mask\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`link2_distackim_c::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@link2_distackim_c`] module"]
#[doc(alias = "LINK2_DISTACKIM_C")]
pub type Link2DistackimC = crate::Reg<link2_distackim_c::Link2DistackimCSpec>;
#[doc = "SpW Link 2 Distributed Interrupt Acknowledge Clear Mask"]
pub mod link2_distackim_c;
#[doc = "PKTRX1_PI_RM (r) register accessor: PktRx Pending Read Masked Interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`pktrx1_pi_rm::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktrx1_pi_rm`] module"]
#[doc(alias = "PKTRX1_PI_RM")]
pub type Pktrx1PiRm = crate::Reg<pktrx1_pi_rm::Pktrx1PiRmSpec>;
#[doc = "PktRx Pending Read Masked Interrupt"]
pub mod pktrx1_pi_rm;
#[doc = "PKTRX1_PI_RCM (r) register accessor: PktRx Pending Read and Clear Masked Interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`pktrx1_pi_rcm::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktrx1_pi_rcm`] module"]
#[doc(alias = "PKTRX1_PI_RCM")]
pub type Pktrx1PiRcm = crate::Reg<pktrx1_pi_rcm::Pktrx1PiRcmSpec>;
#[doc = "PktRx Pending Read and Clear Masked Interrupt"]
pub mod pktrx1_pi_rcm;
#[doc = "PKTRX1_PI_R (r) register accessor: PktRx Pending Read Interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`pktrx1_pi_r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktrx1_pi_r`] module"]
#[doc(alias = "PKTRX1_PI_R")]
pub type Pktrx1PiR = crate::Reg<pktrx1_pi_r::Pktrx1PiRSpec>;
#[doc = "PktRx Pending Read Interrupt"]
pub mod pktrx1_pi_r;
#[doc = "PKTRX1_PI_RCS (rw) register accessor: PktRx Pending Read, Clear and Enabled Interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`pktrx1_pi_rcs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pktrx1_pi_rcs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktrx1_pi_rcs`] module"]
#[doc(alias = "PKTRX1_PI_RCS")]
pub type Pktrx1PiRcs = crate::Reg<pktrx1_pi_rcs::Pktrx1PiRcsSpec>;
#[doc = "PktRx Pending Read, Clear and Enabled Interrupt"]
pub mod pktrx1_pi_rcs;
#[doc = "PKTRX1_IM (rw) register accessor: PktRx Interrupt Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`pktrx1_im::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pktrx1_im::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktrx1_im`] module"]
#[doc(alias = "PKTRX1_IM")]
pub type Pktrx1Im = crate::Reg<pktrx1_im::Pktrx1ImSpec>;
#[doc = "PktRx Interrupt Mask"]
pub mod pktrx1_im;
#[doc = "PKTRX1_PI_C (w) register accessor: PktRx Clear Pending Interrupt\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pktrx1_pi_c::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktrx1_pi_c`] module"]
#[doc(alias = "PKTRX1_PI_C")]
pub type Pktrx1PiC = crate::Reg<pktrx1_pi_c::Pktrx1PiCSpec>;
#[doc = "PktRx Clear Pending Interrupt"]
pub mod pktrx1_pi_c;
#[doc = "PKTRX1_IM_S (w) register accessor: PktRx Interrupt Set Mask\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pktrx1_im_s::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktrx1_im_s`] module"]
#[doc(alias = "PKTRX1_IM_S")]
pub type Pktrx1ImS = crate::Reg<pktrx1_im_s::Pktrx1ImSSpec>;
#[doc = "PktRx Interrupt Set Mask"]
pub mod pktrx1_im_s;
#[doc = "PKTRX1_IM_C (w) register accessor: PktRx Interrupt Clear Mask\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pktrx1_im_c::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktrx1_im_c`] module"]
#[doc(alias = "PKTRX1_IM_C")]
pub type Pktrx1ImC = crate::Reg<pktrx1_im_c::Pktrx1ImCSpec>;
#[doc = "PktRx Interrupt Clear Mask"]
pub mod pktrx1_im_c;
#[doc = "PKTRX1_CFG (rw) register accessor: PktRx Config\n\nYou can [`read`](crate::Reg::read) this register and get [`pktrx1_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pktrx1_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktrx1_cfg`] module"]
#[doc(alias = "PKTRX1_CFG")]
pub type Pktrx1Cfg = crate::Reg<pktrx1_cfg::Pktrx1CfgSpec>;
#[doc = "PktRx Config"]
pub mod pktrx1_cfg;
#[doc = "PKTRX1_STATUS (r) register accessor: PktRx Status\n\nYou can [`read`](crate::Reg::read) this register and get [`pktrx1_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktrx1_status`] module"]
#[doc(alias = "PKTRX1_STATUS")]
pub type Pktrx1Status = crate::Reg<pktrx1_status::Pktrx1StatusSpec>;
#[doc = "PktRx Status"]
pub mod pktrx1_status;
#[doc = "PKTRX1_NXTBUFDATAADDR (rw) register accessor: PktRx Next Buffer Data Address\n\nYou can [`read`](crate::Reg::read) this register and get [`pktrx1_nxtbufdataaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pktrx1_nxtbufdataaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktrx1_nxtbufdataaddr`] module"]
#[doc(alias = "PKTRX1_NXTBUFDATAADDR")]
pub type Pktrx1Nxtbufdataaddr = crate::Reg<pktrx1_nxtbufdataaddr::Pktrx1NxtbufdataaddrSpec>;
#[doc = "PktRx Next Buffer Data Address"]
pub mod pktrx1_nxtbufdataaddr;
#[doc = "PKTRX1_NXTBUFDATALEN (rw) register accessor: PktRx Next Buffer Data Length\n\nYou can [`read`](crate::Reg::read) this register and get [`pktrx1_nxtbufdatalen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pktrx1_nxtbufdatalen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktrx1_nxtbufdatalen`] module"]
#[doc(alias = "PKTRX1_NXTBUFDATALEN")]
pub type Pktrx1Nxtbufdatalen = crate::Reg<pktrx1_nxtbufdatalen::Pktrx1NxtbufdatalenSpec>;
#[doc = "PktRx Next Buffer Data Length"]
pub mod pktrx1_nxtbufdatalen;
#[doc = "PKTRX1_NXTBUFPKTADDR (rw) register accessor: PktRx Next Buffer Packet Address\n\nYou can [`read`](crate::Reg::read) this register and get [`pktrx1_nxtbufpktaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pktrx1_nxtbufpktaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktrx1_nxtbufpktaddr`] module"]
#[doc(alias = "PKTRX1_NXTBUFPKTADDR")]
pub type Pktrx1Nxtbufpktaddr = crate::Reg<pktrx1_nxtbufpktaddr::Pktrx1NxtbufpktaddrSpec>;
#[doc = "PktRx Next Buffer Packet Address"]
pub mod pktrx1_nxtbufpktaddr;
#[doc = "PKTRX1_NXTBUFCFG (rw) register accessor: PktRx Next Buffer Config\n\nYou can [`read`](crate::Reg::read) this register and get [`pktrx1_nxtbufcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pktrx1_nxtbufcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktrx1_nxtbufcfg`] module"]
#[doc(alias = "PKTRX1_NXTBUFCFG")]
pub type Pktrx1Nxtbufcfg = crate::Reg<pktrx1_nxtbufcfg::Pktrx1NxtbufcfgSpec>;
#[doc = "PktRx Next Buffer Config"]
pub mod pktrx1_nxtbufcfg;
#[doc = "PKTRX1_CURBUFDATAADDR (r) register accessor: PktRx Current Buffer Data Address\n\nYou can [`read`](crate::Reg::read) this register and get [`pktrx1_curbufdataaddr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktrx1_curbufdataaddr`] module"]
#[doc(alias = "PKTRX1_CURBUFDATAADDR")]
pub type Pktrx1Curbufdataaddr = crate::Reg<pktrx1_curbufdataaddr::Pktrx1CurbufdataaddrSpec>;
#[doc = "PktRx Current Buffer Data Address"]
pub mod pktrx1_curbufdataaddr;
#[doc = "PKTRX1_CURBUFDATALEN (r) register accessor: PktRx Current Buffer Data Length\n\nYou can [`read`](crate::Reg::read) this register and get [`pktrx1_curbufdatalen::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktrx1_curbufdatalen`] module"]
#[doc(alias = "PKTRX1_CURBUFDATALEN")]
pub type Pktrx1Curbufdatalen = crate::Reg<pktrx1_curbufdatalen::Pktrx1CurbufdatalenSpec>;
#[doc = "PktRx Current Buffer Data Length"]
pub mod pktrx1_curbufdatalen;
#[doc = "PKTRX1_CURBUFPKTADDR (r) register accessor: PktRx Current Buffer Packet Address\n\nYou can [`read`](crate::Reg::read) this register and get [`pktrx1_curbufpktaddr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktrx1_curbufpktaddr`] module"]
#[doc(alias = "PKTRX1_CURBUFPKTADDR")]
pub type Pktrx1Curbufpktaddr = crate::Reg<pktrx1_curbufpktaddr::Pktrx1CurbufpktaddrSpec>;
#[doc = "PktRx Current Buffer Packet Address"]
pub mod pktrx1_curbufpktaddr;
#[doc = "PKTRX1_CURBUFCFG (rw) register accessor: PktRx Current Buffer Config\n\nYou can [`read`](crate::Reg::read) this register and get [`pktrx1_curbufcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pktrx1_curbufcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktrx1_curbufcfg`] module"]
#[doc(alias = "PKTRX1_CURBUFCFG")]
pub type Pktrx1Curbufcfg = crate::Reg<pktrx1_curbufcfg::Pktrx1CurbufcfgSpec>;
#[doc = "PktRx Current Buffer Config"]
pub mod pktrx1_curbufcfg;
#[doc = "PKTRX1_PREVBUFDATALEN (r) register accessor: PktRx Previous Buffer Data Length\n\nYou can [`read`](crate::Reg::read) this register and get [`pktrx1_prevbufdatalen::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktrx1_prevbufdatalen`] module"]
#[doc(alias = "PKTRX1_PREVBUFDATALEN")]
pub type Pktrx1Prevbufdatalen = crate::Reg<pktrx1_prevbufdatalen::Pktrx1PrevbufdatalenSpec>;
#[doc = "PktRx Previous Buffer Data Length"]
pub mod pktrx1_prevbufdatalen;
#[doc = "PKTRX1_PREVBUFSTS (r) register accessor: PktRx Previous Buffer Status\n\nYou can [`read`](crate::Reg::read) this register and get [`pktrx1_prevbufsts::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktrx1_prevbufsts`] module"]
#[doc(alias = "PKTRX1_PREVBUFSTS")]
pub type Pktrx1Prevbufsts = crate::Reg<pktrx1_prevbufsts::Pktrx1PrevbufstsSpec>;
#[doc = "PktRx Previous Buffer Status"]
pub mod pktrx1_prevbufsts;
#[doc = "PKTRX1_SWRESET (rw) register accessor: PktRx Software Reset\n\nYou can [`read`](crate::Reg::read) this register and get [`pktrx1_swreset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pktrx1_swreset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktrx1_swreset`] module"]
#[doc(alias = "PKTRX1_SWRESET")]
pub type Pktrx1Swreset = crate::Reg<pktrx1_swreset::Pktrx1SwresetSpec>;
#[doc = "PktRx Software Reset"]
pub mod pktrx1_swreset;
#[doc = "PKTTX1_PI_RM (r) register accessor: PktTx Pending Read Masked Interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`pkttx1_pi_rm::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pkttx1_pi_rm`] module"]
#[doc(alias = "PKTTX1_PI_RM")]
pub type Pkttx1PiRm = crate::Reg<pkttx1_pi_rm::Pkttx1PiRmSpec>;
#[doc = "PktTx Pending Read Masked Interrupt"]
pub mod pkttx1_pi_rm;
#[doc = "PKTTX1_PI_RCM (r) register accessor: PktTx Pending Read and Clear Masked Interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`pkttx1_pi_rcm::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pkttx1_pi_rcm`] module"]
#[doc(alias = "PKTTX1_PI_RCM")]
pub type Pkttx1PiRcm = crate::Reg<pkttx1_pi_rcm::Pkttx1PiRcmSpec>;
#[doc = "PktTx Pending Read and Clear Masked Interrupt"]
pub mod pkttx1_pi_rcm;
#[doc = "PKTTX1_PI_R (r) register accessor: PktTx Pending Read Interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`pkttx1_pi_r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pkttx1_pi_r`] module"]
#[doc(alias = "PKTTX1_PI_R")]
pub type Pkttx1PiR = crate::Reg<pkttx1_pi_r::Pkttx1PiRSpec>;
#[doc = "PktTx Pending Read Interrupt"]
pub mod pkttx1_pi_r;
#[doc = "PKTTX1_PI_RCS (rw) register accessor: PktTx Pending Read, Clear and Enabled Interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`pkttx1_pi_rcs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pkttx1_pi_rcs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pkttx1_pi_rcs`] module"]
#[doc(alias = "PKTTX1_PI_RCS")]
pub type Pkttx1PiRcs = crate::Reg<pkttx1_pi_rcs::Pkttx1PiRcsSpec>;
#[doc = "PktTx Pending Read, Clear and Enabled Interrupt"]
pub mod pkttx1_pi_rcs;
#[doc = "PKTTX1_IM (rw) register accessor: PktTx Interrupt Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`pkttx1_im::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pkttx1_im::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pkttx1_im`] module"]
#[doc(alias = "PKTTX1_IM")]
pub type Pkttx1Im = crate::Reg<pkttx1_im::Pkttx1ImSpec>;
#[doc = "PktTx Interrupt Mask"]
pub mod pkttx1_im;
#[doc = "PKTTX1_PI_C (w) register accessor: PktTx Clear Pending Interrupt\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pkttx1_pi_c::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pkttx1_pi_c`] module"]
#[doc(alias = "PKTTX1_PI_C")]
pub type Pkttx1PiC = crate::Reg<pkttx1_pi_c::Pkttx1PiCSpec>;
#[doc = "PktTx Clear Pending Interrupt"]
pub mod pkttx1_pi_c;
#[doc = "PKTTX1_IM_S (w) register accessor: PktTx Interrupt Set Mask\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pkttx1_im_s::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pkttx1_im_s`] module"]
#[doc(alias = "PKTTX1_IM_S")]
pub type Pkttx1ImS = crate::Reg<pkttx1_im_s::Pkttx1ImSSpec>;
#[doc = "PktTx Interrupt Set Mask"]
pub mod pkttx1_im_s;
#[doc = "PKTTX1_IM_C (w) register accessor: PktTx Interrupt Clear Mask\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pkttx1_im_c::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pkttx1_im_c`] module"]
#[doc(alias = "PKTTX1_IM_C")]
pub type Pkttx1ImC = crate::Reg<pkttx1_im_c::Pkttx1ImCSpec>;
#[doc = "PktTx Interrupt Clear Mask"]
pub mod pkttx1_im_c;
#[doc = "PKTTX1_STATUS (rw) register accessor: PktTx Status\n\nYou can [`read`](crate::Reg::read) this register and get [`pkttx1_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pkttx1_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pkttx1_status`] module"]
#[doc(alias = "PKTTX1_STATUS")]
pub type Pkttx1Status = crate::Reg<pkttx1_status::Pkttx1StatusSpec>;
#[doc = "PktTx Status"]
pub mod pkttx1_status;
#[doc = "PKTTX1_NXTSENDROUT (rw) register accessor: PktTx Next Send List Router Bytes\n\nYou can [`read`](crate::Reg::read) this register and get [`pkttx1_nxtsendrout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pkttx1_nxtsendrout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pkttx1_nxtsendrout`] module"]
#[doc(alias = "PKTTX1_NXTSENDROUT")]
pub type Pkttx1Nxtsendrout = crate::Reg<pkttx1_nxtsendrout::Pkttx1NxtsendroutSpec>;
#[doc = "PktTx Next Send List Router Bytes"]
pub mod pkttx1_nxtsendrout;
#[doc = "PKTTX1_NXTSENDADDR (rw) register accessor: PktTx Next Send List Address\n\nYou can [`read`](crate::Reg::read) this register and get [`pkttx1_nxtsendaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pkttx1_nxtsendaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pkttx1_nxtsendaddr`] module"]
#[doc(alias = "PKTTX1_NXTSENDADDR")]
pub type Pkttx1Nxtsendaddr = crate::Reg<pkttx1_nxtsendaddr::Pkttx1NxtsendaddrSpec>;
#[doc = "PktTx Next Send List Address"]
pub mod pkttx1_nxtsendaddr;
#[doc = "PKTTX1_NXTSENDCFG (rw) register accessor: PktTx Next Send List Config\n\nYou can [`read`](crate::Reg::read) this register and get [`pkttx1_nxtsendcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pkttx1_nxtsendcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pkttx1_nxtsendcfg`] module"]
#[doc(alias = "PKTTX1_NXTSENDCFG")]
pub type Pkttx1Nxtsendcfg = crate::Reg<pkttx1_nxtsendcfg::Pkttx1NxtsendcfgSpec>;
#[doc = "PktTx Next Send List Config"]
pub mod pkttx1_nxtsendcfg;
#[doc = "PKTTX1_CURSENDROUT (r) register accessor: PktTx Current Send List Router Bytes\n\nYou can [`read`](crate::Reg::read) this register and get [`pkttx1_cursendrout::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pkttx1_cursendrout`] module"]
#[doc(alias = "PKTTX1_CURSENDROUT")]
pub type Pkttx1Cursendrout = crate::Reg<pkttx1_cursendrout::Pkttx1CursendroutSpec>;
#[doc = "PktTx Current Send List Router Bytes"]
pub mod pkttx1_cursendrout;
#[doc = "PKTTX1_CURSENDADDR (r) register accessor: PktTx Current Send List Address\n\nYou can [`read`](crate::Reg::read) this register and get [`pkttx1_cursendaddr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pkttx1_cursendaddr`] module"]
#[doc(alias = "PKTTX1_CURSENDADDR")]
pub type Pkttx1Cursendaddr = crate::Reg<pkttx1_cursendaddr::Pkttx1CursendaddrSpec>;
#[doc = "PktTx Current Send List Address"]
pub mod pkttx1_cursendaddr;
#[doc = "PKTTX1_CURSENDCFG (rw) register accessor: PktTx Current Send List Config\n\nYou can [`read`](crate::Reg::read) this register and get [`pkttx1_cursendcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pkttx1_cursendcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pkttx1_cursendcfg`] module"]
#[doc(alias = "PKTTX1_CURSENDCFG")]
pub type Pkttx1Cursendcfg = crate::Reg<pkttx1_cursendcfg::Pkttx1CursendcfgSpec>;
#[doc = "PktTx Current Send List Config"]
pub mod pkttx1_cursendcfg;
#[doc = "PKTTX1_SWRESET (rw) register accessor: PktTx Software Reset\n\nYou can [`read`](crate::Reg::read) this register and get [`pkttx1_swreset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pkttx1_swreset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pkttx1_swreset`] module"]
#[doc(alias = "PKTTX1_SWRESET")]
pub type Pkttx1Swreset = crate::Reg<pkttx1_swreset::Pkttx1SwresetSpec>;
#[doc = "PktTx Software Reset"]
pub mod pkttx1_swreset;
#[doc = "RMAP1_CFG (rw) register accessor: SpW RMAP 1 Config\n\nYou can [`read`](crate::Reg::read) this register and get [`rmap1_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rmap1_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rmap1_cfg`] module"]
#[doc(alias = "RMAP1_CFG")]
pub type Rmap1Cfg = crate::Reg<rmap1_cfg::Rmap1CfgSpec>;
#[doc = "SpW RMAP 1 Config"]
pub mod rmap1_cfg;
#[doc = "RMAP1_STS_RC (r) register accessor: SpW RMAP 1 Read and Clear Status\n\nYou can [`read`](crate::Reg::read) this register and get [`rmap1_sts_rc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rmap1_sts_rc`] module"]
#[doc(alias = "RMAP1_STS_RC")]
pub type Rmap1StsRc = crate::Reg<rmap1_sts_rc::Rmap1StsRcSpec>;
#[doc = "SpW RMAP 1 Read and Clear Status"]
pub mod rmap1_sts_rc;
#[doc = "RMAP1_STS (r) register accessor: SpW RMAP 1 Read Status\n\nYou can [`read`](crate::Reg::read) this register and get [`rmap1_sts::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rmap1_sts`] module"]
#[doc(alias = "RMAP1_STS")]
pub type Rmap1Sts = crate::Reg<rmap1_sts::Rmap1StsSpec>;
#[doc = "SpW RMAP 1 Read Status"]
pub mod rmap1_sts;
#[doc = "TCH_PI_RM (r) register accessor: SpW Tch Pending Read Masked Interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`tch_pi_rm::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tch_pi_rm`] module"]
#[doc(alias = "TCH_PI_RM")]
pub type TchPiRm = crate::Reg<tch_pi_rm::TchPiRmSpec>;
#[doc = "SpW Tch Pending Read Masked Interrupt"]
pub mod tch_pi_rm;
#[doc = "TCH_PI_RCM (r) register accessor: SpW Tch Pending Read and Clear Masked Interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`tch_pi_rcm::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tch_pi_rcm`] module"]
#[doc(alias = "TCH_PI_RCM")]
pub type TchPiRcm = crate::Reg<tch_pi_rcm::TchPiRcmSpec>;
#[doc = "SpW Tch Pending Read and Clear Masked Interrupt"]
pub mod tch_pi_rcm;
#[doc = "TCH_PI_R (r) register accessor: SpW Tch Pending Read Interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`tch_pi_r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tch_pi_r`] module"]
#[doc(alias = "TCH_PI_R")]
pub type TchPiR = crate::Reg<tch_pi_r::TchPiRSpec>;
#[doc = "SpW Tch Pending Read Interrupt"]
pub mod tch_pi_r;
#[doc = "TCH_PI_RCS (rw) register accessor: SpW Tch Pending Read, Clear and Enabled Interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`tch_pi_rcs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tch_pi_rcs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tch_pi_rcs`] module"]
#[doc(alias = "TCH_PI_RCS")]
pub type TchPiRcs = crate::Reg<tch_pi_rcs::TchPiRcsSpec>;
#[doc = "SpW Tch Pending Read, Clear and Enabled Interrupt"]
pub mod tch_pi_rcs;
#[doc = "TCH_IM (rw) register accessor: SpW Tch Interrupt Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`tch_im::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tch_im::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tch_im`] module"]
#[doc(alias = "TCH_IM")]
pub type TchIm = crate::Reg<tch_im::TchImSpec>;
#[doc = "SpW Tch Interrupt Mask"]
pub mod tch_im;
#[doc = "TCH_PI_C (w) register accessor: SpW Tch Clear Pending Interrupt\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tch_pi_c::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tch_pi_c`] module"]
#[doc(alias = "TCH_PI_C")]
pub type TchPiC = crate::Reg<tch_pi_c::TchPiCSpec>;
#[doc = "SpW Tch Clear Pending Interrupt"]
pub mod tch_pi_c;
#[doc = "TCH_IM_S (w) register accessor: SpW Tch Interrupt Set Mask\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tch_im_s::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tch_im_s`] module"]
#[doc(alias = "TCH_IM_S")]
pub type TchImS = crate::Reg<tch_im_s::TchImSSpec>;
#[doc = "SpW Tch Interrupt Set Mask"]
pub mod tch_im_s;
#[doc = "TCH_IM_C (w) register accessor: SpW Tch Interrupt Clear Mask\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tch_im_c::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tch_im_c`] module"]
#[doc(alias = "TCH_IM_C")]
pub type TchImC = crate::Reg<tch_im_c::TchImCSpec>;
#[doc = "SpW Tch Interrupt Clear Mask"]
pub mod tch_im_c;
#[doc = "TCH_CFGLISTEN (rw) register accessor: SpW Tch Config Listener\n\nYou can [`read`](crate::Reg::read) this register and get [`tch_cfglisten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tch_cfglisten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tch_cfglisten`] module"]
#[doc(alias = "TCH_CFGLISTEN")]
pub type TchCfglisten = crate::Reg<tch_cfglisten::TchCfglistenSpec>;
#[doc = "SpW Tch Config Listener"]
pub mod tch_cfglisten;
#[doc = "TCH_CFGSEND (rw) register accessor: SpW Tch Config Sender\n\nYou can [`read`](crate::Reg::read) this register and get [`tch_cfgsend::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tch_cfgsend::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tch_cfgsend`] module"]
#[doc(alias = "TCH_CFGSEND")]
pub type TchCfgsend = crate::Reg<tch_cfgsend::TchCfgsendSpec>;
#[doc = "SpW Tch Config Sender"]
pub mod tch_cfgsend;
#[doc = "TCH_CFG (rw) register accessor: SpW Tch Config\n\nYou can [`read`](crate::Reg::read) this register and get [`tch_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tch_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tch_cfg`] module"]
#[doc(alias = "TCH_CFG")]
pub type TchCfg = crate::Reg<tch_cfg::TchCfgSpec>;
#[doc = "SpW Tch Config"]
pub mod tch_cfg;
#[doc = "TCH_CFGRESTART (rw) register accessor: SpW Tch Config Restart\n\nYou can [`read`](crate::Reg::read) this register and get [`tch_cfgrestart::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tch_cfgrestart::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tch_cfgrestart`] module"]
#[doc(alias = "TCH_CFGRESTART")]
pub type TchCfgrestart = crate::Reg<tch_cfgrestart::TchCfgrestartSpec>;
#[doc = "SpW Tch Config Restart"]
pub mod tch_cfgrestart;
#[doc = "TCH_CFGTCEVENT (rw) register accessor: SpW Tch Config Tc Event\n\nYou can [`read`](crate::Reg::read) this register and get [`tch_cfgtcevent::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tch_cfgtcevent::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tch_cfgtcevent`] module"]
#[doc(alias = "TCH_CFGTCEVENT")]
pub type TchCfgtcevent = crate::Reg<tch_cfgtcevent::TchCfgtceventSpec>;
#[doc = "SpW Tch Config Tc Event"]
pub mod tch_cfgtcevent;
#[doc = "TCH_CFGWD (rw) register accessor: SpW Tch Config Watchdog\n\nYou can [`read`](crate::Reg::read) this register and get [`tch_cfgwd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tch_cfgwd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tch_cfgwd`] module"]
#[doc(alias = "TCH_CFGWD")]
pub type TchCfgwd = crate::Reg<tch_cfgwd::TchCfgwdSpec>;
#[doc = "SpW Tch Config Watchdog"]
pub mod tch_cfgwd;
#[doc = "TCH_LASTTIMECODE (rw) register accessor: SpW Tch Last Time Code\n\nYou can [`read`](crate::Reg::read) this register and get [`tch_lasttimecode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tch_lasttimecode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tch_lasttimecode`] module"]
#[doc(alias = "TCH_LASTTIMECODE")]
pub type TchLasttimecode = crate::Reg<tch_lasttimecode::TchLasttimecodeSpec>;
#[doc = "SpW Tch Last Time Code"]
pub mod tch_lasttimecode;
#[doc = "TCH_SWRESET (rw) register accessor: SpW Tch Software Reset\n\nYou can [`read`](crate::Reg::read) this register and get [`tch_swreset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tch_swreset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tch_swreset`] module"]
#[doc(alias = "TCH_SWRESET")]
pub type TchSwreset = crate::Reg<tch_swreset::TchSwresetSpec>;
#[doc = "SpW Tch Software Reset"]
pub mod tch_swreset;
#[doc = "GROUP_IRQSTS1 (r) register accessor: SpW Group Interrupt status 1\n\nYou can [`read`](crate::Reg::read) this register and get [`group_irqsts1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group_irqsts1`] module"]
#[doc(alias = "GROUP_IRQSTS1")]
pub type GroupIrqsts1 = crate::Reg<group_irqsts1::GroupIrqsts1Spec>;
#[doc = "SpW Group Interrupt status 1"]
pub mod group_irqsts1;
#[doc = "GROUP_IRQSTS2 (r) register accessor: SpW Group Interrupt status 2\n\nYou can [`read`](crate::Reg::read) this register and get [`group_irqsts2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group_irqsts2`] module"]
#[doc(alias = "GROUP_IRQSTS2")]
pub type GroupIrqsts2 = crate::Reg<group_irqsts2::GroupIrqsts2Spec>;
#[doc = "SpW Group Interrupt status 2"]
pub mod group_irqsts2;
#[doc = "GROUP_EDACSTS (r) register accessor: SpW Group Interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`group_edacsts::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@group_edacsts`] module"]
#[doc(alias = "GROUP_EDACSTS")]
pub type GroupEdacsts = crate::Reg<group_edacsts::GroupEdacstsSpec>;
#[doc = "SpW Group Interrupt status"]
pub mod group_edacsts;
