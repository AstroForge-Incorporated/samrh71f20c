#[doc = "Register `IMR` reader"]
pub type R = crate::R<ImrSpec>;
#[doc = "Field `MOSCXTS` reader - Main Crystal Oscillator Status Interrupt Mask"]
pub type MoscxtsR = crate::BitReader;
#[doc = "Field `LOCKA` reader - PLLA Lock Interrupt Mask"]
pub type LockaR = crate::BitReader;
#[doc = "Field `LOCKB` reader - PLLB Lock Interrupt Mask"]
pub type LockbR = crate::BitReader;
#[doc = "Field `MCKRDY` reader - Master Clock Ready Interrupt Mask"]
pub type MckrdyR = crate::BitReader;
#[doc = "Field `PCKRDY0` reader - Programmable Clock Ready 0 Interrupt Mask"]
pub type Pckrdy0R = crate::BitReader;
#[doc = "Field `PCKRDY1` reader - Programmable Clock Ready 1 Interrupt Mask"]
pub type Pckrdy1R = crate::BitReader;
#[doc = "Field `PCKRDY2` reader - Programmable Clock Ready 2 Interrupt Mask"]
pub type Pckrdy2R = crate::BitReader;
#[doc = "Field `PCKRDY3` reader - Programmable Clock Ready 3 Interrupt Mask"]
pub type Pckrdy3R = crate::BitReader;
#[doc = "Field `MOSCSELS` reader - Main Clock Source Oscillator Selection Status Interrupt Mask"]
pub type MoscselsR = crate::BitReader;
#[doc = "Field `MOSCRCS` reader - Main RC Status Interrupt Mask"]
pub type MoscrcsR = crate::BitReader;
#[doc = "Field `CFDEV` reader - Clock Failure Detector Event Interrupt Mask"]
pub type CfdevR = crate::BitReader;
#[doc = "Field `EXT32KERR` reader - 32.768 kHz Crystal Oscillator Error Interrupt Mask"]
pub type Ext32kerrR = crate::BitReader;
#[doc = "Field `CPUMON` reader - CPU Clock Monitor Error Interrupt Mask"]
pub type CpumonR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Main Crystal Oscillator Status Interrupt Mask"]
    #[inline(always)]
    pub fn moscxts(&self) -> MoscxtsR {
        MoscxtsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PLLA Lock Interrupt Mask"]
    #[inline(always)]
    pub fn locka(&self) -> LockaR {
        LockaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PLLB Lock Interrupt Mask"]
    #[inline(always)]
    pub fn lockb(&self) -> LockbR {
        LockbR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Master Clock Ready Interrupt Mask"]
    #[inline(always)]
    pub fn mckrdy(&self) -> MckrdyR {
        MckrdyR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Programmable Clock Ready 0 Interrupt Mask"]
    #[inline(always)]
    pub fn pckrdy0(&self) -> Pckrdy0R {
        Pckrdy0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Programmable Clock Ready 1 Interrupt Mask"]
    #[inline(always)]
    pub fn pckrdy1(&self) -> Pckrdy1R {
        Pckrdy1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Programmable Clock Ready 2 Interrupt Mask"]
    #[inline(always)]
    pub fn pckrdy2(&self) -> Pckrdy2R {
        Pckrdy2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Programmable Clock Ready 3 Interrupt Mask"]
    #[inline(always)]
    pub fn pckrdy3(&self) -> Pckrdy3R {
        Pckrdy3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Main Clock Source Oscillator Selection Status Interrupt Mask"]
    #[inline(always)]
    pub fn moscsels(&self) -> MoscselsR {
        MoscselsR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Main RC Status Interrupt Mask"]
    #[inline(always)]
    pub fn moscrcs(&self) -> MoscrcsR {
        MoscrcsR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Clock Failure Detector Event Interrupt Mask"]
    #[inline(always)]
    pub fn cfdev(&self) -> CfdevR {
        CfdevR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 21 - 32.768 kHz Crystal Oscillator Error Interrupt Mask"]
    #[inline(always)]
    pub fn ext32kerr(&self) -> Ext32kerrR {
        Ext32kerrR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - CPU Clock Monitor Error Interrupt Mask"]
    #[inline(always)]
    pub fn cpumon(&self) -> CpumonR {
        CpumonR::new(((self.bits >> 23) & 1) != 0)
    }
}
#[doc = "Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`imr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImrSpec;
impl crate::RegisterSpec for ImrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imr::R`](R) reader structure"]
impl crate::Readable for ImrSpec {}
#[doc = "`reset()` method sets IMR to value 0"]
impl crate::Resettable for ImrSpec {}
