#[doc = "Register `LINK1_PI_R` reader"]
pub type R = crate::R<Link1PiRSpec>;
#[doc = "Field `DISERR` reader - DisErr"]
pub type DiserrR = crate::BitReader;
#[doc = "Field `PARERR` reader - ParErr"]
pub type ParerrR = crate::BitReader;
#[doc = "Field `ESCERR` reader - ESCErr"]
pub type EscerrR = crate::BitReader;
#[doc = "Field `CRERR` reader - CrErr"]
pub type CrerrR = crate::BitReader;
#[doc = "Field `LINKABORT` reader - LinkAbort"]
pub type LinkabortR = crate::BitReader;
#[doc = "Field `EEPTRANS` reader - EEP transmitted"]
pub type EeptransR = crate::BitReader;
#[doc = "Field `EEPREC` reader - EEP received"]
pub type EeprecR = crate::BitReader;
#[doc = "Field `DISCARD` reader - Discard"]
pub type DiscardR = crate::BitReader;
#[doc = "Field `ESCEVENT2` reader - Escape Event 2"]
pub type Escevent2R = crate::BitReader;
#[doc = "Field `ESCEVENT1` reader - Escape Event 1"]
pub type Escevent1R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - DisErr"]
    #[inline(always)]
    pub fn diserr(&self) -> DiserrR {
        DiserrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ParErr"]
    #[inline(always)]
    pub fn parerr(&self) -> ParerrR {
        ParerrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ESCErr"]
    #[inline(always)]
    pub fn escerr(&self) -> EscerrR {
        EscerrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CrErr"]
    #[inline(always)]
    pub fn crerr(&self) -> CrerrR {
        CrerrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - LinkAbort"]
    #[inline(always)]
    pub fn linkabort(&self) -> LinkabortR {
        LinkabortR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - EEP transmitted"]
    #[inline(always)]
    pub fn eeptrans(&self) -> EeptransR {
        EeptransR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - EEP received"]
    #[inline(always)]
    pub fn eeprec(&self) -> EeprecR {
        EeprecR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Discard"]
    #[inline(always)]
    pub fn discard(&self) -> DiscardR {
        DiscardR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Escape Event 2"]
    #[inline(always)]
    pub fn escevent2(&self) -> Escevent2R {
        Escevent2R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Escape Event 1"]
    #[inline(always)]
    pub fn escevent1(&self) -> Escevent1R {
        Escevent1R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "SpW Link 1 Pending Read Interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`link1_pi_r::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Link1PiRSpec;
impl crate::RegisterSpec for Link1PiRSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`link1_pi_r::R`](R) reader structure"]
impl crate::Readable for Link1PiRSpec {}
#[doc = "`reset()` method sets LINK1_PI_R to value 0"]
impl crate::Resettable for Link1PiRSpec {}
