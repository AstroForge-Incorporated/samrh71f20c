#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Field `MOSCXTS` reader - Main Crystal Oscillator Status"]
pub type MoscxtsR = crate::BitReader;
#[doc = "Field `LOCKA` reader - PLLA Lock Status"]
pub type LockaR = crate::BitReader;
#[doc = "Field `LOCKB` reader - PLLB Lock Status"]
pub type LockbR = crate::BitReader;
#[doc = "Field `MCKRDY` reader - Master Clock Status"]
pub type MckrdyR = crate::BitReader;
#[doc = "Field `OSCSELS` reader - Monitoring Domain Slow Clock Source Oscillator Selection"]
pub type OscselsR = crate::BitReader;
#[doc = "Field `PCKRDY0` reader - Programmable Clock Ready Status"]
pub type Pckrdy0R = crate::BitReader;
#[doc = "Field `PCKRDY1` reader - Programmable Clock Ready Status"]
pub type Pckrdy1R = crate::BitReader;
#[doc = "Field `PCKRDY2` reader - Programmable Clock Ready Status"]
pub type Pckrdy2R = crate::BitReader;
#[doc = "Field `PCKRDY3` reader - Programmable Clock Ready Status"]
pub type Pckrdy3R = crate::BitReader;
#[doc = "Field `MOSCSELS` reader - Main Clock Source Oscillator Selection Status"]
pub type MoscselsR = crate::BitReader;
#[doc = "Field `MOSCRCS` reader - Main RC Oscillator Status"]
pub type MoscrcsR = crate::BitReader;
#[doc = "Field `CFDEV` reader - Clock Failure Detector Event"]
pub type CfdevR = crate::BitReader;
#[doc = "Field `CFDS` reader - Clock Failure Detector Status"]
pub type CfdsR = crate::BitReader;
#[doc = "Field `FOS` reader - Clock Failure Detector Fault Output Status"]
pub type FosR = crate::BitReader;
#[doc = "Field `EXT32KERR` reader - Slow Crystal Oscillator Error"]
pub type Ext32kerrR = crate::BitReader;
#[doc = "Field `CPUMON` reader - CPU Clock Monitor Error"]
pub type CpumonR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Main Crystal Oscillator Status"]
    #[inline(always)]
    pub fn moscxts(&self) -> MoscxtsR {
        MoscxtsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PLLA Lock Status"]
    #[inline(always)]
    pub fn locka(&self) -> LockaR {
        LockaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PLLB Lock Status"]
    #[inline(always)]
    pub fn lockb(&self) -> LockbR {
        LockbR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Master Clock Status"]
    #[inline(always)]
    pub fn mckrdy(&self) -> MckrdyR {
        MckrdyR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - Monitoring Domain Slow Clock Source Oscillator Selection"]
    #[inline(always)]
    pub fn oscsels(&self) -> OscselsR {
        OscselsR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Programmable Clock Ready Status"]
    #[inline(always)]
    pub fn pckrdy0(&self) -> Pckrdy0R {
        Pckrdy0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Programmable Clock Ready Status"]
    #[inline(always)]
    pub fn pckrdy1(&self) -> Pckrdy1R {
        Pckrdy1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Programmable Clock Ready Status"]
    #[inline(always)]
    pub fn pckrdy2(&self) -> Pckrdy2R {
        Pckrdy2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Programmable Clock Ready Status"]
    #[inline(always)]
    pub fn pckrdy3(&self) -> Pckrdy3R {
        Pckrdy3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Main Clock Source Oscillator Selection Status"]
    #[inline(always)]
    pub fn moscsels(&self) -> MoscselsR {
        MoscselsR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Main RC Oscillator Status"]
    #[inline(always)]
    pub fn moscrcs(&self) -> MoscrcsR {
        MoscrcsR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Clock Failure Detector Event"]
    #[inline(always)]
    pub fn cfdev(&self) -> CfdevR {
        CfdevR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Clock Failure Detector Status"]
    #[inline(always)]
    pub fn cfds(&self) -> CfdsR {
        CfdsR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Clock Failure Detector Fault Output Status"]
    #[inline(always)]
    pub fn fos(&self) -> FosR {
        FosR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Slow Crystal Oscillator Error"]
    #[inline(always)]
    pub fn ext32kerr(&self) -> Ext32kerrR {
        Ext32kerrR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - CPU Clock Monitor Error"]
    #[inline(always)]
    pub fn cpumon(&self) -> CpumonR {
        CpumonR::new(((self.bits >> 23) & 1) != 0)
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
