#[doc = "Register `PKTTX1_STATUS` reader"]
pub type R = crate::R<Pkttx1StatusSpec>;
#[doc = "Register `PKTTX1_STATUS` writer"]
pub type W = crate::W<Pkttx1StatusSpec>;
#[doc = "Field `ARM` reader - Armed"]
pub type ArmR = crate::BitReader;
#[doc = "Field `ARM` writer - Armed"]
pub type ArmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACT` reader - Active"]
pub type ActR = crate::BitReader;
#[doc = "Field `ACT` writer - Active"]
pub type ActW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PENDING` reader - Pending"]
pub type PendingR = crate::BitReader;
#[doc = "Field `PENDING` writer - Pending"]
pub type PendingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEACT` reader - Deactivating"]
pub type DeactR = crate::BitReader;
#[doc = "Field `DEACT` writer - Deactivating"]
pub type DeactW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Previous\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Prevselect {
    #[doc = "0: No information. Field not locked."]
    Noinfo = 0,
    #[doc = "1: Last send list fully done"]
    Lastsendlistok = 1,
    #[doc = "2: Aborted due to memory access error."]
    Abortedmemerr = 2,
    #[doc = "3: Aborted by new send list."]
    Abortednewsd = 3,
    #[doc = "4: Aborted by direct user command."]
    Abortedusercmd = 4,
    #[doc = "5: Aborted by timeout."]
    Abortedtimeout = 5,
}
impl From<Prevselect> for u8 {
    #[inline(always)]
    fn from(variant: Prevselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Prevselect {
    type Ux = u8;
}
impl crate::IsEnum for Prevselect {}
#[doc = "Field `PREV` reader - Previous"]
pub type PrevR = crate::FieldReader<Prevselect>;
impl PrevR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Prevselect> {
        match self.bits {
            0 => Some(Prevselect::Noinfo),
            1 => Some(Prevselect::Lastsendlistok),
            2 => Some(Prevselect::Abortedmemerr),
            3 => Some(Prevselect::Abortednewsd),
            4 => Some(Prevselect::Abortedusercmd),
            5 => Some(Prevselect::Abortedtimeout),
            _ => None,
        }
    }
    #[doc = "No information. Field not locked."]
    #[inline(always)]
    pub fn is_noinfo(&self) -> bool {
        *self == Prevselect::Noinfo
    }
    #[doc = "Last send list fully done"]
    #[inline(always)]
    pub fn is_lastsendlistok(&self) -> bool {
        *self == Prevselect::Lastsendlistok
    }
    #[doc = "Aborted due to memory access error."]
    #[inline(always)]
    pub fn is_abortedmemerr(&self) -> bool {
        *self == Prevselect::Abortedmemerr
    }
    #[doc = "Aborted by new send list."]
    #[inline(always)]
    pub fn is_abortednewsd(&self) -> bool {
        *self == Prevselect::Abortednewsd
    }
    #[doc = "Aborted by direct user command."]
    #[inline(always)]
    pub fn is_abortedusercmd(&self) -> bool {
        *self == Prevselect::Abortedusercmd
    }
    #[doc = "Aborted by timeout."]
    #[inline(always)]
    pub fn is_abortedtimeout(&self) -> bool {
        *self == Prevselect::Abortedtimeout
    }
}
#[doc = "Field `PREV` writer - Previous"]
pub type PrevW<'a, REG> = crate::FieldWriter<'a, REG, 3, Prevselect>;
impl<'a, REG> PrevW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No information. Field not locked."]
    #[inline(always)]
    pub fn noinfo(self) -> &'a mut crate::W<REG> {
        self.variant(Prevselect::Noinfo)
    }
    #[doc = "Last send list fully done"]
    #[inline(always)]
    pub fn lastsendlistok(self) -> &'a mut crate::W<REG> {
        self.variant(Prevselect::Lastsendlistok)
    }
    #[doc = "Aborted due to memory access error."]
    #[inline(always)]
    pub fn abortedmemerr(self) -> &'a mut crate::W<REG> {
        self.variant(Prevselect::Abortedmemerr)
    }
    #[doc = "Aborted by new send list."]
    #[inline(always)]
    pub fn abortednewsd(self) -> &'a mut crate::W<REG> {
        self.variant(Prevselect::Abortednewsd)
    }
    #[doc = "Aborted by direct user command."]
    #[inline(always)]
    pub fn abortedusercmd(self) -> &'a mut crate::W<REG> {
        self.variant(Prevselect::Abortedusercmd)
    }
    #[doc = "Aborted by timeout."]
    #[inline(always)]
    pub fn abortedtimeout(self) -> &'a mut crate::W<REG> {
        self.variant(Prevselect::Abortedtimeout)
    }
}
impl R {
    #[doc = "Bit 0 - Armed"]
    #[inline(always)]
    pub fn arm(&self) -> ArmR {
        ArmR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Active"]
    #[inline(always)]
    pub fn act(&self) -> ActR {
        ActR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pending"]
    #[inline(always)]
    pub fn pending(&self) -> PendingR {
        PendingR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Deactivating"]
    #[inline(always)]
    pub fn deact(&self) -> DeactR {
        DeactR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Previous"]
    #[inline(always)]
    pub fn prev(&self) -> PrevR {
        PrevR::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Armed"]
    #[inline(always)]
    pub fn arm(&mut self) -> ArmW<Pkttx1StatusSpec> {
        ArmW::new(self, 0)
    }
    #[doc = "Bit 1 - Active"]
    #[inline(always)]
    pub fn act(&mut self) -> ActW<Pkttx1StatusSpec> {
        ActW::new(self, 1)
    }
    #[doc = "Bit 2 - Pending"]
    #[inline(always)]
    pub fn pending(&mut self) -> PendingW<Pkttx1StatusSpec> {
        PendingW::new(self, 2)
    }
    #[doc = "Bit 3 - Deactivating"]
    #[inline(always)]
    pub fn deact(&mut self) -> DeactW<Pkttx1StatusSpec> {
        DeactW::new(self, 3)
    }
    #[doc = "Bits 16:18 - Previous"]
    #[inline(always)]
    pub fn prev(&mut self) -> PrevW<Pkttx1StatusSpec> {
        PrevW::new(self, 16)
    }
}
#[doc = "PktTx Status\n\nYou can [`read`](crate::Reg::read) this register and get [`pkttx1_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pkttx1_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pkttx1StatusSpec;
impl crate::RegisterSpec for Pkttx1StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pkttx1_status::R`](R) reader structure"]
impl crate::Readable for Pkttx1StatusSpec {}
#[doc = "`write(|w| ..)` method takes [`pkttx1_status::W`](W) writer structure"]
impl crate::Writable for Pkttx1StatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PKTTX1_STATUS to value 0"]
impl crate::Resettable for Pkttx1StatusSpec {}
