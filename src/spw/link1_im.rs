#[doc = "Register `LINK1_IM` reader"]
pub type R = crate::R<Link1ImSpec>;
#[doc = "Register `LINK1_IM` writer"]
pub type W = crate::W<Link1ImSpec>;
#[doc = "Field `DISERR` reader - DisErr"]
pub type DiserrR = crate::BitReader;
#[doc = "Field `DISERR` writer - DisErr"]
pub type DiserrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARERR` reader - ParErr"]
pub type ParerrR = crate::BitReader;
#[doc = "Field `PARERR` writer - ParErr"]
pub type ParerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ESCERR` reader - ESCErr"]
pub type EscerrR = crate::BitReader;
#[doc = "Field `ESCERR` writer - ESCErr"]
pub type EscerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRERR` reader - CrErr"]
pub type CrerrR = crate::BitReader;
#[doc = "Field `CRERR` writer - CrErr"]
pub type CrerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINKABORT` reader - LinkAbort"]
pub type LinkabortR = crate::BitReader;
#[doc = "Field `LINKABORT` writer - LinkAbort"]
pub type LinkabortW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EEPTRANS` reader - EEP transmitted"]
pub type EeptransR = crate::BitReader;
#[doc = "Field `EEPTRANS` writer - EEP transmitted"]
pub type EeptransW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EEPREC` reader - EEP received"]
pub type EeprecR = crate::BitReader;
#[doc = "Field `EEPREC` writer - EEP received"]
pub type EeprecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISCARD` reader - Discard"]
pub type DiscardR = crate::BitReader;
#[doc = "Field `DISCARD` writer - Discard"]
pub type DiscardW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ESCEVENT2` reader - Escape Event 2"]
pub type Escevent2R = crate::BitReader;
#[doc = "Field `ESCEVENT2` writer - Escape Event 2"]
pub type Escevent2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ESCEVENT1` reader - Escape Event 1"]
pub type Escevent1R = crate::BitReader;
#[doc = "Field `ESCEVENT1` writer - Escape Event 1"]
pub type Escevent1W<'a, REG> = crate::BitWriter<'a, REG>;
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
impl W {
    #[doc = "Bit 0 - DisErr"]
    #[inline(always)]
    pub fn diserr(&mut self) -> DiserrW<Link1ImSpec> {
        DiserrW::new(self, 0)
    }
    #[doc = "Bit 1 - ParErr"]
    #[inline(always)]
    pub fn parerr(&mut self) -> ParerrW<Link1ImSpec> {
        ParerrW::new(self, 1)
    }
    #[doc = "Bit 2 - ESCErr"]
    #[inline(always)]
    pub fn escerr(&mut self) -> EscerrW<Link1ImSpec> {
        EscerrW::new(self, 2)
    }
    #[doc = "Bit 3 - CrErr"]
    #[inline(always)]
    pub fn crerr(&mut self) -> CrerrW<Link1ImSpec> {
        CrerrW::new(self, 3)
    }
    #[doc = "Bit 4 - LinkAbort"]
    #[inline(always)]
    pub fn linkabort(&mut self) -> LinkabortW<Link1ImSpec> {
        LinkabortW::new(self, 4)
    }
    #[doc = "Bit 5 - EEP transmitted"]
    #[inline(always)]
    pub fn eeptrans(&mut self) -> EeptransW<Link1ImSpec> {
        EeptransW::new(self, 5)
    }
    #[doc = "Bit 6 - EEP received"]
    #[inline(always)]
    pub fn eeprec(&mut self) -> EeprecW<Link1ImSpec> {
        EeprecW::new(self, 6)
    }
    #[doc = "Bit 7 - Discard"]
    #[inline(always)]
    pub fn discard(&mut self) -> DiscardW<Link1ImSpec> {
        DiscardW::new(self, 7)
    }
    #[doc = "Bit 8 - Escape Event 2"]
    #[inline(always)]
    pub fn escevent2(&mut self) -> Escevent2W<Link1ImSpec> {
        Escevent2W::new(self, 8)
    }
    #[doc = "Bit 9 - Escape Event 1"]
    #[inline(always)]
    pub fn escevent1(&mut self) -> Escevent1W<Link1ImSpec> {
        Escevent1W::new(self, 9)
    }
}
#[doc = "SpW Link 1 Interrupt Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`link1_im::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`link1_im::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Link1ImSpec;
impl crate::RegisterSpec for Link1ImSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`link1_im::R`](R) reader structure"]
impl crate::Readable for Link1ImSpec {}
#[doc = "`write(|w| ..)` method takes [`link1_im::W`](W) writer structure"]
impl crate::Writable for Link1ImSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LINK1_IM to value 0"]
impl crate::Resettable for Link1ImSpec {}
