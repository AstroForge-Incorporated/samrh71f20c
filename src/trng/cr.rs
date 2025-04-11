#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `ENABLE` writer - Enables the TRNG to Provide Random Values"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Register Write Access Key\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Wakeyselect {
    #[doc = "5393991: Writing any other value in this field aborts the write operation."]
    Passwd = 5393991,
}
impl From<Wakeyselect> for u32 {
    #[inline(always)]
    fn from(variant: Wakeyselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wakeyselect {
    type Ux = u32;
}
impl crate::IsEnum for Wakeyselect {}
#[doc = "Field `WAKEY` writer - Register Write Access Key"]
pub type WakeyW<'a, REG> = crate::FieldWriter<'a, REG, 24, Wakeyselect>;
impl<'a, REG> WakeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Writing any other value in this field aborts the write operation."]
    #[inline(always)]
    pub fn passwd(self) -> &'a mut crate::W<REG> {
        self.variant(Wakeyselect::Passwd)
    }
}
impl W {
    #[doc = "Bit 0 - Enables the TRNG to Provide Random Values"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<CrSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bits 8:31 - Register Write Access Key"]
    #[inline(always)]
    pub fn wakey(&mut self) -> WakeyW<CrSpec> {
        WakeyW::new(self, 8)
    }
}
#[doc = "Control Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {}
