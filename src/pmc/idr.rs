#[doc = "Register `IDR` writer"]
pub type W = crate::W<IdrSpec>;
#[doc = "Field `MOSCXTS` writer - Main Crystal Oscillator Status Interrupt Disable"]
pub type MoscxtsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCKA` writer - PLLA Lock Interrupt Disable"]
pub type LockaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCKB` writer - PLLB Lock Interrupt Disable"]
pub type LockbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCKRDY` writer - Master Clock Ready Interrupt Disable"]
pub type MckrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCKRDY0` writer - Programmable Clock Ready 0 Interrupt Disable"]
pub type Pckrdy0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCKRDY1` writer - Programmable Clock Ready 1 Interrupt Disable"]
pub type Pckrdy1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCKRDY2` writer - Programmable Clock Ready 2 Interrupt Disable"]
pub type Pckrdy2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCKRDY3` writer - Programmable Clock Ready 3 Interrupt Disable"]
pub type Pckrdy3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MOSCSELS` writer - Main Clock Source Oscillator Selection Status Interrupt Disable"]
pub type MoscselsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MOSCRCS` writer - Main RC Status Interrupt Disable"]
pub type MoscrcsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFDEV` writer - Clock Failure Detector Event Interrupt Disable"]
pub type CfdevW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXT32KERR` writer - 32.768 kHz Crystal Oscillator Error Interrupt Disable"]
pub type Ext32kerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPUMON` writer - CPU Clock Monitor Interrupt Disable"]
pub type CpumonW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Main Crystal Oscillator Status Interrupt Disable"]
    #[inline(always)]
    pub fn moscxts(&mut self) -> MoscxtsW<IdrSpec> {
        MoscxtsW::new(self, 0)
    }
    #[doc = "Bit 1 - PLLA Lock Interrupt Disable"]
    #[inline(always)]
    pub fn locka(&mut self) -> LockaW<IdrSpec> {
        LockaW::new(self, 1)
    }
    #[doc = "Bit 2 - PLLB Lock Interrupt Disable"]
    #[inline(always)]
    pub fn lockb(&mut self) -> LockbW<IdrSpec> {
        LockbW::new(self, 2)
    }
    #[doc = "Bit 3 - Master Clock Ready Interrupt Disable"]
    #[inline(always)]
    pub fn mckrdy(&mut self) -> MckrdyW<IdrSpec> {
        MckrdyW::new(self, 3)
    }
    #[doc = "Bit 8 - Programmable Clock Ready 0 Interrupt Disable"]
    #[inline(always)]
    pub fn pckrdy0(&mut self) -> Pckrdy0W<IdrSpec> {
        Pckrdy0W::new(self, 8)
    }
    #[doc = "Bit 9 - Programmable Clock Ready 1 Interrupt Disable"]
    #[inline(always)]
    pub fn pckrdy1(&mut self) -> Pckrdy1W<IdrSpec> {
        Pckrdy1W::new(self, 9)
    }
    #[doc = "Bit 10 - Programmable Clock Ready 2 Interrupt Disable"]
    #[inline(always)]
    pub fn pckrdy2(&mut self) -> Pckrdy2W<IdrSpec> {
        Pckrdy2W::new(self, 10)
    }
    #[doc = "Bit 11 - Programmable Clock Ready 3 Interrupt Disable"]
    #[inline(always)]
    pub fn pckrdy3(&mut self) -> Pckrdy3W<IdrSpec> {
        Pckrdy3W::new(self, 11)
    }
    #[doc = "Bit 16 - Main Clock Source Oscillator Selection Status Interrupt Disable"]
    #[inline(always)]
    pub fn moscsels(&mut self) -> MoscselsW<IdrSpec> {
        MoscselsW::new(self, 16)
    }
    #[doc = "Bit 17 - Main RC Status Interrupt Disable"]
    #[inline(always)]
    pub fn moscrcs(&mut self) -> MoscrcsW<IdrSpec> {
        MoscrcsW::new(self, 17)
    }
    #[doc = "Bit 18 - Clock Failure Detector Event Interrupt Disable"]
    #[inline(always)]
    pub fn cfdev(&mut self) -> CfdevW<IdrSpec> {
        CfdevW::new(self, 18)
    }
    #[doc = "Bit 21 - 32.768 kHz Crystal Oscillator Error Interrupt Disable"]
    #[inline(always)]
    pub fn ext32kerr(&mut self) -> Ext32kerrW<IdrSpec> {
        Ext32kerrW::new(self, 21)
    }
    #[doc = "Bit 23 - CPU Clock Monitor Interrupt Disable"]
    #[inline(always)]
    pub fn cpumon(&mut self) -> CpumonW<IdrSpec> {
        CpumonW::new(self, 23)
    }
}
#[doc = "Interrupt Disable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdrSpec;
impl crate::RegisterSpec for IdrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`idr::W`](W) writer structure"]
impl crate::Writable for IdrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IDR to value 0"]
impl crate::Resettable for IdrSpec {}
