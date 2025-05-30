#[doc = "Register `NBTP` reader"]
pub type R = crate::R<NbtpSpec>;
#[doc = "Register `NBTP` writer"]
pub type W = crate::W<NbtpSpec>;
#[doc = "Field `NTSEG2` reader - Nominal Time Segment After Sample Point"]
pub type Ntseg2R = crate::FieldReader;
#[doc = "Field `NTSEG2` writer - Nominal Time Segment After Sample Point"]
pub type Ntseg2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `NTSEG1` reader - Nominal Time Segment Before Sample Point"]
pub type Ntseg1R = crate::FieldReader;
#[doc = "Field `NTSEG1` writer - Nominal Time Segment Before Sample Point"]
pub type Ntseg1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NBRP` reader - Nominal Bit Rate Prescaler"]
pub type NbrpR = crate::FieldReader<u16>;
#[doc = "Field `NBRP` writer - Nominal Bit Rate Prescaler"]
pub type NbrpW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `NSJW` reader - Nominal (Re) Synchronization Jump Width"]
pub type NsjwR = crate::FieldReader;
#[doc = "Field `NSJW` writer - Nominal (Re) Synchronization Jump Width"]
pub type NsjwW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - Nominal Time Segment After Sample Point"]
    #[inline(always)]
    pub fn ntseg2(&self) -> Ntseg2R {
        Ntseg2R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:15 - Nominal Time Segment Before Sample Point"]
    #[inline(always)]
    pub fn ntseg1(&self) -> Ntseg1R {
        Ntseg1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:24 - Nominal Bit Rate Prescaler"]
    #[inline(always)]
    pub fn nbrp(&self) -> NbrpR {
        NbrpR::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bits 25:31 - Nominal (Re) Synchronization Jump Width"]
    #[inline(always)]
    pub fn nsjw(&self) -> NsjwR {
        NsjwR::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Nominal Time Segment After Sample Point"]
    #[inline(always)]
    pub fn ntseg2(&mut self) -> Ntseg2W<NbtpSpec> {
        Ntseg2W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Nominal Time Segment Before Sample Point"]
    #[inline(always)]
    pub fn ntseg1(&mut self) -> Ntseg1W<NbtpSpec> {
        Ntseg1W::new(self, 8)
    }
    #[doc = "Bits 16:24 - Nominal Bit Rate Prescaler"]
    #[inline(always)]
    pub fn nbrp(&mut self) -> NbrpW<NbtpSpec> {
        NbrpW::new(self, 16)
    }
    #[doc = "Bits 25:31 - Nominal (Re) Synchronization Jump Width"]
    #[inline(always)]
    pub fn nsjw(&mut self) -> NsjwW<NbtpSpec> {
        NsjwW::new(self, 25)
    }
}
#[doc = "Nominal Bit Timing and Prescaler Register\n\nYou can [`read`](crate::Reg::read) this register and get [`nbtp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nbtp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NbtpSpec;
impl crate::RegisterSpec for NbtpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nbtp::R`](R) reader structure"]
impl crate::Readable for NbtpSpec {}
#[doc = "`write(|w| ..)` method takes [`nbtp::W`](W) writer structure"]
impl crate::Writable for NbtpSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NBTP to value 0"]
impl crate::Resettable for NbtpSpec {}
