#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ncr: Ncr,
    ncfgr: Ncfgr,
    nsr: Nsr,
    ur: Ur,
    dcfgr: Dcfgr,
    tsr: Tsr,
    rbqb: Rbqb,
    tbqb: Tbqb,
    rsr: Rsr,
    isr: Isr,
    ier: Ier,
    idr: Idr,
    imr: Imr,
    man: Man,
    rpq: Rpq,
    tpq: Tpq,
    tpsf: Tpsf,
    rpsf: Rpsf,
    rjfml: Rjfml,
    _reserved19: [u8; 0x34],
    hrb: Hrb,
    hrt: Hrt,
    gmac_sa: [GmacSa; 4],
    tidm1: Tidm1,
    tidm2: Tidm2,
    tidm3: Tidm3,
    tidm4: Tidm4,
    _reserved26: [u8; 0x04],
    ipgs: Ipgs,
    svlan: Svlan,
    tpfcp: Tpfcp,
    samb1: Samb1,
    samt1: Samt1,
    _reserved31: [u8; 0x0c],
    nsc: Nsc,
    scl: Scl,
    sch: Sch,
    eftsh: Eftsh,
    efrsh: Efrsh,
    peftsh: Peftsh,
    pefrsh: Pefrsh,
    _reserved38: [u8; 0x04],
    mid: Mid,
    otlo: Otlo,
    othi: Othi,
    ft: Ft,
    bcft: Bcft,
    mft: Mft,
    pft: Pft,
    bft64: Bft64,
    tbft127: Tbft127,
    tbft255: Tbft255,
    tbft511: Tbft511,
    tbft1023: Tbft1023,
    tbft1518: Tbft1518,
    gtbft1518: Gtbft1518,
    tur: Tur,
    scf: Scf,
    mcf: Mcf,
    ec: Ec,
    lc: Lc,
    dtf: Dtf,
    cse: Cse,
    orlo: Orlo,
    orhi: Orhi,
    fr: Fr,
    bcfr: Bcfr,
    mfr: Mfr,
    pfr: Pfr,
    bfr64: Bfr64,
    tbfr127: Tbfr127,
    tbfr255: Tbfr255,
    tbfr511: Tbfr511,
    tbfr1023: Tbfr1023,
    tbfr1518: Tbfr1518,
    tmxbfr: Tmxbfr,
    ufr: Ufr,
    ofr: Ofr,
    jr: Jr,
    fcse: Fcse,
    lffe: Lffe,
    rse: Rse,
    ae: Ae,
    rre: Rre,
    roe: Roe,
    ihce: Ihce,
    tce: Tce,
    uce: Uce,
    _reserved84: [u8; 0x08],
    tisubn: Tisubn,
    tsh: Tsh,
    _reserved86: [u8; 0x0c],
    tsl: Tsl,
    tn: Tn,
    ta: Ta,
    ti: Ti,
    eftsl: Eftsl,
    eftn: Eftn,
    efrsl: Efrsl,
    efrn: Efrn,
    peftsl: Peftsl,
    peftn: Peftn,
    pefrsl: Pefrsl,
    pefrn: Pefrn,
    _reserved98: [u8; 0x70],
    rxlpi: Rxlpi,
    rxlpitime: Rxlpitime,
    txlpi: Txlpi,
    txlpitime: Txlpitime,
    _reserved102: [u8; 0x0180],
    isrpq: [Isrpq; 5],
    _reserved103: [u8; 0x2c],
    tbqbapq: [Tbqbapq; 5],
    _reserved104: [u8; 0x2c],
    rbqbapq: [Rbqbapq; 5],
    _reserved105: [u8; 0x0c],
    rbsrpq: [Rbsrpq; 5],
    _reserved106: [u8; 0x08],
    cbscr: Cbscr,
    cbsisqa: Cbsisqa,
    cbsisqb: Cbsisqb,
    _reserved109: [u8; 0x38],
    st1rpq: [St1rpq; 4],
    _reserved110: [u8; 0x30],
    st2rpq: [St2rpq; 8],
    _reserved111: [u8; 0xa0],
    ierpq: [Ierpq; 5],
    _reserved112: [u8; 0x0c],
    idrpq: [Idrpq; 5],
    _reserved113: [u8; 0x0c],
    imrpq: [Imrpq; 5],
    _reserved114: [u8; 0x8c],
    st2er: [St2er; 4],
    _reserved115: [u8; 0x10],
    gmac_st2cw: [GmacSt2cw; 24],
}
impl RegisterBlock {
    #[doc = "0x00 - Network Control Register"]
    #[inline(always)]
    pub const fn ncr(&self) -> &Ncr {
        &self.ncr
    }
    #[doc = "0x04 - Network Configuration Register"]
    #[inline(always)]
    pub const fn ncfgr(&self) -> &Ncfgr {
        &self.ncfgr
    }
    #[doc = "0x08 - Network Status Register"]
    #[inline(always)]
    pub const fn nsr(&self) -> &Nsr {
        &self.nsr
    }
    #[doc = "0x0c - User Register"]
    #[inline(always)]
    pub const fn ur(&self) -> &Ur {
        &self.ur
    }
    #[doc = "0x10 - DMA Configuration Register"]
    #[inline(always)]
    pub const fn dcfgr(&self) -> &Dcfgr {
        &self.dcfgr
    }
    #[doc = "0x14 - Transmit Status Register"]
    #[inline(always)]
    pub const fn tsr(&self) -> &Tsr {
        &self.tsr
    }
    #[doc = "0x18 - Receive Buffer Queue Base Address Register"]
    #[inline(always)]
    pub const fn rbqb(&self) -> &Rbqb {
        &self.rbqb
    }
    #[doc = "0x1c - Transmit Buffer Queue Base Address Register"]
    #[inline(always)]
    pub const fn tbqb(&self) -> &Tbqb {
        &self.tbqb
    }
    #[doc = "0x20 - Receive Status Register"]
    #[inline(always)]
    pub const fn rsr(&self) -> &Rsr {
        &self.rsr
    }
    #[doc = "0x24 - Interrupt Status Register"]
    #[inline(always)]
    pub const fn isr(&self) -> &Isr {
        &self.isr
    }
    #[doc = "0x28 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ier(&self) -> &Ier {
        &self.ier
    }
    #[doc = "0x2c - Interrupt Disable Register"]
    #[inline(always)]
    pub const fn idr(&self) -> &Idr {
        &self.idr
    }
    #[doc = "0x30 - Interrupt Mask Register"]
    #[inline(always)]
    pub const fn imr(&self) -> &Imr {
        &self.imr
    }
    #[doc = "0x34 - PHY Maintenance Register"]
    #[inline(always)]
    pub const fn man(&self) -> &Man {
        &self.man
    }
    #[doc = "0x38 - Received Pause Quantum Register"]
    #[inline(always)]
    pub const fn rpq(&self) -> &Rpq {
        &self.rpq
    }
    #[doc = "0x3c - Transmit Pause Quantum Register"]
    #[inline(always)]
    pub const fn tpq(&self) -> &Tpq {
        &self.tpq
    }
    #[doc = "0x40 - TX Partial Store and Forward Register"]
    #[inline(always)]
    pub const fn tpsf(&self) -> &Tpsf {
        &self.tpsf
    }
    #[doc = "0x44 - RX Partial Store and Forward Register"]
    #[inline(always)]
    pub const fn rpsf(&self) -> &Rpsf {
        &self.rpsf
    }
    #[doc = "0x48 - RX Jumbo Frame Max Length Register"]
    #[inline(always)]
    pub const fn rjfml(&self) -> &Rjfml {
        &self.rjfml
    }
    #[doc = "0x80 - Hash Register Bottom"]
    #[inline(always)]
    pub const fn hrb(&self) -> &Hrb {
        &self.hrb
    }
    #[doc = "0x84 - Hash Register Top"]
    #[inline(always)]
    pub const fn hrt(&self) -> &Hrt {
        &self.hrt
    }
    #[doc = "0x88..0xa8 - Specific Address 1 Bottom Register"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is the index of cluster in the array. `n == 0` corresponds to `GMAC_SA1` cluster.</div>"]
    #[inline(always)]
    pub const fn gmac_sa(&self, n: usize) -> &GmacSa {
        &self.gmac_sa[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x88..0xa8 - Specific Address 1 Bottom Register"]
    #[inline(always)]
    pub fn gmac_sa_iter(&self) -> impl Iterator<Item = &GmacSa> {
        self.gmac_sa.iter()
    }
    #[doc = "0x88..0x90 - Specific Address 1 Bottom Register"]
    #[inline(always)]
    pub const fn gmac_sa1(&self) -> &GmacSa {
        self.gmac_sa(0)
    }
    #[doc = "0x90..0x98 - Specific Address 1 Bottom Register"]
    #[inline(always)]
    pub const fn gmac_sa2(&self) -> &GmacSa {
        self.gmac_sa(1)
    }
    #[doc = "0x98..0xa0 - Specific Address 1 Bottom Register"]
    #[inline(always)]
    pub const fn gmac_sa3(&self) -> &GmacSa {
        self.gmac_sa(2)
    }
    #[doc = "0xa0..0xa8 - Specific Address 1 Bottom Register"]
    #[inline(always)]
    pub const fn gmac_sa4(&self) -> &GmacSa {
        self.gmac_sa(3)
    }
    #[doc = "0xa8 - Type ID Match 1 Register"]
    #[inline(always)]
    pub const fn tidm1(&self) -> &Tidm1 {
        &self.tidm1
    }
    #[doc = "0xac - Type ID Match 2 Register"]
    #[inline(always)]
    pub const fn tidm2(&self) -> &Tidm2 {
        &self.tidm2
    }
    #[doc = "0xb0 - Type ID Match 3 Register"]
    #[inline(always)]
    pub const fn tidm3(&self) -> &Tidm3 {
        &self.tidm3
    }
    #[doc = "0xb4 - Type ID Match 4 Register"]
    #[inline(always)]
    pub const fn tidm4(&self) -> &Tidm4 {
        &self.tidm4
    }
    #[doc = "0xbc - IPG Stretch Register"]
    #[inline(always)]
    pub const fn ipgs(&self) -> &Ipgs {
        &self.ipgs
    }
    #[doc = "0xc0 - Stacked VLAN Register"]
    #[inline(always)]
    pub const fn svlan(&self) -> &Svlan {
        &self.svlan
    }
    #[doc = "0xc4 - Transmit PFC Pause Register"]
    #[inline(always)]
    pub const fn tpfcp(&self) -> &Tpfcp {
        &self.tpfcp
    }
    #[doc = "0xc8 - Specific Address 1 Mask Bottom Register"]
    #[inline(always)]
    pub const fn samb1(&self) -> &Samb1 {
        &self.samb1
    }
    #[doc = "0xcc - Specific Address 1 Mask Top Register"]
    #[inline(always)]
    pub const fn samt1(&self) -> &Samt1 {
        &self.samt1
    }
    #[doc = "0xdc - 1588 Timer Nanosecond Comparison Register"]
    #[inline(always)]
    pub const fn nsc(&self) -> &Nsc {
        &self.nsc
    }
    #[doc = "0xe0 - 1588 Timer Second Comparison Low Register"]
    #[inline(always)]
    pub const fn scl(&self) -> &Scl {
        &self.scl
    }
    #[doc = "0xe4 - 1588 Timer Second Comparison High Register"]
    #[inline(always)]
    pub const fn sch(&self) -> &Sch {
        &self.sch
    }
    #[doc = "0xe8 - PTP Event Frame Transmitted Seconds High Register"]
    #[inline(always)]
    pub const fn eftsh(&self) -> &Eftsh {
        &self.eftsh
    }
    #[doc = "0xec - PTP Event Frame Received Seconds High Register"]
    #[inline(always)]
    pub const fn efrsh(&self) -> &Efrsh {
        &self.efrsh
    }
    #[doc = "0xf0 - PTP Peer Event Frame Transmitted Seconds High Register"]
    #[inline(always)]
    pub const fn peftsh(&self) -> &Peftsh {
        &self.peftsh
    }
    #[doc = "0xf4 - PTP Peer Event Frame Received Seconds High Register"]
    #[inline(always)]
    pub const fn pefrsh(&self) -> &Pefrsh {
        &self.pefrsh
    }
    #[doc = "0xfc - Module ID Register"]
    #[inline(always)]
    pub const fn mid(&self) -> &Mid {
        &self.mid
    }
    #[doc = "0x100 - Octets Transmitted Low Register"]
    #[inline(always)]
    pub const fn otlo(&self) -> &Otlo {
        &self.otlo
    }
    #[doc = "0x104 - Octets Transmitted High Register"]
    #[inline(always)]
    pub const fn othi(&self) -> &Othi {
        &self.othi
    }
    #[doc = "0x108 - Frames Transmitted Register"]
    #[inline(always)]
    pub const fn ft(&self) -> &Ft {
        &self.ft
    }
    #[doc = "0x10c - Broadcast Frames Transmitted Register"]
    #[inline(always)]
    pub const fn bcft(&self) -> &Bcft {
        &self.bcft
    }
    #[doc = "0x110 - Multicast Frames Transmitted Register"]
    #[inline(always)]
    pub const fn mft(&self) -> &Mft {
        &self.mft
    }
    #[doc = "0x114 - Pause Frames Transmitted Register"]
    #[inline(always)]
    pub const fn pft(&self) -> &Pft {
        &self.pft
    }
    #[doc = "0x118 - 64 Byte Frames Transmitted Register"]
    #[inline(always)]
    pub const fn bft64(&self) -> &Bft64 {
        &self.bft64
    }
    #[doc = "0x11c - 65 to 127 Byte Frames Transmitted Register"]
    #[inline(always)]
    pub const fn tbft127(&self) -> &Tbft127 {
        &self.tbft127
    }
    #[doc = "0x120 - 128 to 255 Byte Frames Transmitted Register"]
    #[inline(always)]
    pub const fn tbft255(&self) -> &Tbft255 {
        &self.tbft255
    }
    #[doc = "0x124 - 256 to 511 Byte Frames Transmitted Register"]
    #[inline(always)]
    pub const fn tbft511(&self) -> &Tbft511 {
        &self.tbft511
    }
    #[doc = "0x128 - 512 to 1023 Byte Frames Transmitted Register"]
    #[inline(always)]
    pub const fn tbft1023(&self) -> &Tbft1023 {
        &self.tbft1023
    }
    #[doc = "0x12c - 1024 to 1518 Byte Frames Transmitted Register"]
    #[inline(always)]
    pub const fn tbft1518(&self) -> &Tbft1518 {
        &self.tbft1518
    }
    #[doc = "0x130 - Greater Than 1518 Byte Frames Transmitted Register"]
    #[inline(always)]
    pub const fn gtbft1518(&self) -> &Gtbft1518 {
        &self.gtbft1518
    }
    #[doc = "0x134 - Transmit Underruns Register"]
    #[inline(always)]
    pub const fn tur(&self) -> &Tur {
        &self.tur
    }
    #[doc = "0x138 - Single Collision Frames Register"]
    #[inline(always)]
    pub const fn scf(&self) -> &Scf {
        &self.scf
    }
    #[doc = "0x13c - Multiple Collision Frames Register"]
    #[inline(always)]
    pub const fn mcf(&self) -> &Mcf {
        &self.mcf
    }
    #[doc = "0x140 - Excessive Collisions Register"]
    #[inline(always)]
    pub const fn ec(&self) -> &Ec {
        &self.ec
    }
    #[doc = "0x144 - Late Collisions Register"]
    #[inline(always)]
    pub const fn lc(&self) -> &Lc {
        &self.lc
    }
    #[doc = "0x148 - Deferred Transmission Frames Register"]
    #[inline(always)]
    pub const fn dtf(&self) -> &Dtf {
        &self.dtf
    }
    #[doc = "0x14c - Carrier Sense Errors Register"]
    #[inline(always)]
    pub const fn cse(&self) -> &Cse {
        &self.cse
    }
    #[doc = "0x150 - Octets Received Low Received Register"]
    #[inline(always)]
    pub const fn orlo(&self) -> &Orlo {
        &self.orlo
    }
    #[doc = "0x154 - Octets Received High Received Register"]
    #[inline(always)]
    pub const fn orhi(&self) -> &Orhi {
        &self.orhi
    }
    #[doc = "0x158 - Frames Received Register"]
    #[inline(always)]
    pub const fn fr(&self) -> &Fr {
        &self.fr
    }
    #[doc = "0x15c - Broadcast Frames Received Register"]
    #[inline(always)]
    pub const fn bcfr(&self) -> &Bcfr {
        &self.bcfr
    }
    #[doc = "0x160 - Multicast Frames Received Register"]
    #[inline(always)]
    pub const fn mfr(&self) -> &Mfr {
        &self.mfr
    }
    #[doc = "0x164 - Pause Frames Received Register"]
    #[inline(always)]
    pub const fn pfr(&self) -> &Pfr {
        &self.pfr
    }
    #[doc = "0x168 - 64 Byte Frames Received Register"]
    #[inline(always)]
    pub const fn bfr64(&self) -> &Bfr64 {
        &self.bfr64
    }
    #[doc = "0x16c - 65 to 127 Byte Frames Received Register"]
    #[inline(always)]
    pub const fn tbfr127(&self) -> &Tbfr127 {
        &self.tbfr127
    }
    #[doc = "0x170 - 128 to 255 Byte Frames Received Register"]
    #[inline(always)]
    pub const fn tbfr255(&self) -> &Tbfr255 {
        &self.tbfr255
    }
    #[doc = "0x174 - 256 to 511 Byte Frames Received Register"]
    #[inline(always)]
    pub const fn tbfr511(&self) -> &Tbfr511 {
        &self.tbfr511
    }
    #[doc = "0x178 - 512 to 1023 Byte Frames Received Register"]
    #[inline(always)]
    pub const fn tbfr1023(&self) -> &Tbfr1023 {
        &self.tbfr1023
    }
    #[doc = "0x17c - 1024 to 1518 Byte Frames Received Register"]
    #[inline(always)]
    pub const fn tbfr1518(&self) -> &Tbfr1518 {
        &self.tbfr1518
    }
    #[doc = "0x180 - 1519 to Maximum Byte Frames Received Register"]
    #[inline(always)]
    pub const fn tmxbfr(&self) -> &Tmxbfr {
        &self.tmxbfr
    }
    #[doc = "0x184 - Undersize Frames Received Register"]
    #[inline(always)]
    pub const fn ufr(&self) -> &Ufr {
        &self.ufr
    }
    #[doc = "0x188 - Oversize Frames Received Register"]
    #[inline(always)]
    pub const fn ofr(&self) -> &Ofr {
        &self.ofr
    }
    #[doc = "0x18c - Jabbers Received Register"]
    #[inline(always)]
    pub const fn jr(&self) -> &Jr {
        &self.jr
    }
    #[doc = "0x190 - Frame Check Sequence Errors Register"]
    #[inline(always)]
    pub const fn fcse(&self) -> &Fcse {
        &self.fcse
    }
    #[doc = "0x194 - Length Field Frame Errors Register"]
    #[inline(always)]
    pub const fn lffe(&self) -> &Lffe {
        &self.lffe
    }
    #[doc = "0x198 - Receive Symbol Errors Register"]
    #[inline(always)]
    pub const fn rse(&self) -> &Rse {
        &self.rse
    }
    #[doc = "0x19c - Alignment Errors Register"]
    #[inline(always)]
    pub const fn ae(&self) -> &Ae {
        &self.ae
    }
    #[doc = "0x1a0 - Receive Resource Errors Register"]
    #[inline(always)]
    pub const fn rre(&self) -> &Rre {
        &self.rre
    }
    #[doc = "0x1a4 - Receive Overrun Register"]
    #[inline(always)]
    pub const fn roe(&self) -> &Roe {
        &self.roe
    }
    #[doc = "0x1a8 - IP Header Checksum Errors Register"]
    #[inline(always)]
    pub const fn ihce(&self) -> &Ihce {
        &self.ihce
    }
    #[doc = "0x1ac - TCP Checksum Errors Register"]
    #[inline(always)]
    pub const fn tce(&self) -> &Tce {
        &self.tce
    }
    #[doc = "0x1b0 - UDP Checksum Errors Register"]
    #[inline(always)]
    pub const fn uce(&self) -> &Uce {
        &self.uce
    }
    #[doc = "0x1bc - 1588 Timer Increment Sub-nanoseconds Register"]
    #[inline(always)]
    pub const fn tisubn(&self) -> &Tisubn {
        &self.tisubn
    }
    #[doc = "0x1c0 - 1588 Timer Seconds High Register"]
    #[inline(always)]
    pub const fn tsh(&self) -> &Tsh {
        &self.tsh
    }
    #[doc = "0x1d0 - 1588 Timer Seconds Low Register"]
    #[inline(always)]
    pub const fn tsl(&self) -> &Tsl {
        &self.tsl
    }
    #[doc = "0x1d4 - 1588 Timer Nanoseconds Register"]
    #[inline(always)]
    pub const fn tn(&self) -> &Tn {
        &self.tn
    }
    #[doc = "0x1d8 - 1588 Timer Adjust Register"]
    #[inline(always)]
    pub const fn ta(&self) -> &Ta {
        &self.ta
    }
    #[doc = "0x1dc - 1588 Timer Increment Register"]
    #[inline(always)]
    pub const fn ti(&self) -> &Ti {
        &self.ti
    }
    #[doc = "0x1e0 - PTP Event Frame Transmitted Seconds Low Register"]
    #[inline(always)]
    pub const fn eftsl(&self) -> &Eftsl {
        &self.eftsl
    }
    #[doc = "0x1e4 - PTP Event Frame Transmitted Nanoseconds Register"]
    #[inline(always)]
    pub const fn eftn(&self) -> &Eftn {
        &self.eftn
    }
    #[doc = "0x1e8 - PTP Event Frame Received Seconds Low Register"]
    #[inline(always)]
    pub const fn efrsl(&self) -> &Efrsl {
        &self.efrsl
    }
    #[doc = "0x1ec - PTP Event Frame Received Nanoseconds Register"]
    #[inline(always)]
    pub const fn efrn(&self) -> &Efrn {
        &self.efrn
    }
    #[doc = "0x1f0 - PTP Peer Event Frame Transmitted Seconds Low Register"]
    #[inline(always)]
    pub const fn peftsl(&self) -> &Peftsl {
        &self.peftsl
    }
    #[doc = "0x1f4 - PTP Peer Event Frame Transmitted Nanoseconds Register"]
    #[inline(always)]
    pub const fn peftn(&self) -> &Peftn {
        &self.peftn
    }
    #[doc = "0x1f8 - PTP Peer Event Frame Received Seconds Low Register"]
    #[inline(always)]
    pub const fn pefrsl(&self) -> &Pefrsl {
        &self.pefrsl
    }
    #[doc = "0x1fc - PTP Peer Event Frame Received Nanoseconds Register"]
    #[inline(always)]
    pub const fn pefrn(&self) -> &Pefrn {
        &self.pefrn
    }
    #[doc = "0x270 - Received LPI Transitions"]
    #[inline(always)]
    pub const fn rxlpi(&self) -> &Rxlpi {
        &self.rxlpi
    }
    #[doc = "0x274 - Received LPI Time"]
    #[inline(always)]
    pub const fn rxlpitime(&self) -> &Rxlpitime {
        &self.rxlpitime
    }
    #[doc = "0x278 - Transmit LPI Transitions"]
    #[inline(always)]
    pub const fn txlpi(&self) -> &Txlpi {
        &self.txlpi
    }
    #[doc = "0x27c - Transmit LPI Time"]
    #[inline(always)]
    pub const fn txlpitime(&self) -> &Txlpitime {
        &self.txlpitime
    }
    #[doc = "0x400..0x414 - Interrupt Status Register Priority Queue (1..5)"]
    #[inline(always)]
    pub const fn isrpq(&self, n: usize) -> &Isrpq {
        &self.isrpq[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x400..0x414 - Interrupt Status Register Priority Queue (1..5)"]
    #[inline(always)]
    pub fn isrpq_iter(&self) -> impl Iterator<Item = &Isrpq> {
        self.isrpq.iter()
    }
    #[doc = "0x440..0x454 - Transmit Buffer Queue Base Address Register Priority Queue (1..5)"]
    #[inline(always)]
    pub const fn tbqbapq(&self, n: usize) -> &Tbqbapq {
        &self.tbqbapq[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x440..0x454 - Transmit Buffer Queue Base Address Register Priority Queue (1..5)"]
    #[inline(always)]
    pub fn tbqbapq_iter(&self) -> impl Iterator<Item = &Tbqbapq> {
        self.tbqbapq.iter()
    }
    #[doc = "0x480..0x494 - Receive Buffer Queue Base Address Register Priority Queue (1..5)"]
    #[inline(always)]
    pub const fn rbqbapq(&self, n: usize) -> &Rbqbapq {
        &self.rbqbapq[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x480..0x494 - Receive Buffer Queue Base Address Register Priority Queue (1..5)"]
    #[inline(always)]
    pub fn rbqbapq_iter(&self) -> impl Iterator<Item = &Rbqbapq> {
        self.rbqbapq.iter()
    }
    #[doc = "0x4a0..0x4b4 - Receive Buffer Size Register Priority Queue (1..5)"]
    #[inline(always)]
    pub const fn rbsrpq(&self, n: usize) -> &Rbsrpq {
        &self.rbsrpq[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x4a0..0x4b4 - Receive Buffer Size Register Priority Queue (1..5)"]
    #[inline(always)]
    pub fn rbsrpq_iter(&self) -> impl Iterator<Item = &Rbsrpq> {
        self.rbsrpq.iter()
    }
    #[doc = "0x4bc - Credit-Based Shaping Control Register"]
    #[inline(always)]
    pub const fn cbscr(&self) -> &Cbscr {
        &self.cbscr
    }
    #[doc = "0x4c0 - Credit-Based Shaping IdleSlope Register for Queue A"]
    #[inline(always)]
    pub const fn cbsisqa(&self) -> &Cbsisqa {
        &self.cbsisqa
    }
    #[doc = "0x4c4 - Credit-Based Shaping IdleSlope Register for Queue B"]
    #[inline(always)]
    pub const fn cbsisqb(&self) -> &Cbsisqb {
        &self.cbsisqb
    }
    #[doc = "0x500..0x510 - Screening Type 1 Register Priority Queue"]
    #[inline(always)]
    pub const fn st1rpq(&self, n: usize) -> &St1rpq {
        &self.st1rpq[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x500..0x510 - Screening Type 1 Register Priority Queue"]
    #[inline(always)]
    pub fn st1rpq_iter(&self) -> impl Iterator<Item = &St1rpq> {
        self.st1rpq.iter()
    }
    #[doc = "0x540..0x560 - Screening Type 2 Register Priority Queue"]
    #[inline(always)]
    pub const fn st2rpq(&self, n: usize) -> &St2rpq {
        &self.st2rpq[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x540..0x560 - Screening Type 2 Register Priority Queue"]
    #[inline(always)]
    pub fn st2rpq_iter(&self) -> impl Iterator<Item = &St2rpq> {
        self.st2rpq.iter()
    }
    #[doc = "0x600..0x614 - Interrupt Enable Register Priority Queue (1..5)"]
    #[inline(always)]
    pub const fn ierpq(&self, n: usize) -> &Ierpq {
        &self.ierpq[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x600..0x614 - Interrupt Enable Register Priority Queue (1..5)"]
    #[inline(always)]
    pub fn ierpq_iter(&self) -> impl Iterator<Item = &Ierpq> {
        self.ierpq.iter()
    }
    #[doc = "0x620..0x634 - Interrupt Disable Register Priority Queue (1..5)"]
    #[inline(always)]
    pub const fn idrpq(&self, n: usize) -> &Idrpq {
        &self.idrpq[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x620..0x634 - Interrupt Disable Register Priority Queue (1..5)"]
    #[inline(always)]
    pub fn idrpq_iter(&self) -> impl Iterator<Item = &Idrpq> {
        self.idrpq.iter()
    }
    #[doc = "0x640..0x654 - Interrupt Mask Register Priority Queue (1..5)"]
    #[inline(always)]
    pub const fn imrpq(&self, n: usize) -> &Imrpq {
        &self.imrpq[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x640..0x654 - Interrupt Mask Register Priority Queue (1..5)"]
    #[inline(always)]
    pub fn imrpq_iter(&self) -> impl Iterator<Item = &Imrpq> {
        self.imrpq.iter()
    }
    #[doc = "0x6e0..0x6f0 - Screening Type 2 Ethertype Register"]
    #[inline(always)]
    pub const fn st2er(&self, n: usize) -> &St2er {
        &self.st2er[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x6e0..0x6f0 - Screening Type 2 Ethertype Register"]
    #[inline(always)]
    pub fn st2er_iter(&self) -> impl Iterator<Item = &St2er> {
        self.st2er.iter()
    }
    #[doc = "0x700..0x7c0 - Screening Type 2 Compare Word 0 Register"]
    #[inline(always)]
    pub const fn gmac_st2cw(&self, n: usize) -> &GmacSt2cw {
        &self.gmac_st2cw[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x700..0x7c0 - Screening Type 2 Compare Word 0 Register"]
    #[inline(always)]
    pub fn gmac_st2cw_iter(&self) -> impl Iterator<Item = &GmacSt2cw> {
        self.gmac_st2cw.iter()
    }
}
#[doc = "NCR (rw) register accessor: Network Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ncr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ncr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ncr`] module"]
#[doc(alias = "NCR")]
pub type Ncr = crate::Reg<ncr::NcrSpec>;
#[doc = "Network Control Register"]
pub mod ncr;
#[doc = "NCFGR (rw) register accessor: Network Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ncfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ncfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ncfgr`] module"]
#[doc(alias = "NCFGR")]
pub type Ncfgr = crate::Reg<ncfgr::NcfgrSpec>;
#[doc = "Network Configuration Register"]
pub mod ncfgr;
#[doc = "NSR (r) register accessor: Network Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`nsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nsr`] module"]
#[doc(alias = "NSR")]
pub type Nsr = crate::Reg<nsr::NsrSpec>;
#[doc = "Network Status Register"]
pub mod nsr;
#[doc = "UR (rw) register accessor: User Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ur`] module"]
#[doc(alias = "UR")]
pub type Ur = crate::Reg<ur::UrSpec>;
#[doc = "User Register"]
pub mod ur;
#[doc = "DCFGR (rw) register accessor: DMA Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcfgr`] module"]
#[doc(alias = "DCFGR")]
pub type Dcfgr = crate::Reg<dcfgr::DcfgrSpec>;
#[doc = "DMA Configuration Register"]
pub mod dcfgr;
#[doc = "TSR (rw) register accessor: Transmit Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsr`] module"]
#[doc(alias = "TSR")]
pub type Tsr = crate::Reg<tsr::TsrSpec>;
#[doc = "Transmit Status Register"]
pub mod tsr;
#[doc = "RBQB (rw) register accessor: Receive Buffer Queue Base Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rbqb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rbqb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rbqb`] module"]
#[doc(alias = "RBQB")]
pub type Rbqb = crate::Reg<rbqb::RbqbSpec>;
#[doc = "Receive Buffer Queue Base Address Register"]
pub mod rbqb;
#[doc = "TBQB (rw) register accessor: Transmit Buffer Queue Base Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tbqb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbqb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbqb`] module"]
#[doc(alias = "TBQB")]
pub type Tbqb = crate::Reg<tbqb::TbqbSpec>;
#[doc = "Transmit Buffer Queue Base Address Register"]
pub mod tbqb;
#[doc = "RSR (rw) register accessor: Receive Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsr`] module"]
#[doc(alias = "RSR")]
pub type Rsr = crate::Reg<rsr::RsrSpec>;
#[doc = "Receive Status Register"]
pub mod rsr;
#[doc = "ISR (r) register accessor: Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`] module"]
#[doc(alias = "ISR")]
pub type Isr = crate::Reg<isr::IsrSpec>;
#[doc = "Interrupt Status Register"]
pub mod isr;
#[doc = "IER (w) register accessor: Interrupt Enable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`] module"]
#[doc(alias = "IER")]
pub type Ier = crate::Reg<ier::IerSpec>;
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "IDR (w) register accessor: Interrupt Disable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idr`] module"]
#[doc(alias = "IDR")]
pub type Idr = crate::Reg<idr::IdrSpec>;
#[doc = "Interrupt Disable Register"]
pub mod idr;
#[doc = "IMR (rw) register accessor: Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`imr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr`] module"]
#[doc(alias = "IMR")]
pub type Imr = crate::Reg<imr::ImrSpec>;
#[doc = "Interrupt Mask Register"]
pub mod imr;
#[doc = "MAN (rw) register accessor: PHY Maintenance Register\n\nYou can [`read`](crate::Reg::read) this register and get [`man::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`man::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@man`] module"]
#[doc(alias = "MAN")]
pub type Man = crate::Reg<man::ManSpec>;
#[doc = "PHY Maintenance Register"]
pub mod man;
#[doc = "RPQ (r) register accessor: Received Pause Quantum Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rpq::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rpq`] module"]
#[doc(alias = "RPQ")]
pub type Rpq = crate::Reg<rpq::RpqSpec>;
#[doc = "Received Pause Quantum Register"]
pub mod rpq;
#[doc = "TPQ (rw) register accessor: Transmit Pause Quantum Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tpq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tpq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tpq`] module"]
#[doc(alias = "TPQ")]
pub type Tpq = crate::Reg<tpq::TpqSpec>;
#[doc = "Transmit Pause Quantum Register"]
pub mod tpq;
#[doc = "TPSF (rw) register accessor: TX Partial Store and Forward Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tpsf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tpsf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tpsf`] module"]
#[doc(alias = "TPSF")]
pub type Tpsf = crate::Reg<tpsf::TpsfSpec>;
#[doc = "TX Partial Store and Forward Register"]
pub mod tpsf;
#[doc = "RPSF (rw) register accessor: RX Partial Store and Forward Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rpsf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rpsf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rpsf`] module"]
#[doc(alias = "RPSF")]
pub type Rpsf = crate::Reg<rpsf::RpsfSpec>;
#[doc = "RX Partial Store and Forward Register"]
pub mod rpsf;
#[doc = "RJFML (rw) register accessor: RX Jumbo Frame Max Length Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rjfml::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rjfml::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rjfml`] module"]
#[doc(alias = "RJFML")]
pub type Rjfml = crate::Reg<rjfml::RjfmlSpec>;
#[doc = "RX Jumbo Frame Max Length Register"]
pub mod rjfml;
#[doc = "HRB (rw) register accessor: Hash Register Bottom\n\nYou can [`read`](crate::Reg::read) this register and get [`hrb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hrb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hrb`] module"]
#[doc(alias = "HRB")]
pub type Hrb = crate::Reg<hrb::HrbSpec>;
#[doc = "Hash Register Bottom"]
pub mod hrb;
#[doc = "HRT (rw) register accessor: Hash Register Top\n\nYou can [`read`](crate::Reg::read) this register and get [`hrt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hrt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hrt`] module"]
#[doc(alias = "HRT")]
pub type Hrt = crate::Reg<hrt::HrtSpec>;
#[doc = "Hash Register Top"]
pub mod hrt;
#[doc = "Specific Address 1 Bottom Register"]
pub use self::gmac_sa::GmacSa;
#[doc = r"Cluster"]
#[doc = "Specific Address 1 Bottom Register"]
pub mod gmac_sa;
#[doc = "TIDM1 (rw) register accessor: Type ID Match 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tidm1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tidm1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tidm1`] module"]
#[doc(alias = "TIDM1")]
pub type Tidm1 = crate::Reg<tidm1::Tidm1Spec>;
#[doc = "Type ID Match 1 Register"]
pub mod tidm1;
#[doc = "TIDM2 (rw) register accessor: Type ID Match 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tidm2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tidm2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tidm2`] module"]
#[doc(alias = "TIDM2")]
pub type Tidm2 = crate::Reg<tidm2::Tidm2Spec>;
#[doc = "Type ID Match 2 Register"]
pub mod tidm2;
#[doc = "TIDM3 (rw) register accessor: Type ID Match 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tidm3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tidm3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tidm3`] module"]
#[doc(alias = "TIDM3")]
pub type Tidm3 = crate::Reg<tidm3::Tidm3Spec>;
#[doc = "Type ID Match 3 Register"]
pub mod tidm3;
#[doc = "TIDM4 (rw) register accessor: Type ID Match 4 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tidm4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tidm4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tidm4`] module"]
#[doc(alias = "TIDM4")]
pub type Tidm4 = crate::Reg<tidm4::Tidm4Spec>;
#[doc = "Type ID Match 4 Register"]
pub mod tidm4;
#[doc = "IPGS (rw) register accessor: IPG Stretch Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ipgs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipgs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipgs`] module"]
#[doc(alias = "IPGS")]
pub type Ipgs = crate::Reg<ipgs::IpgsSpec>;
#[doc = "IPG Stretch Register"]
pub mod ipgs;
#[doc = "SVLAN (rw) register accessor: Stacked VLAN Register\n\nYou can [`read`](crate::Reg::read) this register and get [`svlan::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`svlan::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@svlan`] module"]
#[doc(alias = "SVLAN")]
pub type Svlan = crate::Reg<svlan::SvlanSpec>;
#[doc = "Stacked VLAN Register"]
pub mod svlan;
#[doc = "TPFCP (rw) register accessor: Transmit PFC Pause Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tpfcp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tpfcp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tpfcp`] module"]
#[doc(alias = "TPFCP")]
pub type Tpfcp = crate::Reg<tpfcp::TpfcpSpec>;
#[doc = "Transmit PFC Pause Register"]
pub mod tpfcp;
#[doc = "SAMB1 (rw) register accessor: Specific Address 1 Mask Bottom Register\n\nYou can [`read`](crate::Reg::read) this register and get [`samb1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`samb1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@samb1`] module"]
#[doc(alias = "SAMB1")]
pub type Samb1 = crate::Reg<samb1::Samb1Spec>;
#[doc = "Specific Address 1 Mask Bottom Register"]
pub mod samb1;
#[doc = "SAMT1 (rw) register accessor: Specific Address 1 Mask Top Register\n\nYou can [`read`](crate::Reg::read) this register and get [`samt1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`samt1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@samt1`] module"]
#[doc(alias = "SAMT1")]
pub type Samt1 = crate::Reg<samt1::Samt1Spec>;
#[doc = "Specific Address 1 Mask Top Register"]
pub mod samt1;
#[doc = "NSC (rw) register accessor: 1588 Timer Nanosecond Comparison Register\n\nYou can [`read`](crate::Reg::read) this register and get [`nsc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nsc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nsc`] module"]
#[doc(alias = "NSC")]
pub type Nsc = crate::Reg<nsc::NscSpec>;
#[doc = "1588 Timer Nanosecond Comparison Register"]
pub mod nsc;
#[doc = "SCL (rw) register accessor: 1588 Timer Second Comparison Low Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scl`] module"]
#[doc(alias = "SCL")]
pub type Scl = crate::Reg<scl::SclSpec>;
#[doc = "1588 Timer Second Comparison Low Register"]
pub mod scl;
#[doc = "SCH (rw) register accessor: 1588 Timer Second Comparison High Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sch`] module"]
#[doc(alias = "SCH")]
pub type Sch = crate::Reg<sch::SchSpec>;
#[doc = "1588 Timer Second Comparison High Register"]
pub mod sch;
#[doc = "EFTSH (r) register accessor: PTP Event Frame Transmitted Seconds High Register\n\nYou can [`read`](crate::Reg::read) this register and get [`eftsh::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eftsh`] module"]
#[doc(alias = "EFTSH")]
pub type Eftsh = crate::Reg<eftsh::EftshSpec>;
#[doc = "PTP Event Frame Transmitted Seconds High Register"]
pub mod eftsh;
#[doc = "EFRSH (r) register accessor: PTP Event Frame Received Seconds High Register\n\nYou can [`read`](crate::Reg::read) this register and get [`efrsh::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efrsh`] module"]
#[doc(alias = "EFRSH")]
pub type Efrsh = crate::Reg<efrsh::EfrshSpec>;
#[doc = "PTP Event Frame Received Seconds High Register"]
pub mod efrsh;
#[doc = "PEFTSH (r) register accessor: PTP Peer Event Frame Transmitted Seconds High Register\n\nYou can [`read`](crate::Reg::read) this register and get [`peftsh::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peftsh`] module"]
#[doc(alias = "PEFTSH")]
pub type Peftsh = crate::Reg<peftsh::PeftshSpec>;
#[doc = "PTP Peer Event Frame Transmitted Seconds High Register"]
pub mod peftsh;
#[doc = "PEFRSH (r) register accessor: PTP Peer Event Frame Received Seconds High Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pefrsh::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pefrsh`] module"]
#[doc(alias = "PEFRSH")]
pub type Pefrsh = crate::Reg<pefrsh::PefrshSpec>;
#[doc = "PTP Peer Event Frame Received Seconds High Register"]
pub mod pefrsh;
#[doc = "MID (r) register accessor: Module ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mid`] module"]
#[doc(alias = "MID")]
pub type Mid = crate::Reg<mid::MidSpec>;
#[doc = "Module ID Register"]
pub mod mid;
#[doc = "OTLO (r) register accessor: Octets Transmitted Low Register\n\nYou can [`read`](crate::Reg::read) this register and get [`otlo::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otlo`] module"]
#[doc(alias = "OTLO")]
pub type Otlo = crate::Reg<otlo::OtloSpec>;
#[doc = "Octets Transmitted Low Register"]
pub mod otlo;
#[doc = "OTHI (r) register accessor: Octets Transmitted High Register\n\nYou can [`read`](crate::Reg::read) this register and get [`othi::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@othi`] module"]
#[doc(alias = "OTHI")]
pub type Othi = crate::Reg<othi::OthiSpec>;
#[doc = "Octets Transmitted High Register"]
pub mod othi;
#[doc = "FT (r) register accessor: Frames Transmitted Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ft::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ft`] module"]
#[doc(alias = "FT")]
pub type Ft = crate::Reg<ft::FtSpec>;
#[doc = "Frames Transmitted Register"]
pub mod ft;
#[doc = "BCFT (r) register accessor: Broadcast Frames Transmitted Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bcft::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcft`] module"]
#[doc(alias = "BCFT")]
pub type Bcft = crate::Reg<bcft::BcftSpec>;
#[doc = "Broadcast Frames Transmitted Register"]
pub mod bcft;
#[doc = "MFT (r) register accessor: Multicast Frames Transmitted Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mft::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mft`] module"]
#[doc(alias = "MFT")]
pub type Mft = crate::Reg<mft::MftSpec>;
#[doc = "Multicast Frames Transmitted Register"]
pub mod mft;
#[doc = "PFT (r) register accessor: Pause Frames Transmitted Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pft::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pft`] module"]
#[doc(alias = "PFT")]
pub type Pft = crate::Reg<pft::PftSpec>;
#[doc = "Pause Frames Transmitted Register"]
pub mod pft;
#[doc = "BFT64 (r) register accessor: 64 Byte Frames Transmitted Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bft64::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bft64`] module"]
#[doc(alias = "BFT64")]
pub type Bft64 = crate::Reg<bft64::Bft64Spec>;
#[doc = "64 Byte Frames Transmitted Register"]
pub mod bft64;
#[doc = "TBFT127 (r) register accessor: 65 to 127 Byte Frames Transmitted Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tbft127::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbft127`] module"]
#[doc(alias = "TBFT127")]
pub type Tbft127 = crate::Reg<tbft127::Tbft127Spec>;
#[doc = "65 to 127 Byte Frames Transmitted Register"]
pub mod tbft127;
#[doc = "TBFT255 (r) register accessor: 128 to 255 Byte Frames Transmitted Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tbft255::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbft255`] module"]
#[doc(alias = "TBFT255")]
pub type Tbft255 = crate::Reg<tbft255::Tbft255Spec>;
#[doc = "128 to 255 Byte Frames Transmitted Register"]
pub mod tbft255;
#[doc = "TBFT511 (r) register accessor: 256 to 511 Byte Frames Transmitted Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tbft511::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbft511`] module"]
#[doc(alias = "TBFT511")]
pub type Tbft511 = crate::Reg<tbft511::Tbft511Spec>;
#[doc = "256 to 511 Byte Frames Transmitted Register"]
pub mod tbft511;
#[doc = "TBFT1023 (r) register accessor: 512 to 1023 Byte Frames Transmitted Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tbft1023::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbft1023`] module"]
#[doc(alias = "TBFT1023")]
pub type Tbft1023 = crate::Reg<tbft1023::Tbft1023Spec>;
#[doc = "512 to 1023 Byte Frames Transmitted Register"]
pub mod tbft1023;
#[doc = "TBFT1518 (r) register accessor: 1024 to 1518 Byte Frames Transmitted Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tbft1518::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbft1518`] module"]
#[doc(alias = "TBFT1518")]
pub type Tbft1518 = crate::Reg<tbft1518::Tbft1518Spec>;
#[doc = "1024 to 1518 Byte Frames Transmitted Register"]
pub mod tbft1518;
#[doc = "GTBFT1518 (r) register accessor: Greater Than 1518 Byte Frames Transmitted Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gtbft1518::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtbft1518`] module"]
#[doc(alias = "GTBFT1518")]
pub type Gtbft1518 = crate::Reg<gtbft1518::Gtbft1518Spec>;
#[doc = "Greater Than 1518 Byte Frames Transmitted Register"]
pub mod gtbft1518;
#[doc = "TUR (r) register accessor: Transmit Underruns Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tur::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tur`] module"]
#[doc(alias = "TUR")]
pub type Tur = crate::Reg<tur::TurSpec>;
#[doc = "Transmit Underruns Register"]
pub mod tur;
#[doc = "SCF (r) register accessor: Single Collision Frames Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scf::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scf`] module"]
#[doc(alias = "SCF")]
pub type Scf = crate::Reg<scf::ScfSpec>;
#[doc = "Single Collision Frames Register"]
pub mod scf;
#[doc = "MCF (r) register accessor: Multiple Collision Frames Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcf::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcf`] module"]
#[doc(alias = "MCF")]
pub type Mcf = crate::Reg<mcf::McfSpec>;
#[doc = "Multiple Collision Frames Register"]
pub mod mcf;
#[doc = "EC (r) register accessor: Excessive Collisions Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ec::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ec`] module"]
#[doc(alias = "EC")]
pub type Ec = crate::Reg<ec::EcSpec>;
#[doc = "Excessive Collisions Register"]
pub mod ec;
#[doc = "LC (r) register accessor: Late Collisions Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lc`] module"]
#[doc(alias = "LC")]
pub type Lc = crate::Reg<lc::LcSpec>;
#[doc = "Late Collisions Register"]
pub mod lc;
#[doc = "DTF (r) register accessor: Deferred Transmission Frames Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dtf::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtf`] module"]
#[doc(alias = "DTF")]
pub type Dtf = crate::Reg<dtf::DtfSpec>;
#[doc = "Deferred Transmission Frames Register"]
pub mod dtf;
#[doc = "CSE (r) register accessor: Carrier Sense Errors Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cse::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cse`] module"]
#[doc(alias = "CSE")]
pub type Cse = crate::Reg<cse::CseSpec>;
#[doc = "Carrier Sense Errors Register"]
pub mod cse;
#[doc = "ORLO (r) register accessor: Octets Received Low Received Register\n\nYou can [`read`](crate::Reg::read) this register and get [`orlo::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@orlo`] module"]
#[doc(alias = "ORLO")]
pub type Orlo = crate::Reg<orlo::OrloSpec>;
#[doc = "Octets Received Low Received Register"]
pub mod orlo;
#[doc = "ORHI (r) register accessor: Octets Received High Received Register\n\nYou can [`read`](crate::Reg::read) this register and get [`orhi::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@orhi`] module"]
#[doc(alias = "ORHI")]
pub type Orhi = crate::Reg<orhi::OrhiSpec>;
#[doc = "Octets Received High Received Register"]
pub mod orhi;
#[doc = "FR (r) register accessor: Frames Received Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fr`] module"]
#[doc(alias = "FR")]
pub type Fr = crate::Reg<fr::FrSpec>;
#[doc = "Frames Received Register"]
pub mod fr;
#[doc = "BCFR (r) register accessor: Broadcast Frames Received Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bcfr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcfr`] module"]
#[doc(alias = "BCFR")]
pub type Bcfr = crate::Reg<bcfr::BcfrSpec>;
#[doc = "Broadcast Frames Received Register"]
pub mod bcfr;
#[doc = "MFR (r) register accessor: Multicast Frames Received Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mfr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mfr`] module"]
#[doc(alias = "MFR")]
pub type Mfr = crate::Reg<mfr::MfrSpec>;
#[doc = "Multicast Frames Received Register"]
pub mod mfr;
#[doc = "PFR (r) register accessor: Pause Frames Received Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pfr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pfr`] module"]
#[doc(alias = "PFR")]
pub type Pfr = crate::Reg<pfr::PfrSpec>;
#[doc = "Pause Frames Received Register"]
pub mod pfr;
#[doc = "BFR64 (r) register accessor: 64 Byte Frames Received Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bfr64::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bfr64`] module"]
#[doc(alias = "BFR64")]
pub type Bfr64 = crate::Reg<bfr64::Bfr64Spec>;
#[doc = "64 Byte Frames Received Register"]
pub mod bfr64;
#[doc = "TBFR127 (r) register accessor: 65 to 127 Byte Frames Received Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tbfr127::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbfr127`] module"]
#[doc(alias = "TBFR127")]
pub type Tbfr127 = crate::Reg<tbfr127::Tbfr127Spec>;
#[doc = "65 to 127 Byte Frames Received Register"]
pub mod tbfr127;
#[doc = "TBFR255 (r) register accessor: 128 to 255 Byte Frames Received Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tbfr255::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbfr255`] module"]
#[doc(alias = "TBFR255")]
pub type Tbfr255 = crate::Reg<tbfr255::Tbfr255Spec>;
#[doc = "128 to 255 Byte Frames Received Register"]
pub mod tbfr255;
#[doc = "TBFR511 (r) register accessor: 256 to 511 Byte Frames Received Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tbfr511::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbfr511`] module"]
#[doc(alias = "TBFR511")]
pub type Tbfr511 = crate::Reg<tbfr511::Tbfr511Spec>;
#[doc = "256 to 511 Byte Frames Received Register"]
pub mod tbfr511;
#[doc = "TBFR1023 (r) register accessor: 512 to 1023 Byte Frames Received Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tbfr1023::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbfr1023`] module"]
#[doc(alias = "TBFR1023")]
pub type Tbfr1023 = crate::Reg<tbfr1023::Tbfr1023Spec>;
#[doc = "512 to 1023 Byte Frames Received Register"]
pub mod tbfr1023;
#[doc = "TBFR1518 (r) register accessor: 1024 to 1518 Byte Frames Received Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tbfr1518::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbfr1518`] module"]
#[doc(alias = "TBFR1518")]
pub type Tbfr1518 = crate::Reg<tbfr1518::Tbfr1518Spec>;
#[doc = "1024 to 1518 Byte Frames Received Register"]
pub mod tbfr1518;
#[doc = "TMXBFR (r) register accessor: 1519 to Maximum Byte Frames Received Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tmxbfr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmxbfr`] module"]
#[doc(alias = "TMXBFR")]
pub type Tmxbfr = crate::Reg<tmxbfr::TmxbfrSpec>;
#[doc = "1519 to Maximum Byte Frames Received Register"]
pub mod tmxbfr;
#[doc = "UFR (r) register accessor: Undersize Frames Received Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ufr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ufr`] module"]
#[doc(alias = "UFR")]
pub type Ufr = crate::Reg<ufr::UfrSpec>;
#[doc = "Undersize Frames Received Register"]
pub mod ufr;
#[doc = "OFR (r) register accessor: Oversize Frames Received Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ofr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ofr`] module"]
#[doc(alias = "OFR")]
pub type Ofr = crate::Reg<ofr::OfrSpec>;
#[doc = "Oversize Frames Received Register"]
pub mod ofr;
#[doc = "JR (r) register accessor: Jabbers Received Register\n\nYou can [`read`](crate::Reg::read) this register and get [`jr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jr`] module"]
#[doc(alias = "JR")]
pub type Jr = crate::Reg<jr::JrSpec>;
#[doc = "Jabbers Received Register"]
pub mod jr;
#[doc = "FCSE (r) register accessor: Frame Check Sequence Errors Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fcse::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcse`] module"]
#[doc(alias = "FCSE")]
pub type Fcse = crate::Reg<fcse::FcseSpec>;
#[doc = "Frame Check Sequence Errors Register"]
pub mod fcse;
#[doc = "LFFE (r) register accessor: Length Field Frame Errors Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lffe::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lffe`] module"]
#[doc(alias = "LFFE")]
pub type Lffe = crate::Reg<lffe::LffeSpec>;
#[doc = "Length Field Frame Errors Register"]
pub mod lffe;
#[doc = "RSE (r) register accessor: Receive Symbol Errors Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rse::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rse`] module"]
#[doc(alias = "RSE")]
pub type Rse = crate::Reg<rse::RseSpec>;
#[doc = "Receive Symbol Errors Register"]
pub mod rse;
#[doc = "AE (r) register accessor: Alignment Errors Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ae::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ae`] module"]
#[doc(alias = "AE")]
pub type Ae = crate::Reg<ae::AeSpec>;
#[doc = "Alignment Errors Register"]
pub mod ae;
#[doc = "RRE (r) register accessor: Receive Resource Errors Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rre::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rre`] module"]
#[doc(alias = "RRE")]
pub type Rre = crate::Reg<rre::RreSpec>;
#[doc = "Receive Resource Errors Register"]
pub mod rre;
#[doc = "ROE (r) register accessor: Receive Overrun Register\n\nYou can [`read`](crate::Reg::read) this register and get [`roe::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@roe`] module"]
#[doc(alias = "ROE")]
pub type Roe = crate::Reg<roe::RoeSpec>;
#[doc = "Receive Overrun Register"]
pub mod roe;
#[doc = "IHCE (r) register accessor: IP Header Checksum Errors Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ihce::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ihce`] module"]
#[doc(alias = "IHCE")]
pub type Ihce = crate::Reg<ihce::IhceSpec>;
#[doc = "IP Header Checksum Errors Register"]
pub mod ihce;
#[doc = "TCE (r) register accessor: TCP Checksum Errors Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tce::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tce`] module"]
#[doc(alias = "TCE")]
pub type Tce = crate::Reg<tce::TceSpec>;
#[doc = "TCP Checksum Errors Register"]
pub mod tce;
#[doc = "UCE (r) register accessor: UDP Checksum Errors Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uce::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uce`] module"]
#[doc(alias = "UCE")]
pub type Uce = crate::Reg<uce::UceSpec>;
#[doc = "UDP Checksum Errors Register"]
pub mod uce;
#[doc = "TISUBN (rw) register accessor: 1588 Timer Increment Sub-nanoseconds Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tisubn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tisubn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tisubn`] module"]
#[doc(alias = "TISUBN")]
pub type Tisubn = crate::Reg<tisubn::TisubnSpec>;
#[doc = "1588 Timer Increment Sub-nanoseconds Register"]
pub mod tisubn;
#[doc = "TSH (rw) register accessor: 1588 Timer Seconds High Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tsh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsh`] module"]
#[doc(alias = "TSH")]
pub type Tsh = crate::Reg<tsh::TshSpec>;
#[doc = "1588 Timer Seconds High Register"]
pub mod tsh;
#[doc = "TSL (rw) register accessor: 1588 Timer Seconds Low Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tsl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsl`] module"]
#[doc(alias = "TSL")]
pub type Tsl = crate::Reg<tsl::TslSpec>;
#[doc = "1588 Timer Seconds Low Register"]
pub mod tsl;
#[doc = "TN (rw) register accessor: 1588 Timer Nanoseconds Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tn`] module"]
#[doc(alias = "TN")]
pub type Tn = crate::Reg<tn::TnSpec>;
#[doc = "1588 Timer Nanoseconds Register"]
pub mod tn;
#[doc = "TA (w) register accessor: 1588 Timer Adjust Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ta::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta`] module"]
#[doc(alias = "TA")]
pub type Ta = crate::Reg<ta::TaSpec>;
#[doc = "1588 Timer Adjust Register"]
pub mod ta;
#[doc = "TI (rw) register accessor: 1588 Timer Increment Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ti::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ti::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ti`] module"]
#[doc(alias = "TI")]
pub type Ti = crate::Reg<ti::TiSpec>;
#[doc = "1588 Timer Increment Register"]
pub mod ti;
#[doc = "EFTSL (r) register accessor: PTP Event Frame Transmitted Seconds Low Register\n\nYou can [`read`](crate::Reg::read) this register and get [`eftsl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eftsl`] module"]
#[doc(alias = "EFTSL")]
pub type Eftsl = crate::Reg<eftsl::EftslSpec>;
#[doc = "PTP Event Frame Transmitted Seconds Low Register"]
pub mod eftsl;
#[doc = "EFTN (r) register accessor: PTP Event Frame Transmitted Nanoseconds Register\n\nYou can [`read`](crate::Reg::read) this register and get [`eftn::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eftn`] module"]
#[doc(alias = "EFTN")]
pub type Eftn = crate::Reg<eftn::EftnSpec>;
#[doc = "PTP Event Frame Transmitted Nanoseconds Register"]
pub mod eftn;
#[doc = "EFRSL (r) register accessor: PTP Event Frame Received Seconds Low Register\n\nYou can [`read`](crate::Reg::read) this register and get [`efrsl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efrsl`] module"]
#[doc(alias = "EFRSL")]
pub type Efrsl = crate::Reg<efrsl::EfrslSpec>;
#[doc = "PTP Event Frame Received Seconds Low Register"]
pub mod efrsl;
#[doc = "EFRN (r) register accessor: PTP Event Frame Received Nanoseconds Register\n\nYou can [`read`](crate::Reg::read) this register and get [`efrn::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efrn`] module"]
#[doc(alias = "EFRN")]
pub type Efrn = crate::Reg<efrn::EfrnSpec>;
#[doc = "PTP Event Frame Received Nanoseconds Register"]
pub mod efrn;
#[doc = "PEFTSL (r) register accessor: PTP Peer Event Frame Transmitted Seconds Low Register\n\nYou can [`read`](crate::Reg::read) this register and get [`peftsl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peftsl`] module"]
#[doc(alias = "PEFTSL")]
pub type Peftsl = crate::Reg<peftsl::PeftslSpec>;
#[doc = "PTP Peer Event Frame Transmitted Seconds Low Register"]
pub mod peftsl;
#[doc = "PEFTN (r) register accessor: PTP Peer Event Frame Transmitted Nanoseconds Register\n\nYou can [`read`](crate::Reg::read) this register and get [`peftn::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peftn`] module"]
#[doc(alias = "PEFTN")]
pub type Peftn = crate::Reg<peftn::PeftnSpec>;
#[doc = "PTP Peer Event Frame Transmitted Nanoseconds Register"]
pub mod peftn;
#[doc = "PEFRSL (r) register accessor: PTP Peer Event Frame Received Seconds Low Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pefrsl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pefrsl`] module"]
#[doc(alias = "PEFRSL")]
pub type Pefrsl = crate::Reg<pefrsl::PefrslSpec>;
#[doc = "PTP Peer Event Frame Received Seconds Low Register"]
pub mod pefrsl;
#[doc = "PEFRN (r) register accessor: PTP Peer Event Frame Received Nanoseconds Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pefrn::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pefrn`] module"]
#[doc(alias = "PEFRN")]
pub type Pefrn = crate::Reg<pefrn::PefrnSpec>;
#[doc = "PTP Peer Event Frame Received Nanoseconds Register"]
pub mod pefrn;
#[doc = "RXLPI (r) register accessor: Received LPI Transitions\n\nYou can [`read`](crate::Reg::read) this register and get [`rxlpi::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxlpi`] module"]
#[doc(alias = "RXLPI")]
pub type Rxlpi = crate::Reg<rxlpi::RxlpiSpec>;
#[doc = "Received LPI Transitions"]
pub mod rxlpi;
#[doc = "RXLPITIME (r) register accessor: Received LPI Time\n\nYou can [`read`](crate::Reg::read) this register and get [`rxlpitime::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxlpitime`] module"]
#[doc(alias = "RXLPITIME")]
pub type Rxlpitime = crate::Reg<rxlpitime::RxlpitimeSpec>;
#[doc = "Received LPI Time"]
pub mod rxlpitime;
#[doc = "TXLPI (r) register accessor: Transmit LPI Transitions\n\nYou can [`read`](crate::Reg::read) this register and get [`txlpi::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txlpi`] module"]
#[doc(alias = "TXLPI")]
pub type Txlpi = crate::Reg<txlpi::TxlpiSpec>;
#[doc = "Transmit LPI Transitions"]
pub mod txlpi;
#[doc = "TXLPITIME (r) register accessor: Transmit LPI Time\n\nYou can [`read`](crate::Reg::read) this register and get [`txlpitime::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txlpitime`] module"]
#[doc(alias = "TXLPITIME")]
pub type Txlpitime = crate::Reg<txlpitime::TxlpitimeSpec>;
#[doc = "Transmit LPI Time"]
pub mod txlpitime;
#[doc = "ISRPQ (r) register accessor: Interrupt Status Register Priority Queue (1..5)\n\nYou can [`read`](crate::Reg::read) this register and get [`isrpq::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isrpq`] module"]
#[doc(alias = "ISRPQ")]
pub type Isrpq = crate::Reg<isrpq::IsrpqSpec>;
#[doc = "Interrupt Status Register Priority Queue (1..5)"]
pub mod isrpq;
#[doc = "TBQBAPQ (rw) register accessor: Transmit Buffer Queue Base Address Register Priority Queue (1..5)\n\nYou can [`read`](crate::Reg::read) this register and get [`tbqbapq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbqbapq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbqbapq`] module"]
#[doc(alias = "TBQBAPQ")]
pub type Tbqbapq = crate::Reg<tbqbapq::TbqbapqSpec>;
#[doc = "Transmit Buffer Queue Base Address Register Priority Queue (1..5)"]
pub mod tbqbapq;
#[doc = "RBQBAPQ (rw) register accessor: Receive Buffer Queue Base Address Register Priority Queue (1..5)\n\nYou can [`read`](crate::Reg::read) this register and get [`rbqbapq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rbqbapq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rbqbapq`] module"]
#[doc(alias = "RBQBAPQ")]
pub type Rbqbapq = crate::Reg<rbqbapq::RbqbapqSpec>;
#[doc = "Receive Buffer Queue Base Address Register Priority Queue (1..5)"]
pub mod rbqbapq;
#[doc = "RBSRPQ (rw) register accessor: Receive Buffer Size Register Priority Queue (1..5)\n\nYou can [`read`](crate::Reg::read) this register and get [`rbsrpq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rbsrpq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rbsrpq`] module"]
#[doc(alias = "RBSRPQ")]
pub type Rbsrpq = crate::Reg<rbsrpq::RbsrpqSpec>;
#[doc = "Receive Buffer Size Register Priority Queue (1..5)"]
pub mod rbsrpq;
#[doc = "CBSCR (rw) register accessor: Credit-Based Shaping Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cbscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cbscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cbscr`] module"]
#[doc(alias = "CBSCR")]
pub type Cbscr = crate::Reg<cbscr::CbscrSpec>;
#[doc = "Credit-Based Shaping Control Register"]
pub mod cbscr;
#[doc = "CBSISQA (rw) register accessor: Credit-Based Shaping IdleSlope Register for Queue A\n\nYou can [`read`](crate::Reg::read) this register and get [`cbsisqa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cbsisqa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cbsisqa`] module"]
#[doc(alias = "CBSISQA")]
pub type Cbsisqa = crate::Reg<cbsisqa::CbsisqaSpec>;
#[doc = "Credit-Based Shaping IdleSlope Register for Queue A"]
pub mod cbsisqa;
#[doc = "CBSISQB (rw) register accessor: Credit-Based Shaping IdleSlope Register for Queue B\n\nYou can [`read`](crate::Reg::read) this register and get [`cbsisqb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cbsisqb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cbsisqb`] module"]
#[doc(alias = "CBSISQB")]
pub type Cbsisqb = crate::Reg<cbsisqb::CbsisqbSpec>;
#[doc = "Credit-Based Shaping IdleSlope Register for Queue B"]
pub mod cbsisqb;
#[doc = "ST1RPQ (rw) register accessor: Screening Type 1 Register Priority Queue\n\nYou can [`read`](crate::Reg::read) this register and get [`st1rpq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st1rpq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st1rpq`] module"]
#[doc(alias = "ST1RPQ")]
pub type St1rpq = crate::Reg<st1rpq::St1rpqSpec>;
#[doc = "Screening Type 1 Register Priority Queue"]
pub mod st1rpq;
#[doc = "ST2RPQ (rw) register accessor: Screening Type 2 Register Priority Queue\n\nYou can [`read`](crate::Reg::read) this register and get [`st2rpq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st2rpq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st2rpq`] module"]
#[doc(alias = "ST2RPQ")]
pub type St2rpq = crate::Reg<st2rpq::St2rpqSpec>;
#[doc = "Screening Type 2 Register Priority Queue"]
pub mod st2rpq;
#[doc = "IERPQ (w) register accessor: Interrupt Enable Register Priority Queue (1..5)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ierpq::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ierpq`] module"]
#[doc(alias = "IERPQ")]
pub type Ierpq = crate::Reg<ierpq::IerpqSpec>;
#[doc = "Interrupt Enable Register Priority Queue (1..5)"]
pub mod ierpq;
#[doc = "IDRPQ (w) register accessor: Interrupt Disable Register Priority Queue (1..5)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idrpq::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idrpq`] module"]
#[doc(alias = "IDRPQ")]
pub type Idrpq = crate::Reg<idrpq::IdrpqSpec>;
#[doc = "Interrupt Disable Register Priority Queue (1..5)"]
pub mod idrpq;
#[doc = "IMRPQ (rw) register accessor: Interrupt Mask Register Priority Queue (1..5)\n\nYou can [`read`](crate::Reg::read) this register and get [`imrpq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imrpq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imrpq`] module"]
#[doc(alias = "IMRPQ")]
pub type Imrpq = crate::Reg<imrpq::ImrpqSpec>;
#[doc = "Interrupt Mask Register Priority Queue (1..5)"]
pub mod imrpq;
#[doc = "ST2ER (rw) register accessor: Screening Type 2 Ethertype Register\n\nYou can [`read`](crate::Reg::read) this register and get [`st2er::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st2er::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st2er`] module"]
#[doc(alias = "ST2ER")]
pub type St2er = crate::Reg<st2er::St2erSpec>;
#[doc = "Screening Type 2 Ethertype Register"]
pub mod st2er;
#[doc = "Screening Type 2 Compare Word 0 Register"]
pub use self::gmac_st2cw::GmacSt2cw;
#[doc = r"Cluster"]
#[doc = "Screening Type 2 Compare Word 0 Register"]
pub mod gmac_st2cw;
