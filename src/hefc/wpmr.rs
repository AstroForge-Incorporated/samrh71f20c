#[doc = "Register `WPMR` reader"]
pub type R = crate::R<WpmrSpec>;
#[doc = "Register `WPMR` writer"]
pub type W = crate::W<WpmrSpec>;
#[doc = "Field `WPEN` reader - Write Protection Enable"]
pub type WpenR = crate::BitReader;
#[doc = "Field `WPEN` writer - Write Protection Enable"]
pub type WpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPNVMWP` reader - GPNVM Bit Write Protection"]
pub type GpnvmwpR = crate::BitReader;
#[doc = "Field `GPNVMWP` writer - GPNVM Bit Write Protection"]
pub type GpnvmwpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCKWP` reader - Lock Bit Write Protection"]
pub type LockwpR = crate::BitReader;
#[doc = "Field `LOCKWP` writer - Lock Bit Write Protection"]
pub type LockwpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERASEWP` reader - Page, Sector and Plane Write Protection"]
pub type ErasewpR = crate::BitReader;
#[doc = "Field `ERASEWP` writer - Page, Sector and Plane Write Protection"]
pub type ErasewpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USER` reader - User Signature Write Protection"]
pub type UserR = crate::BitReader;
#[doc = "Field `USER` writer - User Signature Write Protection"]
pub type UserW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Write Protection Key\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Wpkeyselect {
    #[doc = "4539971: Writing any other value in this field aborts the write operation.Always reads as 0."]
    Passwd = 4539971,
}
impl From<Wpkeyselect> for u32 {
    #[inline(always)]
    fn from(variant: Wpkeyselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wpkeyselect {
    type Ux = u32;
}
impl crate::IsEnum for Wpkeyselect {}
#[doc = "Field `WPKEY` reader - Write Protection Key"]
pub type WpkeyR = crate::FieldReader<Wpkeyselect>;
impl WpkeyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Wpkeyselect> {
        match self.bits {
            4539971 => Some(Wpkeyselect::Passwd),
            _ => None,
        }
    }
    #[doc = "Writing any other value in this field aborts the write operation.Always reads as 0."]
    #[inline(always)]
    pub fn is_passwd(&self) -> bool {
        *self == Wpkeyselect::Passwd
    }
}
#[doc = "Field `WPKEY` writer - Write Protection Key"]
pub type WpkeyW<'a, REG> = crate::FieldWriter<'a, REG, 24, Wpkeyselect>;
impl<'a, REG> WpkeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Writing any other value in this field aborts the write operation.Always reads as 0."]
    #[inline(always)]
    pub fn passwd(self) -> &'a mut crate::W<REG> {
        self.variant(Wpkeyselect::Passwd)
    }
}
impl R {
    #[doc = "Bit 0 - Write Protection Enable"]
    #[inline(always)]
    pub fn wpen(&self) -> WpenR {
        WpenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPNVM Bit Write Protection"]
    #[inline(always)]
    pub fn gpnvmwp(&self) -> GpnvmwpR {
        GpnvmwpR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Lock Bit Write Protection"]
    #[inline(always)]
    pub fn lockwp(&self) -> LockwpR {
        LockwpR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Page, Sector and Plane Write Protection"]
    #[inline(always)]
    pub fn erasewp(&self) -> ErasewpR {
        ErasewpR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - User Signature Write Protection"]
    #[inline(always)]
    pub fn user(&self) -> UserR {
        UserR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:31 - Write Protection Key"]
    #[inline(always)]
    pub fn wpkey(&self) -> WpkeyR {
        WpkeyR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Write Protection Enable"]
    #[inline(always)]
    pub fn wpen(&mut self) -> WpenW<WpmrSpec> {
        WpenW::new(self, 0)
    }
    #[doc = "Bit 1 - GPNVM Bit Write Protection"]
    #[inline(always)]
    pub fn gpnvmwp(&mut self) -> GpnvmwpW<WpmrSpec> {
        GpnvmwpW::new(self, 1)
    }
    #[doc = "Bit 2 - Lock Bit Write Protection"]
    #[inline(always)]
    pub fn lockwp(&mut self) -> LockwpW<WpmrSpec> {
        LockwpW::new(self, 2)
    }
    #[doc = "Bit 3 - Page, Sector and Plane Write Protection"]
    #[inline(always)]
    pub fn erasewp(&mut self) -> ErasewpW<WpmrSpec> {
        ErasewpW::new(self, 3)
    }
    #[doc = "Bit 4 - User Signature Write Protection"]
    #[inline(always)]
    pub fn user(&mut self) -> UserW<WpmrSpec> {
        UserW::new(self, 4)
    }
    #[doc = "Bits 8:31 - Write Protection Key"]
    #[inline(always)]
    pub fn wpkey(&mut self) -> WpkeyW<WpmrSpec> {
        WpkeyW::new(self, 8)
    }
}
#[doc = "Write Protection Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wpmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WpmrSpec;
impl crate::RegisterSpec for WpmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wpmr::R`](R) reader structure"]
impl crate::Readable for WpmrSpec {}
#[doc = "`write(|w| ..)` method takes [`wpmr::W`](W) writer structure"]
impl crate::Writable for WpmrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WPMR to value 0"]
impl crate::Resettable for WpmrSpec {}
