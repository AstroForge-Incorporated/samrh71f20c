#[doc = "Register `SCSR` reader"]
pub type R = crate::R<ScsrSpec>;
#[doc = "Field `PCK0` reader - Programmable Clock 0 Output Status"]
pub type Pck0R = crate::BitReader;
#[doc = "Field `PCK1` reader - Programmable Clock 1 Output Status"]
pub type Pck1R = crate::BitReader;
#[doc = "Field `PCK2` reader - Programmable Clock 2 Output Status"]
pub type Pck2R = crate::BitReader;
#[doc = "Field `PCK3` reader - Programmable Clock 3 Output Status"]
pub type Pck3R = crate::BitReader;
impl R {
    #[doc = "Bit 8 - Programmable Clock 0 Output Status"]
    #[inline(always)]
    pub fn pck0(&self) -> Pck0R {
        Pck0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Programmable Clock 1 Output Status"]
    #[inline(always)]
    pub fn pck1(&self) -> Pck1R {
        Pck1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Programmable Clock 2 Output Status"]
    #[inline(always)]
    pub fn pck2(&self) -> Pck2R {
        Pck2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Programmable Clock 3 Output Status"]
    #[inline(always)]
    pub fn pck3(&self) -> Pck3R {
        Pck3R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "System Clock Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScsrSpec;
impl crate::RegisterSpec for ScsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scsr::R`](R) reader structure"]
impl crate::Readable for ScsrSpec {}
#[doc = "`reset()` method sets SCSR to value 0"]
impl crate::Resettable for ScsrSpec {}
