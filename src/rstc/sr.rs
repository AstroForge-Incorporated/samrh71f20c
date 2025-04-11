#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Reset Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rsttypselect {
    #[doc = "0: First powerup reset"]
    GeneralRst = 0,
    #[doc = "2: Watchdog fault occurred"]
    WdtRst = 2,
    #[doc = "3: Processor reset required by the software"]
    SoftRst = 3,
    #[doc = "6: CPU clock failure detection occurred"]
    CpuFailRst = 6,
    #[doc = "7: 32.768 kHz crystal failure detection fault occurred"]
    SlckXtalRst = 7,
}
impl From<Rsttypselect> for u8 {
    #[inline(always)]
    fn from(variant: Rsttypselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rsttypselect {
    type Ux = u8;
}
impl crate::IsEnum for Rsttypselect {}
#[doc = "Field `RSTTYP` reader - Reset Type"]
pub type RsttypR = crate::FieldReader<Rsttypselect>;
impl RsttypR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rsttypselect> {
        match self.bits {
            0 => Some(Rsttypselect::GeneralRst),
            2 => Some(Rsttypselect::WdtRst),
            3 => Some(Rsttypselect::SoftRst),
            6 => Some(Rsttypselect::CpuFailRst),
            7 => Some(Rsttypselect::SlckXtalRst),
            _ => None,
        }
    }
    #[doc = "First powerup reset"]
    #[inline(always)]
    pub fn is_general_rst(&self) -> bool {
        *self == Rsttypselect::GeneralRst
    }
    #[doc = "Watchdog fault occurred"]
    #[inline(always)]
    pub fn is_wdt_rst(&self) -> bool {
        *self == Rsttypselect::WdtRst
    }
    #[doc = "Processor reset required by the software"]
    #[inline(always)]
    pub fn is_soft_rst(&self) -> bool {
        *self == Rsttypselect::SoftRst
    }
    #[doc = "CPU clock failure detection occurred"]
    #[inline(always)]
    pub fn is_cpu_fail_rst(&self) -> bool {
        *self == Rsttypselect::CpuFailRst
    }
    #[doc = "32.768 kHz crystal failure detection fault occurred"]
    #[inline(always)]
    pub fn is_slck_xtal_rst(&self) -> bool {
        *self == Rsttypselect::SlckXtalRst
    }
}
#[doc = "Field `NRSTL` reader - NRST Pin Level"]
pub type NrstlR = crate::BitReader;
#[doc = "Field `SRCMP` reader - Software Reset Command in Progress"]
pub type SrcmpR = crate::BitReader;
impl R {
    #[doc = "Bits 8:10 - Reset Type"]
    #[inline(always)]
    pub fn rsttyp(&self) -> RsttypR {
        RsttypR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 16 - NRST Pin Level"]
    #[inline(always)]
    pub fn nrstl(&self) -> NrstlR {
        NrstlR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Software Reset Command in Progress"]
    #[inline(always)]
    pub fn srcmp(&self) -> SrcmpR {
        SrcmpR::new(((self.bits >> 17) & 1) != 0)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SrSpec {}
