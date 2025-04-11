#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    flex_mr: FlexMr,
    _reserved1: [u8; 0x0c],
    flex_rhr: FlexRhr,
    _reserved2: [u8; 0x0c],
    flex_thr: FlexThr,
    _reserved3: [u8; 0x01dc],
    flex_us_cr: FlexUsCr,
    flex_us_mr: FlexUsMr,
    _reserved_5_flex_us_: [u8; 0x04],
    _reserved_6_flex_us_: [u8; 0x04],
    _reserved_7_flex_us_: [u8; 0x04],
    _reserved_8_flex_us_: [u8; 0x04],
    _reserved_9_flex_us_: [u8; 0x04],
    _reserved_10_flex_us_: [u8; 0x04],
    flex_us_brgr: FlexUsBrgr,
    flex_us_rtor: FlexUsRtor,
    flex_us_ttgr: FlexUsTtgr,
    _reserved14: [u8; 0x14],
    flex_us_fidi: FlexUsFidi,
    flex_us_ner: FlexUsNer,
    _reserved16: [u8; 0x04],
    flex_us_if: FlexUsIf,
    flex_us_man: FlexUsMan,
    flex_us_linmr: FlexUsLinmr,
    flex_us_linir: FlexUsLinir,
    flex_us_linbrr: FlexUsLinbrr,
    flex_us_lonmr: FlexUsLonmr,
    flex_us_lonpr: FlexUsLonpr,
    flex_us_londl: FlexUsLondl,
    flex_us_lonl2hdr: FlexUsLonl2hdr,
    flex_us_lonbl: FlexUsLonbl,
    flex_us_lonb1tx: FlexUsLonb1tx,
    flex_us_lonb1rx: FlexUsLonb1rx,
    flex_us_lonprio: FlexUsLonprio,
    flex_us_idttx: FlexUsIdttx,
    flex_us_idtrx: FlexUsIdtrx,
    flex_us_icdiff: FlexUsIcdiff,
    _reserved32: [u8; 0x04],
    flex_us_cmpr: FlexUsCmpr,
    _reserved33: [u8; 0x0c],
    flex_us_fmr: FlexUsFmr,
    flex_us_flr: FlexUsFlr,
    flex_us_fier: FlexUsFier,
    flex_us_fidr: FlexUsFidr,
    flex_us_fimr: FlexUsFimr,
    flex_us_fesr: FlexUsFesr,
    _reserved39: [u8; 0x2c],
    flex_us_wpmr: FlexUsWpmr,
    flex_us_wpsr: FlexUsWpsr,
    _reserved41: [u8; 0x0114],
    flex_spi_cr: FlexSpiCr,
    flex_spi_mr: FlexSpiMr,
    _reserved_43_flex_spi_: [u8; 0x04],
    _reserved_44_flex_spi_: [u8; 0x04],
    flex_spi_sr: FlexSpiSr,
    flex_spi_ier: FlexSpiIer,
    flex_spi_idr: FlexSpiIdr,
    flex_spi_imr: FlexSpiImr,
    _reserved49: [u8; 0x10],
    flex_spi_csr: [FlexSpiCsr; 4],
    flex_spi_fmr: FlexSpiFmr,
    flex_spi_flr: FlexSpiFlr,
    flex_spi_cmpr: FlexSpiCmpr,
    _reserved53: [u8; 0x98],
    flex_spi_wpmr: FlexSpiWpmr,
    flex_spi_wpsr: FlexSpiWpsr,
    _reserved55: [u8; 0x0114],
    _reserved_55_flex_twi_: [u8; 0x04],
    flex_twi_mmr: FlexTwiMmr,
    flex_twi_smr: FlexTwiSmr,
    flex_twi_iadr: FlexTwiIadr,
    flex_twi_cwgr: FlexTwiCwgr,
    _reserved60: [u8; 0x0c],
    _reserved_60_flex_twi_: [u8; 0x04],
    flex_twi_ier: FlexTwiIer,
    flex_twi_idr: FlexTwiIdr,
    flex_twi_imr: FlexTwiImr,
    _reserved_64_flex_twi_: [u8; 0x04],
    _reserved_65_flex_twi_: [u8; 0x04],
    flex_twi_smbtr: FlexTwiSmbtr,
    _reserved67: [u8; 0x04],
    flex_twi_acr: FlexTwiAcr,
    flex_twi_filtr: FlexTwiFiltr,
    _reserved69: [u8; 0x08],
    flex_twi_fmr: FlexTwiFmr,
    flex_twi_flr: FlexTwiFlr,
    _reserved71: [u8; 0x08],
    flex_twi_fsr: FlexTwiFsr,
    flex_twi_fier: FlexTwiFier,
    flex_twi_fidr: FlexTwiFidr,
    flex_twi_fimr: FlexTwiFimr,
    _reserved75: [u8; 0x60],
    flex_twi_dr: FlexTwiDr,
    _reserved76: [u8; 0x10],
    flex_twi_wpmr: FlexTwiWpmr,
    flex_twi_wpsr: FlexTwiWpsr,
}
impl RegisterBlock {
    #[doc = "0x00 - FLEXCOM Mode Register"]
    #[inline(always)]
    pub const fn flex_mr(&self) -> &FlexMr {
        &self.flex_mr
    }
    #[doc = "0x10 - FLEXCOM Receive Holding Register"]
    #[inline(always)]
    pub const fn flex_rhr(&self) -> &FlexRhr {
        &self.flex_rhr
    }
    #[doc = "0x20 - FLEXCOM Transmit Holding Register"]
    #[inline(always)]
    pub const fn flex_thr(&self) -> &FlexThr {
        &self.flex_thr
    }
    #[doc = "0x200 - USART Control Register"]
    #[inline(always)]
    pub const fn flex_us_cr(&self) -> &FlexUsCr {
        &self.flex_us_cr
    }
    #[doc = "0x204 - USART Mode Register"]
    #[inline(always)]
    pub const fn flex_us_mr(&self) -> &FlexUsMr {
        &self.flex_us_mr
    }
    #[doc = "0x208 - USART Interrupt Enable Register"]
    #[inline(always)]
    pub const fn flex_us_ier_lin_mode_mode(&self) -> &FlexUsIerLinModeMode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(520).cast() }
    }
    #[doc = "0x208 - USART Interrupt Enable Register"]
    #[inline(always)]
    pub const fn flex_us_ier(&self) -> &FlexUsIer {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(520).cast() }
    }
    #[doc = "0x20c - USART Interrupt Disable Register"]
    #[inline(always)]
    pub const fn flex_us_idr_lin_mode_mode(&self) -> &FlexUsIdrLinModeMode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(524).cast() }
    }
    #[doc = "0x20c - USART Interrupt Disable Register"]
    #[inline(always)]
    pub const fn flex_us_idr(&self) -> &FlexUsIdr {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(524).cast() }
    }
    #[doc = "0x210 - USART Interrupt Mask Register"]
    #[inline(always)]
    pub const fn flex_us_imr_lin_mode_mode(&self) -> &FlexUsImrLinModeMode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(528).cast() }
    }
    #[doc = "0x210 - USART Interrupt Mask Register"]
    #[inline(always)]
    pub const fn flex_us_imr(&self) -> &FlexUsImr {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(528).cast() }
    }
    #[doc = "0x214 - USART Channel Status Register"]
    #[inline(always)]
    pub const fn flex_us_csr_lin_mode_mode(&self) -> &FlexUsCsrLinModeMode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(532).cast() }
    }
    #[doc = "0x214 - USART Channel Status Register"]
    #[inline(always)]
    pub const fn flex_us_csr(&self) -> &FlexUsCsr {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(532).cast() }
    }
    #[doc = "0x218 - USART Receive Holding Register"]
    #[inline(always)]
    pub const fn flex_us_rhr_fifo_multi_data_mode(&self) -> &FlexUsRhrFifoMultiDataMode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(536).cast() }
    }
    #[doc = "0x218 - USART Receive Holding Register"]
    #[inline(always)]
    pub const fn flex_us_rhr(&self) -> &FlexUsRhr {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(536).cast() }
    }
    #[doc = "0x21c - USART Transmit Holding Register"]
    #[inline(always)]
    pub const fn flex_us_thr_fifo_multi_data_mode(&self) -> &FlexUsThrFifoMultiDataMode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(540).cast() }
    }
    #[doc = "0x21c - USART Transmit Holding Register"]
    #[inline(always)]
    pub const fn flex_us_thr(&self) -> &FlexUsThr {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(540).cast() }
    }
    #[doc = "0x220 - USART Baud Rate Generator Register"]
    #[inline(always)]
    pub const fn flex_us_brgr(&self) -> &FlexUsBrgr {
        &self.flex_us_brgr
    }
    #[doc = "0x224 - USART Receiver Timeout Register"]
    #[inline(always)]
    pub const fn flex_us_rtor(&self) -> &FlexUsRtor {
        &self.flex_us_rtor
    }
    #[doc = "0x228 - USART Transmitter Timeguard Register"]
    #[inline(always)]
    pub const fn flex_us_ttgr(&self) -> &FlexUsTtgr {
        &self.flex_us_ttgr
    }
    #[doc = "0x240 - USART FI DI Ratio Register"]
    #[inline(always)]
    pub const fn flex_us_fidi(&self) -> &FlexUsFidi {
        &self.flex_us_fidi
    }
    #[doc = "0x244 - USART Number of Errors Register"]
    #[inline(always)]
    pub const fn flex_us_ner(&self) -> &FlexUsNer {
        &self.flex_us_ner
    }
    #[doc = "0x24c - USART IrDA Filter Register"]
    #[inline(always)]
    pub const fn flex_us_if(&self) -> &FlexUsIf {
        &self.flex_us_if
    }
    #[doc = "0x250 - USART Manchester Configuration Register"]
    #[inline(always)]
    pub const fn flex_us_man(&self) -> &FlexUsMan {
        &self.flex_us_man
    }
    #[doc = "0x254 - USART LIN Mode Register"]
    #[inline(always)]
    pub const fn flex_us_linmr(&self) -> &FlexUsLinmr {
        &self.flex_us_linmr
    }
    #[doc = "0x258 - USART LIN Identifier Register"]
    #[inline(always)]
    pub const fn flex_us_linir(&self) -> &FlexUsLinir {
        &self.flex_us_linir
    }
    #[doc = "0x25c - USART LIN Baud Rate Register"]
    #[inline(always)]
    pub const fn flex_us_linbrr(&self) -> &FlexUsLinbrr {
        &self.flex_us_linbrr
    }
    #[doc = "0x260 - USART LON Mode Register"]
    #[inline(always)]
    pub const fn flex_us_lonmr(&self) -> &FlexUsLonmr {
        &self.flex_us_lonmr
    }
    #[doc = "0x264 - USART LON Preamble Register"]
    #[inline(always)]
    pub const fn flex_us_lonpr(&self) -> &FlexUsLonpr {
        &self.flex_us_lonpr
    }
    #[doc = "0x268 - USART LON Data Length Register"]
    #[inline(always)]
    pub const fn flex_us_londl(&self) -> &FlexUsLondl {
        &self.flex_us_londl
    }
    #[doc = "0x26c - USART LON L2HDR Register"]
    #[inline(always)]
    pub const fn flex_us_lonl2hdr(&self) -> &FlexUsLonl2hdr {
        &self.flex_us_lonl2hdr
    }
    #[doc = "0x270 - USART LON Backlog Register"]
    #[inline(always)]
    pub const fn flex_us_lonbl(&self) -> &FlexUsLonbl {
        &self.flex_us_lonbl
    }
    #[doc = "0x274 - USART LON Beta1 Tx Register"]
    #[inline(always)]
    pub const fn flex_us_lonb1tx(&self) -> &FlexUsLonb1tx {
        &self.flex_us_lonb1tx
    }
    #[doc = "0x278 - USART LON Beta1 Rx Register"]
    #[inline(always)]
    pub const fn flex_us_lonb1rx(&self) -> &FlexUsLonb1rx {
        &self.flex_us_lonb1rx
    }
    #[doc = "0x27c - USART LON Priority Register"]
    #[inline(always)]
    pub const fn flex_us_lonprio(&self) -> &FlexUsLonprio {
        &self.flex_us_lonprio
    }
    #[doc = "0x280 - USART LON IDT Tx Register"]
    #[inline(always)]
    pub const fn flex_us_idttx(&self) -> &FlexUsIdttx {
        &self.flex_us_idttx
    }
    #[doc = "0x284 - USART LON IDT Rx Register"]
    #[inline(always)]
    pub const fn flex_us_idtrx(&self) -> &FlexUsIdtrx {
        &self.flex_us_idtrx
    }
    #[doc = "0x288 - USART IC DIFF Register"]
    #[inline(always)]
    pub const fn flex_us_icdiff(&self) -> &FlexUsIcdiff {
        &self.flex_us_icdiff
    }
    #[doc = "0x290 - USART Comparison Register"]
    #[inline(always)]
    pub const fn flex_us_cmpr(&self) -> &FlexUsCmpr {
        &self.flex_us_cmpr
    }
    #[doc = "0x2a0 - USART FIFO Mode Register"]
    #[inline(always)]
    pub const fn flex_us_fmr(&self) -> &FlexUsFmr {
        &self.flex_us_fmr
    }
    #[doc = "0x2a4 - USART FIFO Level Register"]
    #[inline(always)]
    pub const fn flex_us_flr(&self) -> &FlexUsFlr {
        &self.flex_us_flr
    }
    #[doc = "0x2a8 - USART FIFO Interrupt Enable Register"]
    #[inline(always)]
    pub const fn flex_us_fier(&self) -> &FlexUsFier {
        &self.flex_us_fier
    }
    #[doc = "0x2ac - USART FIFO Interrupt Disable Register"]
    #[inline(always)]
    pub const fn flex_us_fidr(&self) -> &FlexUsFidr {
        &self.flex_us_fidr
    }
    #[doc = "0x2b0 - USART FIFO Interrupt Mask Register"]
    #[inline(always)]
    pub const fn flex_us_fimr(&self) -> &FlexUsFimr {
        &self.flex_us_fimr
    }
    #[doc = "0x2b4 - USART FIFO Event Status Register"]
    #[inline(always)]
    pub const fn flex_us_fesr(&self) -> &FlexUsFesr {
        &self.flex_us_fesr
    }
    #[doc = "0x2e4 - USART Write Protection Mode Register"]
    #[inline(always)]
    pub const fn flex_us_wpmr(&self) -> &FlexUsWpmr {
        &self.flex_us_wpmr
    }
    #[doc = "0x2e8 - USART Write Protection Status Register"]
    #[inline(always)]
    pub const fn flex_us_wpsr(&self) -> &FlexUsWpsr {
        &self.flex_us_wpsr
    }
    #[doc = "0x400 - SPI Control Register"]
    #[inline(always)]
    pub const fn flex_spi_cr(&self) -> &FlexSpiCr {
        &self.flex_spi_cr
    }
    #[doc = "0x404 - SPI Mode Register"]
    #[inline(always)]
    pub const fn flex_spi_mr(&self) -> &FlexSpiMr {
        &self.flex_spi_mr
    }
    #[doc = "0x408 - SPI Receive Data Register"]
    #[inline(always)]
    pub const fn flex_spi_rdr_fifo_multi_data_16_mode(&self) -> &FlexSpiRdrFifoMultiData16Mode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1032).cast() }
    }
    #[doc = "0x408 - SPI Receive Data Register"]
    #[inline(always)]
    pub const fn flex_spi_rdr_fifo_multi_data_8_mode(&self) -> &FlexSpiRdrFifoMultiData8Mode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1032).cast() }
    }
    #[doc = "0x408 - SPI Receive Data Register"]
    #[inline(always)]
    pub const fn flex_spi_rdr(&self) -> &FlexSpiRdr {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1032).cast() }
    }
    #[doc = "0x40c - SPI Transmit Data Register"]
    #[inline(always)]
    pub const fn flex_spi_tdr_fifo_multi_data_mode(&self) -> &FlexSpiTdrFifoMultiDataMode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1036).cast() }
    }
    #[doc = "0x40c - SPI Transmit Data Register"]
    #[inline(always)]
    pub const fn flex_spi_tdr(&self) -> &FlexSpiTdr {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1036).cast() }
    }
    #[doc = "0x410 - SPI Status Register"]
    #[inline(always)]
    pub const fn flex_spi_sr(&self) -> &FlexSpiSr {
        &self.flex_spi_sr
    }
    #[doc = "0x414 - SPI Interrupt Enable Register"]
    #[inline(always)]
    pub const fn flex_spi_ier(&self) -> &FlexSpiIer {
        &self.flex_spi_ier
    }
    #[doc = "0x418 - SPI Interrupt Disable Register"]
    #[inline(always)]
    pub const fn flex_spi_idr(&self) -> &FlexSpiIdr {
        &self.flex_spi_idr
    }
    #[doc = "0x41c - SPI Interrupt Mask Register"]
    #[inline(always)]
    pub const fn flex_spi_imr(&self) -> &FlexSpiImr {
        &self.flex_spi_imr
    }
    #[doc = "0x430..0x440 - SPI Chip Select Register"]
    #[inline(always)]
    pub const fn flex_spi_csr(&self, n: usize) -> &FlexSpiCsr {
        &self.flex_spi_csr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x430..0x440 - SPI Chip Select Register"]
    #[inline(always)]
    pub fn flex_spi_csr_iter(&self) -> impl Iterator<Item = &FlexSpiCsr> {
        self.flex_spi_csr.iter()
    }
    #[doc = "0x440 - SPI FIFO Mode Register"]
    #[inline(always)]
    pub const fn flex_spi_fmr(&self) -> &FlexSpiFmr {
        &self.flex_spi_fmr
    }
    #[doc = "0x444 - SPI FIFO Level Register"]
    #[inline(always)]
    pub const fn flex_spi_flr(&self) -> &FlexSpiFlr {
        &self.flex_spi_flr
    }
    #[doc = "0x448 - SPI Comparison Register"]
    #[inline(always)]
    pub const fn flex_spi_cmpr(&self) -> &FlexSpiCmpr {
        &self.flex_spi_cmpr
    }
    #[doc = "0x4e4 - SPI Write Protection Mode Register"]
    #[inline(always)]
    pub const fn flex_spi_wpmr(&self) -> &FlexSpiWpmr {
        &self.flex_spi_wpmr
    }
    #[doc = "0x4e8 - SPI Write Protection Status Register"]
    #[inline(always)]
    pub const fn flex_spi_wpsr(&self) -> &FlexSpiWpsr {
        &self.flex_spi_wpsr
    }
    #[doc = "0x600 - TWI Control Register"]
    #[inline(always)]
    pub const fn flex_twi_cr_fifo_enabled_mode(&self) -> &FlexTwiCrFifoEnabledMode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1536).cast() }
    }
    #[doc = "0x600 - TWI Control Register"]
    #[inline(always)]
    pub const fn flex_twi_cr(&self) -> &FlexTwiCr {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1536).cast() }
    }
    #[doc = "0x604 - TWI Master Mode Register"]
    #[inline(always)]
    pub const fn flex_twi_mmr(&self) -> &FlexTwiMmr {
        &self.flex_twi_mmr
    }
    #[doc = "0x608 - TWI Slave Mode Register"]
    #[inline(always)]
    pub const fn flex_twi_smr(&self) -> &FlexTwiSmr {
        &self.flex_twi_smr
    }
    #[doc = "0x60c - TWI Internal Address Register"]
    #[inline(always)]
    pub const fn flex_twi_iadr(&self) -> &FlexTwiIadr {
        &self.flex_twi_iadr
    }
    #[doc = "0x610 - TWI Clock Waveform Generator Register"]
    #[inline(always)]
    pub const fn flex_twi_cwgr(&self) -> &FlexTwiCwgr {
        &self.flex_twi_cwgr
    }
    #[doc = "0x620 - TWI Status Register"]
    #[inline(always)]
    pub const fn flex_twi_sr_fifo_enabled_mode(&self) -> &FlexTwiSrFifoEnabledMode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1568).cast() }
    }
    #[doc = "0x620 - TWI Status Register"]
    #[inline(always)]
    pub const fn flex_twi_sr(&self) -> &FlexTwiSr {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1568).cast() }
    }
    #[doc = "0x624 - TWI Interrupt Enable Register"]
    #[inline(always)]
    pub const fn flex_twi_ier(&self) -> &FlexTwiIer {
        &self.flex_twi_ier
    }
    #[doc = "0x628 - TWI Interrupt Disable Register"]
    #[inline(always)]
    pub const fn flex_twi_idr(&self) -> &FlexTwiIdr {
        &self.flex_twi_idr
    }
    #[doc = "0x62c - TWI Interrupt Mask Register"]
    #[inline(always)]
    pub const fn flex_twi_imr(&self) -> &FlexTwiImr {
        &self.flex_twi_imr
    }
    #[doc = "0x630 - TWI Receive Holding Register"]
    #[inline(always)]
    pub const fn flex_twi_rhr_fifo_enabled_mode(&self) -> &FlexTwiRhrFifoEnabledMode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1584).cast() }
    }
    #[doc = "0x630 - TWI Receive Holding Register"]
    #[inline(always)]
    pub const fn flex_twi_rhr(&self) -> &FlexTwiRhr {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1584).cast() }
    }
    #[doc = "0x634 - TWI Transmit Holding Register"]
    #[inline(always)]
    pub const fn flex_twi_thr_fifo_enabled_mode(&self) -> &FlexTwiThrFifoEnabledMode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1588).cast() }
    }
    #[doc = "0x634 - TWI Transmit Holding Register"]
    #[inline(always)]
    pub const fn flex_twi_thr(&self) -> &FlexTwiThr {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1588).cast() }
    }
    #[doc = "0x638 - TWI SMBus Timing Register"]
    #[inline(always)]
    pub const fn flex_twi_smbtr(&self) -> &FlexTwiSmbtr {
        &self.flex_twi_smbtr
    }
    #[doc = "0x640 - TWI Alternative Command Register"]
    #[inline(always)]
    pub const fn flex_twi_acr(&self) -> &FlexTwiAcr {
        &self.flex_twi_acr
    }
    #[doc = "0x644 - TWI Filter Register"]
    #[inline(always)]
    pub const fn flex_twi_filtr(&self) -> &FlexTwiFiltr {
        &self.flex_twi_filtr
    }
    #[doc = "0x650 - TWI FIFO Mode Register"]
    #[inline(always)]
    pub const fn flex_twi_fmr(&self) -> &FlexTwiFmr {
        &self.flex_twi_fmr
    }
    #[doc = "0x654 - TWI FIFO Level Register"]
    #[inline(always)]
    pub const fn flex_twi_flr(&self) -> &FlexTwiFlr {
        &self.flex_twi_flr
    }
    #[doc = "0x660 - TWI FIFO Status Register"]
    #[inline(always)]
    pub const fn flex_twi_fsr(&self) -> &FlexTwiFsr {
        &self.flex_twi_fsr
    }
    #[doc = "0x664 - TWI FIFO Interrupt Enable Register"]
    #[inline(always)]
    pub const fn flex_twi_fier(&self) -> &FlexTwiFier {
        &self.flex_twi_fier
    }
    #[doc = "0x668 - TWI FIFO Interrupt Disable Register"]
    #[inline(always)]
    pub const fn flex_twi_fidr(&self) -> &FlexTwiFidr {
        &self.flex_twi_fidr
    }
    #[doc = "0x66c - TWI FIFO Interrupt Mask Register"]
    #[inline(always)]
    pub const fn flex_twi_fimr(&self) -> &FlexTwiFimr {
        &self.flex_twi_fimr
    }
    #[doc = "0x6d0 - TWI Debug Register"]
    #[inline(always)]
    pub const fn flex_twi_dr(&self) -> &FlexTwiDr {
        &self.flex_twi_dr
    }
    #[doc = "0x6e4 - TWI Write Protection Mode Register"]
    #[inline(always)]
    pub const fn flex_twi_wpmr(&self) -> &FlexTwiWpmr {
        &self.flex_twi_wpmr
    }
    #[doc = "0x6e8 - TWI Write Protection Status Register"]
    #[inline(always)]
    pub const fn flex_twi_wpsr(&self) -> &FlexTwiWpsr {
        &self.flex_twi_wpsr
    }
}
#[doc = "FLEX_MR (rw) register accessor: FLEXCOM Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_mr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_mr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_mr`] module"]
#[doc(alias = "FLEX_MR")]
pub type FlexMr = crate::Reg<flex_mr::FlexMrSpec>;
#[doc = "FLEXCOM Mode Register"]
pub mod flex_mr;
#[doc = "FLEX_RHR (r) register accessor: FLEXCOM Receive Holding Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_rhr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_rhr`] module"]
#[doc(alias = "FLEX_RHR")]
pub type FlexRhr = crate::Reg<flex_rhr::FlexRhrSpec>;
#[doc = "FLEXCOM Receive Holding Register"]
pub mod flex_rhr;
#[doc = "FLEX_THR (rw) register accessor: FLEXCOM Transmit Holding Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_thr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_thr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_thr`] module"]
#[doc(alias = "FLEX_THR")]
pub type FlexThr = crate::Reg<flex_thr::FlexThrSpec>;
#[doc = "FLEXCOM Transmit Holding Register"]
pub mod flex_thr;
#[doc = "FLEX_US_CR (w) register accessor: USART Control Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_us_cr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_us_cr`] module"]
#[doc(alias = "FLEX_US_CR")]
pub type FlexUsCr = crate::Reg<flex_us_cr::FlexUsCrSpec>;
#[doc = "USART Control Register"]
pub mod flex_us_cr;
#[doc = "FLEX_US_MR (rw) register accessor: USART Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_us_mr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_us_mr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_us_mr`] module"]
#[doc(alias = "FLEX_US_MR")]
pub type FlexUsMr = crate::Reg<flex_us_mr::FlexUsMrSpec>;
#[doc = "USART Mode Register"]
pub mod flex_us_mr;
#[doc = "FLEX_US_IER (w) register accessor: USART Interrupt Enable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_us_ier::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_us_ier`] module"]
#[doc(alias = "FLEX_US_IER")]
pub type FlexUsIer = crate::Reg<flex_us_ier::FlexUsIerSpec>;
#[doc = "USART Interrupt Enable Register"]
pub mod flex_us_ier;
#[doc = "FLEX_US_IER_LIN_MODE_MODE (w) register accessor: USART Interrupt Enable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_us_ier_lin_mode_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_us_ier_lin_mode_mode`] module"]
#[doc(alias = "FLEX_US_IER_LIN_MODE_MODE")]
pub type FlexUsIerLinModeMode = crate::Reg<flex_us_ier_lin_mode_mode::FlexUsIerLinModeModeSpec>;
#[doc = "USART Interrupt Enable Register"]
pub mod flex_us_ier_lin_mode_mode;
#[doc = "FLEX_US_IDR (w) register accessor: USART Interrupt Disable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_us_idr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_us_idr`] module"]
#[doc(alias = "FLEX_US_IDR")]
pub type FlexUsIdr = crate::Reg<flex_us_idr::FlexUsIdrSpec>;
#[doc = "USART Interrupt Disable Register"]
pub mod flex_us_idr;
#[doc = "FLEX_US_IDR_LIN_MODE_MODE (w) register accessor: USART Interrupt Disable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_us_idr_lin_mode_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_us_idr_lin_mode_mode`] module"]
#[doc(alias = "FLEX_US_IDR_LIN_MODE_MODE")]
pub type FlexUsIdrLinModeMode = crate::Reg<flex_us_idr_lin_mode_mode::FlexUsIdrLinModeModeSpec>;
#[doc = "USART Interrupt Disable Register"]
pub mod flex_us_idr_lin_mode_mode;
#[doc = "FLEX_US_IMR (r) register accessor: USART Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_us_imr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_us_imr`] module"]
#[doc(alias = "FLEX_US_IMR")]
pub type FlexUsImr = crate::Reg<flex_us_imr::FlexUsImrSpec>;
#[doc = "USART Interrupt Mask Register"]
pub mod flex_us_imr;
#[doc = "FLEX_US_IMR_LIN_MODE_MODE (r) register accessor: USART Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_us_imr_lin_mode_mode::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_us_imr_lin_mode_mode`] module"]
#[doc(alias = "FLEX_US_IMR_LIN_MODE_MODE")]
pub type FlexUsImrLinModeMode = crate::Reg<flex_us_imr_lin_mode_mode::FlexUsImrLinModeModeSpec>;
#[doc = "USART Interrupt Mask Register"]
pub mod flex_us_imr_lin_mode_mode;
#[doc = "FLEX_US_CSR (r) register accessor: USART Channel Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_us_csr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_us_csr`] module"]
#[doc(alias = "FLEX_US_CSR")]
pub type FlexUsCsr = crate::Reg<flex_us_csr::FlexUsCsrSpec>;
#[doc = "USART Channel Status Register"]
pub mod flex_us_csr;
#[doc = "FLEX_US_CSR_LIN_MODE_MODE (r) register accessor: USART Channel Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_us_csr_lin_mode_mode::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_us_csr_lin_mode_mode`] module"]
#[doc(alias = "FLEX_US_CSR_LIN_MODE_MODE")]
pub type FlexUsCsrLinModeMode = crate::Reg<flex_us_csr_lin_mode_mode::FlexUsCsrLinModeModeSpec>;
#[doc = "USART Channel Status Register"]
pub mod flex_us_csr_lin_mode_mode;
#[doc = "FLEX_US_RHR (r) register accessor: USART Receive Holding Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_us_rhr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_us_rhr`] module"]
#[doc(alias = "FLEX_US_RHR")]
pub type FlexUsRhr = crate::Reg<flex_us_rhr::FlexUsRhrSpec>;
#[doc = "USART Receive Holding Register"]
pub mod flex_us_rhr;
#[doc = "FLEX_US_RHR_FIFO_MULTI_DATA_MODE (r) register accessor: USART Receive Holding Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_us_rhr_fifo_multi_data_mode::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_us_rhr_fifo_multi_data_mode`] module"]
#[doc(alias = "FLEX_US_RHR_FIFO_MULTI_DATA_MODE")]
pub type FlexUsRhrFifoMultiDataMode =
    crate::Reg<flex_us_rhr_fifo_multi_data_mode::FlexUsRhrFifoMultiDataModeSpec>;
#[doc = "USART Receive Holding Register"]
pub mod flex_us_rhr_fifo_multi_data_mode;
#[doc = "FLEX_US_THR (w) register accessor: USART Transmit Holding Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_us_thr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_us_thr`] module"]
#[doc(alias = "FLEX_US_THR")]
pub type FlexUsThr = crate::Reg<flex_us_thr::FlexUsThrSpec>;
#[doc = "USART Transmit Holding Register"]
pub mod flex_us_thr;
#[doc = "FLEX_US_THR_FIFO_MULTI_DATA_MODE (w) register accessor: USART Transmit Holding Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_us_thr_fifo_multi_data_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_us_thr_fifo_multi_data_mode`] module"]
#[doc(alias = "FLEX_US_THR_FIFO_MULTI_DATA_MODE")]
pub type FlexUsThrFifoMultiDataMode =
    crate::Reg<flex_us_thr_fifo_multi_data_mode::FlexUsThrFifoMultiDataModeSpec>;
#[doc = "USART Transmit Holding Register"]
pub mod flex_us_thr_fifo_multi_data_mode;
#[doc = "FLEX_US_BRGR (rw) register accessor: USART Baud Rate Generator Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_us_brgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_us_brgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_us_brgr`] module"]
#[doc(alias = "FLEX_US_BRGR")]
pub type FlexUsBrgr = crate::Reg<flex_us_brgr::FlexUsBrgrSpec>;
#[doc = "USART Baud Rate Generator Register"]
pub mod flex_us_brgr;
#[doc = "FLEX_US_RTOR (rw) register accessor: USART Receiver Timeout Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_us_rtor::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_us_rtor::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_us_rtor`] module"]
#[doc(alias = "FLEX_US_RTOR")]
pub type FlexUsRtor = crate::Reg<flex_us_rtor::FlexUsRtorSpec>;
#[doc = "USART Receiver Timeout Register"]
pub mod flex_us_rtor;
#[doc = "FLEX_US_TTGR (rw) register accessor: USART Transmitter Timeguard Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_us_ttgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_us_ttgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_us_ttgr`] module"]
#[doc(alias = "FLEX_US_TTGR")]
pub type FlexUsTtgr = crate::Reg<flex_us_ttgr::FlexUsTtgrSpec>;
#[doc = "USART Transmitter Timeguard Register"]
pub mod flex_us_ttgr;
#[doc = "FLEX_US_FIDI (rw) register accessor: USART FI DI Ratio Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_us_fidi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_us_fidi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_us_fidi`] module"]
#[doc(alias = "FLEX_US_FIDI")]
pub type FlexUsFidi = crate::Reg<flex_us_fidi::FlexUsFidiSpec>;
#[doc = "USART FI DI Ratio Register"]
pub mod flex_us_fidi;
#[doc = "FLEX_US_NER (r) register accessor: USART Number of Errors Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_us_ner::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_us_ner`] module"]
#[doc(alias = "FLEX_US_NER")]
pub type FlexUsNer = crate::Reg<flex_us_ner::FlexUsNerSpec>;
#[doc = "USART Number of Errors Register"]
pub mod flex_us_ner;
#[doc = "FLEX_US_IF (rw) register accessor: USART IrDA Filter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_us_if::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_us_if::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_us_if`] module"]
#[doc(alias = "FLEX_US_IF")]
pub type FlexUsIf = crate::Reg<flex_us_if::FlexUsIfSpec>;
#[doc = "USART IrDA Filter Register"]
pub mod flex_us_if;
#[doc = "FLEX_US_MAN (rw) register accessor: USART Manchester Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_us_man::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_us_man::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_us_man`] module"]
#[doc(alias = "FLEX_US_MAN")]
pub type FlexUsMan = crate::Reg<flex_us_man::FlexUsManSpec>;
#[doc = "USART Manchester Configuration Register"]
pub mod flex_us_man;
#[doc = "FLEX_US_LINMR (rw) register accessor: USART LIN Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_us_linmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_us_linmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_us_linmr`] module"]
#[doc(alias = "FLEX_US_LINMR")]
pub type FlexUsLinmr = crate::Reg<flex_us_linmr::FlexUsLinmrSpec>;
#[doc = "USART LIN Mode Register"]
pub mod flex_us_linmr;
#[doc = "FLEX_US_LINIR (rw) register accessor: USART LIN Identifier Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_us_linir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_us_linir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_us_linir`] module"]
#[doc(alias = "FLEX_US_LINIR")]
pub type FlexUsLinir = crate::Reg<flex_us_linir::FlexUsLinirSpec>;
#[doc = "USART LIN Identifier Register"]
pub mod flex_us_linir;
#[doc = "FLEX_US_LINBRR (r) register accessor: USART LIN Baud Rate Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_us_linbrr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_us_linbrr`] module"]
#[doc(alias = "FLEX_US_LINBRR")]
pub type FlexUsLinbrr = crate::Reg<flex_us_linbrr::FlexUsLinbrrSpec>;
#[doc = "USART LIN Baud Rate Register"]
pub mod flex_us_linbrr;
#[doc = "FLEX_US_LONMR (rw) register accessor: USART LON Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_us_lonmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_us_lonmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_us_lonmr`] module"]
#[doc(alias = "FLEX_US_LONMR")]
pub type FlexUsLonmr = crate::Reg<flex_us_lonmr::FlexUsLonmrSpec>;
#[doc = "USART LON Mode Register"]
pub mod flex_us_lonmr;
#[doc = "FLEX_US_LONPR (rw) register accessor: USART LON Preamble Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_us_lonpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_us_lonpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_us_lonpr`] module"]
#[doc(alias = "FLEX_US_LONPR")]
pub type FlexUsLonpr = crate::Reg<flex_us_lonpr::FlexUsLonprSpec>;
#[doc = "USART LON Preamble Register"]
pub mod flex_us_lonpr;
#[doc = "FLEX_US_LONDL (rw) register accessor: USART LON Data Length Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_us_londl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_us_londl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_us_londl`] module"]
#[doc(alias = "FLEX_US_LONDL")]
pub type FlexUsLondl = crate::Reg<flex_us_londl::FlexUsLondlSpec>;
#[doc = "USART LON Data Length Register"]
pub mod flex_us_londl;
#[doc = "FLEX_US_LONL2HDR (rw) register accessor: USART LON L2HDR Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_us_lonl2hdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_us_lonl2hdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_us_lonl2hdr`] module"]
#[doc(alias = "FLEX_US_LONL2HDR")]
pub type FlexUsLonl2hdr = crate::Reg<flex_us_lonl2hdr::FlexUsLonl2hdrSpec>;
#[doc = "USART LON L2HDR Register"]
pub mod flex_us_lonl2hdr;
#[doc = "FLEX_US_LONBL (r) register accessor: USART LON Backlog Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_us_lonbl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_us_lonbl`] module"]
#[doc(alias = "FLEX_US_LONBL")]
pub type FlexUsLonbl = crate::Reg<flex_us_lonbl::FlexUsLonblSpec>;
#[doc = "USART LON Backlog Register"]
pub mod flex_us_lonbl;
#[doc = "FLEX_US_LONB1TX (rw) register accessor: USART LON Beta1 Tx Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_us_lonb1tx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_us_lonb1tx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_us_lonb1tx`] module"]
#[doc(alias = "FLEX_US_LONB1TX")]
pub type FlexUsLonb1tx = crate::Reg<flex_us_lonb1tx::FlexUsLonb1txSpec>;
#[doc = "USART LON Beta1 Tx Register"]
pub mod flex_us_lonb1tx;
#[doc = "FLEX_US_LONB1RX (rw) register accessor: USART LON Beta1 Rx Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_us_lonb1rx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_us_lonb1rx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_us_lonb1rx`] module"]
#[doc(alias = "FLEX_US_LONB1RX")]
pub type FlexUsLonb1rx = crate::Reg<flex_us_lonb1rx::FlexUsLonb1rxSpec>;
#[doc = "USART LON Beta1 Rx Register"]
pub mod flex_us_lonb1rx;
#[doc = "FLEX_US_LONPRIO (rw) register accessor: USART LON Priority Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_us_lonprio::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_us_lonprio::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_us_lonprio`] module"]
#[doc(alias = "FLEX_US_LONPRIO")]
pub type FlexUsLonprio = crate::Reg<flex_us_lonprio::FlexUsLonprioSpec>;
#[doc = "USART LON Priority Register"]
pub mod flex_us_lonprio;
#[doc = "FLEX_US_IDTTX (rw) register accessor: USART LON IDT Tx Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_us_idttx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_us_idttx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_us_idttx`] module"]
#[doc(alias = "FLEX_US_IDTTX")]
pub type FlexUsIdttx = crate::Reg<flex_us_idttx::FlexUsIdttxSpec>;
#[doc = "USART LON IDT Tx Register"]
pub mod flex_us_idttx;
#[doc = "FLEX_US_IDTRX (rw) register accessor: USART LON IDT Rx Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_us_idtrx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_us_idtrx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_us_idtrx`] module"]
#[doc(alias = "FLEX_US_IDTRX")]
pub type FlexUsIdtrx = crate::Reg<flex_us_idtrx::FlexUsIdtrxSpec>;
#[doc = "USART LON IDT Rx Register"]
pub mod flex_us_idtrx;
#[doc = "FLEX_US_ICDIFF (rw) register accessor: USART IC DIFF Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_us_icdiff::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_us_icdiff::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_us_icdiff`] module"]
#[doc(alias = "FLEX_US_ICDIFF")]
pub type FlexUsIcdiff = crate::Reg<flex_us_icdiff::FlexUsIcdiffSpec>;
#[doc = "USART IC DIFF Register"]
pub mod flex_us_icdiff;
#[doc = "FLEX_US_CMPR (rw) register accessor: USART Comparison Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_us_cmpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_us_cmpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_us_cmpr`] module"]
#[doc(alias = "FLEX_US_CMPR")]
pub type FlexUsCmpr = crate::Reg<flex_us_cmpr::FlexUsCmprSpec>;
#[doc = "USART Comparison Register"]
pub mod flex_us_cmpr;
#[doc = "FLEX_US_FMR (rw) register accessor: USART FIFO Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_us_fmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_us_fmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_us_fmr`] module"]
#[doc(alias = "FLEX_US_FMR")]
pub type FlexUsFmr = crate::Reg<flex_us_fmr::FlexUsFmrSpec>;
#[doc = "USART FIFO Mode Register"]
pub mod flex_us_fmr;
#[doc = "FLEX_US_FLR (r) register accessor: USART FIFO Level Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_us_flr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_us_flr`] module"]
#[doc(alias = "FLEX_US_FLR")]
pub type FlexUsFlr = crate::Reg<flex_us_flr::FlexUsFlrSpec>;
#[doc = "USART FIFO Level Register"]
pub mod flex_us_flr;
#[doc = "FLEX_US_FIER (w) register accessor: USART FIFO Interrupt Enable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_us_fier::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_us_fier`] module"]
#[doc(alias = "FLEX_US_FIER")]
pub type FlexUsFier = crate::Reg<flex_us_fier::FlexUsFierSpec>;
#[doc = "USART FIFO Interrupt Enable Register"]
pub mod flex_us_fier;
#[doc = "FLEX_US_FIDR (w) register accessor: USART FIFO Interrupt Disable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_us_fidr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_us_fidr`] module"]
#[doc(alias = "FLEX_US_FIDR")]
pub type FlexUsFidr = crate::Reg<flex_us_fidr::FlexUsFidrSpec>;
#[doc = "USART FIFO Interrupt Disable Register"]
pub mod flex_us_fidr;
#[doc = "FLEX_US_FIMR (r) register accessor: USART FIFO Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_us_fimr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_us_fimr`] module"]
#[doc(alias = "FLEX_US_FIMR")]
pub type FlexUsFimr = crate::Reg<flex_us_fimr::FlexUsFimrSpec>;
#[doc = "USART FIFO Interrupt Mask Register"]
pub mod flex_us_fimr;
#[doc = "FLEX_US_FESR (r) register accessor: USART FIFO Event Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_us_fesr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_us_fesr`] module"]
#[doc(alias = "FLEX_US_FESR")]
pub type FlexUsFesr = crate::Reg<flex_us_fesr::FlexUsFesrSpec>;
#[doc = "USART FIFO Event Status Register"]
pub mod flex_us_fesr;
#[doc = "FLEX_US_WPMR (rw) register accessor: USART Write Protection Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_us_wpmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_us_wpmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_us_wpmr`] module"]
#[doc(alias = "FLEX_US_WPMR")]
pub type FlexUsWpmr = crate::Reg<flex_us_wpmr::FlexUsWpmrSpec>;
#[doc = "USART Write Protection Mode Register"]
pub mod flex_us_wpmr;
#[doc = "FLEX_US_WPSR (r) register accessor: USART Write Protection Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_us_wpsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_us_wpsr`] module"]
#[doc(alias = "FLEX_US_WPSR")]
pub type FlexUsWpsr = crate::Reg<flex_us_wpsr::FlexUsWpsrSpec>;
#[doc = "USART Write Protection Status Register"]
pub mod flex_us_wpsr;
#[doc = "FLEX_SPI_CR (w) register accessor: SPI Control Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_spi_cr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_spi_cr`] module"]
#[doc(alias = "FLEX_SPI_CR")]
pub type FlexSpiCr = crate::Reg<flex_spi_cr::FlexSpiCrSpec>;
#[doc = "SPI Control Register"]
pub mod flex_spi_cr;
#[doc = "FLEX_SPI_MR (rw) register accessor: SPI Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_spi_mr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_spi_mr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_spi_mr`] module"]
#[doc(alias = "FLEX_SPI_MR")]
pub type FlexSpiMr = crate::Reg<flex_spi_mr::FlexSpiMrSpec>;
#[doc = "SPI Mode Register"]
pub mod flex_spi_mr;
#[doc = "FLEX_SPI_RDR (r) register accessor: SPI Receive Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_spi_rdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_spi_rdr`] module"]
#[doc(alias = "FLEX_SPI_RDR")]
pub type FlexSpiRdr = crate::Reg<flex_spi_rdr::FlexSpiRdrSpec>;
#[doc = "SPI Receive Data Register"]
pub mod flex_spi_rdr;
#[doc = "FLEX_SPI_RDR_FIFO_MULTI_DATA_8_MODE (r) register accessor: SPI Receive Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_spi_rdr_fifo_multi_data_8_mode::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_spi_rdr_fifo_multi_data_8_mode`] module"]
#[doc(alias = "FLEX_SPI_RDR_FIFO_MULTI_DATA_8_MODE")]
pub type FlexSpiRdrFifoMultiData8Mode =
    crate::Reg<flex_spi_rdr_fifo_multi_data_8_mode::FlexSpiRdrFifoMultiData8ModeSpec>;
#[doc = "SPI Receive Data Register"]
pub mod flex_spi_rdr_fifo_multi_data_8_mode;
#[doc = "FLEX_SPI_RDR_FIFO_MULTI_DATA_16_MODE (r) register accessor: SPI Receive Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_spi_rdr_fifo_multi_data_16_mode::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_spi_rdr_fifo_multi_data_16_mode`] module"]
#[doc(alias = "FLEX_SPI_RDR_FIFO_MULTI_DATA_16_MODE")]
pub type FlexSpiRdrFifoMultiData16Mode =
    crate::Reg<flex_spi_rdr_fifo_multi_data_16_mode::FlexSpiRdrFifoMultiData16ModeSpec>;
#[doc = "SPI Receive Data Register"]
pub mod flex_spi_rdr_fifo_multi_data_16_mode;
#[doc = "FLEX_SPI_TDR (w) register accessor: SPI Transmit Data Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_spi_tdr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_spi_tdr`] module"]
#[doc(alias = "FLEX_SPI_TDR")]
pub type FlexSpiTdr = crate::Reg<flex_spi_tdr::FlexSpiTdrSpec>;
#[doc = "SPI Transmit Data Register"]
pub mod flex_spi_tdr;
#[doc = "FLEX_SPI_TDR_FIFO_MULTI_DATA_MODE (w) register accessor: SPI Transmit Data Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_spi_tdr_fifo_multi_data_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_spi_tdr_fifo_multi_data_mode`] module"]
#[doc(alias = "FLEX_SPI_TDR_FIFO_MULTI_DATA_MODE")]
pub type FlexSpiTdrFifoMultiDataMode =
    crate::Reg<flex_spi_tdr_fifo_multi_data_mode::FlexSpiTdrFifoMultiDataModeSpec>;
#[doc = "SPI Transmit Data Register"]
pub mod flex_spi_tdr_fifo_multi_data_mode;
#[doc = "FLEX_SPI_SR (r) register accessor: SPI Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_spi_sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_spi_sr`] module"]
#[doc(alias = "FLEX_SPI_SR")]
pub type FlexSpiSr = crate::Reg<flex_spi_sr::FlexSpiSrSpec>;
#[doc = "SPI Status Register"]
pub mod flex_spi_sr;
#[doc = "FLEX_SPI_IER (w) register accessor: SPI Interrupt Enable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_spi_ier::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_spi_ier`] module"]
#[doc(alias = "FLEX_SPI_IER")]
pub type FlexSpiIer = crate::Reg<flex_spi_ier::FlexSpiIerSpec>;
#[doc = "SPI Interrupt Enable Register"]
pub mod flex_spi_ier;
#[doc = "FLEX_SPI_IDR (w) register accessor: SPI Interrupt Disable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_spi_idr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_spi_idr`] module"]
#[doc(alias = "FLEX_SPI_IDR")]
pub type FlexSpiIdr = crate::Reg<flex_spi_idr::FlexSpiIdrSpec>;
#[doc = "SPI Interrupt Disable Register"]
pub mod flex_spi_idr;
#[doc = "FLEX_SPI_IMR (r) register accessor: SPI Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_spi_imr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_spi_imr`] module"]
#[doc(alias = "FLEX_SPI_IMR")]
pub type FlexSpiImr = crate::Reg<flex_spi_imr::FlexSpiImrSpec>;
#[doc = "SPI Interrupt Mask Register"]
pub mod flex_spi_imr;
#[doc = "FLEX_SPI_CSR (rw) register accessor: SPI Chip Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_spi_csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_spi_csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_spi_csr`] module"]
#[doc(alias = "FLEX_SPI_CSR")]
pub type FlexSpiCsr = crate::Reg<flex_spi_csr::FlexSpiCsrSpec>;
#[doc = "SPI Chip Select Register"]
pub mod flex_spi_csr;
#[doc = "FLEX_SPI_FMR (rw) register accessor: SPI FIFO Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_spi_fmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_spi_fmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_spi_fmr`] module"]
#[doc(alias = "FLEX_SPI_FMR")]
pub type FlexSpiFmr = crate::Reg<flex_spi_fmr::FlexSpiFmrSpec>;
#[doc = "SPI FIFO Mode Register"]
pub mod flex_spi_fmr;
#[doc = "FLEX_SPI_FLR (r) register accessor: SPI FIFO Level Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_spi_flr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_spi_flr`] module"]
#[doc(alias = "FLEX_SPI_FLR")]
pub type FlexSpiFlr = crate::Reg<flex_spi_flr::FlexSpiFlrSpec>;
#[doc = "SPI FIFO Level Register"]
pub mod flex_spi_flr;
#[doc = "FLEX_SPI_CMPR (rw) register accessor: SPI Comparison Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_spi_cmpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_spi_cmpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_spi_cmpr`] module"]
#[doc(alias = "FLEX_SPI_CMPR")]
pub type FlexSpiCmpr = crate::Reg<flex_spi_cmpr::FlexSpiCmprSpec>;
#[doc = "SPI Comparison Register"]
pub mod flex_spi_cmpr;
#[doc = "FLEX_SPI_WPMR (rw) register accessor: SPI Write Protection Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_spi_wpmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_spi_wpmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_spi_wpmr`] module"]
#[doc(alias = "FLEX_SPI_WPMR")]
pub type FlexSpiWpmr = crate::Reg<flex_spi_wpmr::FlexSpiWpmrSpec>;
#[doc = "SPI Write Protection Mode Register"]
pub mod flex_spi_wpmr;
#[doc = "FLEX_SPI_WPSR (r) register accessor: SPI Write Protection Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_spi_wpsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_spi_wpsr`] module"]
#[doc(alias = "FLEX_SPI_WPSR")]
pub type FlexSpiWpsr = crate::Reg<flex_spi_wpsr::FlexSpiWpsrSpec>;
#[doc = "SPI Write Protection Status Register"]
pub mod flex_spi_wpsr;
#[doc = "FLEX_TWI_CR (w) register accessor: TWI Control Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_twi_cr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_twi_cr`] module"]
#[doc(alias = "FLEX_TWI_CR")]
pub type FlexTwiCr = crate::Reg<flex_twi_cr::FlexTwiCrSpec>;
#[doc = "TWI Control Register"]
pub mod flex_twi_cr;
#[doc = "FLEX_TWI_CR_FIFO_ENABLED_MODE (w) register accessor: TWI Control Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_twi_cr_fifo_enabled_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_twi_cr_fifo_enabled_mode`] module"]
#[doc(alias = "FLEX_TWI_CR_FIFO_ENABLED_MODE")]
pub type FlexTwiCrFifoEnabledMode =
    crate::Reg<flex_twi_cr_fifo_enabled_mode::FlexTwiCrFifoEnabledModeSpec>;
#[doc = "TWI Control Register"]
pub mod flex_twi_cr_fifo_enabled_mode;
#[doc = "FLEX_TWI_MMR (rw) register accessor: TWI Master Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_twi_mmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_twi_mmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_twi_mmr`] module"]
#[doc(alias = "FLEX_TWI_MMR")]
pub type FlexTwiMmr = crate::Reg<flex_twi_mmr::FlexTwiMmrSpec>;
#[doc = "TWI Master Mode Register"]
pub mod flex_twi_mmr;
#[doc = "FLEX_TWI_SMR (rw) register accessor: TWI Slave Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_twi_smr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_twi_smr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_twi_smr`] module"]
#[doc(alias = "FLEX_TWI_SMR")]
pub type FlexTwiSmr = crate::Reg<flex_twi_smr::FlexTwiSmrSpec>;
#[doc = "TWI Slave Mode Register"]
pub mod flex_twi_smr;
#[doc = "FLEX_TWI_IADR (rw) register accessor: TWI Internal Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_twi_iadr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_twi_iadr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_twi_iadr`] module"]
#[doc(alias = "FLEX_TWI_IADR")]
pub type FlexTwiIadr = crate::Reg<flex_twi_iadr::FlexTwiIadrSpec>;
#[doc = "TWI Internal Address Register"]
pub mod flex_twi_iadr;
#[doc = "FLEX_TWI_CWGR (rw) register accessor: TWI Clock Waveform Generator Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_twi_cwgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_twi_cwgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_twi_cwgr`] module"]
#[doc(alias = "FLEX_TWI_CWGR")]
pub type FlexTwiCwgr = crate::Reg<flex_twi_cwgr::FlexTwiCwgrSpec>;
#[doc = "TWI Clock Waveform Generator Register"]
pub mod flex_twi_cwgr;
#[doc = "FLEX_TWI_SR (r) register accessor: TWI Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_twi_sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_twi_sr`] module"]
#[doc(alias = "FLEX_TWI_SR")]
pub type FlexTwiSr = crate::Reg<flex_twi_sr::FlexTwiSrSpec>;
#[doc = "TWI Status Register"]
pub mod flex_twi_sr;
#[doc = "FLEX_TWI_SR_FIFO_ENABLED_MODE (r) register accessor: TWI Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_twi_sr_fifo_enabled_mode::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_twi_sr_fifo_enabled_mode`] module"]
#[doc(alias = "FLEX_TWI_SR_FIFO_ENABLED_MODE")]
pub type FlexTwiSrFifoEnabledMode =
    crate::Reg<flex_twi_sr_fifo_enabled_mode::FlexTwiSrFifoEnabledModeSpec>;
#[doc = "TWI Status Register"]
pub mod flex_twi_sr_fifo_enabled_mode;
#[doc = "FLEX_TWI_IER (w) register accessor: TWI Interrupt Enable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_twi_ier::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_twi_ier`] module"]
#[doc(alias = "FLEX_TWI_IER")]
pub type FlexTwiIer = crate::Reg<flex_twi_ier::FlexTwiIerSpec>;
#[doc = "TWI Interrupt Enable Register"]
pub mod flex_twi_ier;
#[doc = "FLEX_TWI_IDR (w) register accessor: TWI Interrupt Disable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_twi_idr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_twi_idr`] module"]
#[doc(alias = "FLEX_TWI_IDR")]
pub type FlexTwiIdr = crate::Reg<flex_twi_idr::FlexTwiIdrSpec>;
#[doc = "TWI Interrupt Disable Register"]
pub mod flex_twi_idr;
#[doc = "FLEX_TWI_IMR (r) register accessor: TWI Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_twi_imr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_twi_imr`] module"]
#[doc(alias = "FLEX_TWI_IMR")]
pub type FlexTwiImr = crate::Reg<flex_twi_imr::FlexTwiImrSpec>;
#[doc = "TWI Interrupt Mask Register"]
pub mod flex_twi_imr;
#[doc = "FLEX_TWI_RHR (r) register accessor: TWI Receive Holding Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_twi_rhr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_twi_rhr`] module"]
#[doc(alias = "FLEX_TWI_RHR")]
pub type FlexTwiRhr = crate::Reg<flex_twi_rhr::FlexTwiRhrSpec>;
#[doc = "TWI Receive Holding Register"]
pub mod flex_twi_rhr;
#[doc = "FLEX_TWI_RHR_FIFO_ENABLED_MODE (r) register accessor: TWI Receive Holding Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_twi_rhr_fifo_enabled_mode::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_twi_rhr_fifo_enabled_mode`] module"]
#[doc(alias = "FLEX_TWI_RHR_FIFO_ENABLED_MODE")]
pub type FlexTwiRhrFifoEnabledMode =
    crate::Reg<flex_twi_rhr_fifo_enabled_mode::FlexTwiRhrFifoEnabledModeSpec>;
#[doc = "TWI Receive Holding Register"]
pub mod flex_twi_rhr_fifo_enabled_mode;
#[doc = "FLEX_TWI_THR (w) register accessor: TWI Transmit Holding Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_twi_thr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_twi_thr`] module"]
#[doc(alias = "FLEX_TWI_THR")]
pub type FlexTwiThr = crate::Reg<flex_twi_thr::FlexTwiThrSpec>;
#[doc = "TWI Transmit Holding Register"]
pub mod flex_twi_thr;
#[doc = "FLEX_TWI_THR_FIFO_ENABLED_MODE (w) register accessor: TWI Transmit Holding Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_twi_thr_fifo_enabled_mode::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_twi_thr_fifo_enabled_mode`] module"]
#[doc(alias = "FLEX_TWI_THR_FIFO_ENABLED_MODE")]
pub type FlexTwiThrFifoEnabledMode =
    crate::Reg<flex_twi_thr_fifo_enabled_mode::FlexTwiThrFifoEnabledModeSpec>;
#[doc = "TWI Transmit Holding Register"]
pub mod flex_twi_thr_fifo_enabled_mode;
#[doc = "FLEX_TWI_SMBTR (rw) register accessor: TWI SMBus Timing Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_twi_smbtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_twi_smbtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_twi_smbtr`] module"]
#[doc(alias = "FLEX_TWI_SMBTR")]
pub type FlexTwiSmbtr = crate::Reg<flex_twi_smbtr::FlexTwiSmbtrSpec>;
#[doc = "TWI SMBus Timing Register"]
pub mod flex_twi_smbtr;
#[doc = "FLEX_TWI_ACR (rw) register accessor: TWI Alternative Command Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_twi_acr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_twi_acr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_twi_acr`] module"]
#[doc(alias = "FLEX_TWI_ACR")]
pub type FlexTwiAcr = crate::Reg<flex_twi_acr::FlexTwiAcrSpec>;
#[doc = "TWI Alternative Command Register"]
pub mod flex_twi_acr;
#[doc = "FLEX_TWI_FILTR (rw) register accessor: TWI Filter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_twi_filtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_twi_filtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_twi_filtr`] module"]
#[doc(alias = "FLEX_TWI_FILTR")]
pub type FlexTwiFiltr = crate::Reg<flex_twi_filtr::FlexTwiFiltrSpec>;
#[doc = "TWI Filter Register"]
pub mod flex_twi_filtr;
#[doc = "FLEX_TWI_FMR (rw) register accessor: TWI FIFO Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_twi_fmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_twi_fmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_twi_fmr`] module"]
#[doc(alias = "FLEX_TWI_FMR")]
pub type FlexTwiFmr = crate::Reg<flex_twi_fmr::FlexTwiFmrSpec>;
#[doc = "TWI FIFO Mode Register"]
pub mod flex_twi_fmr;
#[doc = "FLEX_TWI_FLR (r) register accessor: TWI FIFO Level Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_twi_flr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_twi_flr`] module"]
#[doc(alias = "FLEX_TWI_FLR")]
pub type FlexTwiFlr = crate::Reg<flex_twi_flr::FlexTwiFlrSpec>;
#[doc = "TWI FIFO Level Register"]
pub mod flex_twi_flr;
#[doc = "FLEX_TWI_FSR (r) register accessor: TWI FIFO Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_twi_fsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_twi_fsr`] module"]
#[doc(alias = "FLEX_TWI_FSR")]
pub type FlexTwiFsr = crate::Reg<flex_twi_fsr::FlexTwiFsrSpec>;
#[doc = "TWI FIFO Status Register"]
pub mod flex_twi_fsr;
#[doc = "FLEX_TWI_FIER (w) register accessor: TWI FIFO Interrupt Enable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_twi_fier::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_twi_fier`] module"]
#[doc(alias = "FLEX_TWI_FIER")]
pub type FlexTwiFier = crate::Reg<flex_twi_fier::FlexTwiFierSpec>;
#[doc = "TWI FIFO Interrupt Enable Register"]
pub mod flex_twi_fier;
#[doc = "FLEX_TWI_FIDR (w) register accessor: TWI FIFO Interrupt Disable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_twi_fidr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_twi_fidr`] module"]
#[doc(alias = "FLEX_TWI_FIDR")]
pub type FlexTwiFidr = crate::Reg<flex_twi_fidr::FlexTwiFidrSpec>;
#[doc = "TWI FIFO Interrupt Disable Register"]
pub mod flex_twi_fidr;
#[doc = "FLEX_TWI_FIMR (r) register accessor: TWI FIFO Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_twi_fimr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_twi_fimr`] module"]
#[doc(alias = "FLEX_TWI_FIMR")]
pub type FlexTwiFimr = crate::Reg<flex_twi_fimr::FlexTwiFimrSpec>;
#[doc = "TWI FIFO Interrupt Mask Register"]
pub mod flex_twi_fimr;
#[doc = "FLEX_TWI_DR (r) register accessor: TWI Debug Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_twi_dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_twi_dr`] module"]
#[doc(alias = "FLEX_TWI_DR")]
pub type FlexTwiDr = crate::Reg<flex_twi_dr::FlexTwiDrSpec>;
#[doc = "TWI Debug Register"]
pub mod flex_twi_dr;
#[doc = "FLEX_TWI_WPMR (rw) register accessor: TWI Write Protection Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_twi_wpmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_twi_wpmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_twi_wpmr`] module"]
#[doc(alias = "FLEX_TWI_WPMR")]
pub type FlexTwiWpmr = crate::Reg<flex_twi_wpmr::FlexTwiWpmrSpec>;
#[doc = "TWI Write Protection Mode Register"]
pub mod flex_twi_wpmr;
#[doc = "FLEX_TWI_WPSR (r) register accessor: TWI Write Protection Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_twi_wpsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flex_twi_wpsr`] module"]
#[doc(alias = "FLEX_TWI_WPSR")]
pub type FlexTwiWpsr = crate::Reg<flex_twi_wpsr::FlexTwiWpsrSpec>;
#[doc = "TWI Write Protection Status Register"]
pub mod flex_twi_wpsr;
