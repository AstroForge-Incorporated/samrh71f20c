#[doc = "Register `MR` reader"]
pub type R = crate::R<MrSpec>;
#[doc = "Register `MR` writer"]
pub type W = crate::W<MrSpec>;
#[doc = "Start Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Smodselect {
    #[doc = "0: Manual mode"]
    ManualStart = 0,
    #[doc = "1: Auto mode"]
    AutoStart = 1,
    #[doc = "2: SHA_IDATAR0 access only mode (mandatory when DMA is used)"]
    Idatar0Start = 2,
}
impl From<Smodselect> for u8 {
    #[inline(always)]
    fn from(variant: Smodselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Smodselect {
    type Ux = u8;
}
impl crate::IsEnum for Smodselect {}
#[doc = "Field `SMOD` reader - Start Mode"]
pub type SmodR = crate::FieldReader<Smodselect>;
impl SmodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Smodselect> {
        match self.bits {
            0 => Some(Smodselect::ManualStart),
            1 => Some(Smodselect::AutoStart),
            2 => Some(Smodselect::Idatar0Start),
            _ => None,
        }
    }
    #[doc = "Manual mode"]
    #[inline(always)]
    pub fn is_manual_start(&self) -> bool {
        *self == Smodselect::ManualStart
    }
    #[doc = "Auto mode"]
    #[inline(always)]
    pub fn is_auto_start(&self) -> bool {
        *self == Smodselect::AutoStart
    }
    #[doc = "SHA_IDATAR0 access only mode (mandatory when DMA is used)"]
    #[inline(always)]
    pub fn is_idatar0_start(&self) -> bool {
        *self == Smodselect::Idatar0Start
    }
}
#[doc = "Field `SMOD` writer - Start Mode"]
pub type SmodW<'a, REG> = crate::FieldWriter<'a, REG, 2, Smodselect>;
impl<'a, REG> SmodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Manual mode"]
    #[inline(always)]
    pub fn manual_start(self) -> &'a mut crate::W<REG> {
        self.variant(Smodselect::ManualStart)
    }
    #[doc = "Auto mode"]
    #[inline(always)]
    pub fn auto_start(self) -> &'a mut crate::W<REG> {
        self.variant(Smodselect::AutoStart)
    }
    #[doc = "SHA_IDATAR0 access only mode (mandatory when DMA is used)"]
    #[inline(always)]
    pub fn idatar0_start(self) -> &'a mut crate::W<REG> {
        self.variant(Smodselect::Idatar0Start)
    }
}
#[doc = "Field `UIHV` reader - User Initial Hash Value Registers"]
pub type UihvR = crate::BitReader;
#[doc = "Field `UIHV` writer - User Initial Hash Value Registers"]
pub type UihvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UIEHV` reader - User Initial or Expected Hash Value Registers"]
pub type UiehvR = crate::BitReader;
#[doc = "Field `UIEHV` writer - User Initial or Expected Hash Value Registers"]
pub type UiehvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "SHA Algorithm\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Algoselect {
    #[doc = "0: SHA1 algorithm processed"]
    Sha1 = 0,
    #[doc = "1: SHA256 algorithm processed"]
    Sha256 = 1,
    #[doc = "2: SHA384 algorithm processed"]
    Sha384 = 2,
    #[doc = "3: SHA512 algorithm processed"]
    Sha512 = 3,
    #[doc = "4: SHA224 algorithm processed"]
    Sha224 = 4,
    #[doc = "8: HMAC algorithm with SHA1 Hash processed"]
    HmacSha1 = 8,
    #[doc = "9: HMAC algorithm with SHA256 Hash processed"]
    HmacSha256 = 9,
    #[doc = "10: HMAC algorithm with SHA384 Hash processed"]
    HmacSha384 = 10,
    #[doc = "11: HMAC algorithm with SHA512 Hash processed"]
    HmacSha512 = 11,
    #[doc = "12: HMAC algorithm with SHA224 Hash processed"]
    HmacSha224 = 12,
}
impl From<Algoselect> for u8 {
    #[inline(always)]
    fn from(variant: Algoselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Algoselect {
    type Ux = u8;
}
impl crate::IsEnum for Algoselect {}
#[doc = "Field `ALGO` reader - SHA Algorithm"]
pub type AlgoR = crate::FieldReader<Algoselect>;
impl AlgoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Algoselect> {
        match self.bits {
            0 => Some(Algoselect::Sha1),
            1 => Some(Algoselect::Sha256),
            2 => Some(Algoselect::Sha384),
            3 => Some(Algoselect::Sha512),
            4 => Some(Algoselect::Sha224),
            8 => Some(Algoselect::HmacSha1),
            9 => Some(Algoselect::HmacSha256),
            10 => Some(Algoselect::HmacSha384),
            11 => Some(Algoselect::HmacSha512),
            12 => Some(Algoselect::HmacSha224),
            _ => None,
        }
    }
    #[doc = "SHA1 algorithm processed"]
    #[inline(always)]
    pub fn is_sha1(&self) -> bool {
        *self == Algoselect::Sha1
    }
    #[doc = "SHA256 algorithm processed"]
    #[inline(always)]
    pub fn is_sha256(&self) -> bool {
        *self == Algoselect::Sha256
    }
    #[doc = "SHA384 algorithm processed"]
    #[inline(always)]
    pub fn is_sha384(&self) -> bool {
        *self == Algoselect::Sha384
    }
    #[doc = "SHA512 algorithm processed"]
    #[inline(always)]
    pub fn is_sha512(&self) -> bool {
        *self == Algoselect::Sha512
    }
    #[doc = "SHA224 algorithm processed"]
    #[inline(always)]
    pub fn is_sha224(&self) -> bool {
        *self == Algoselect::Sha224
    }
    #[doc = "HMAC algorithm with SHA1 Hash processed"]
    #[inline(always)]
    pub fn is_hmac_sha1(&self) -> bool {
        *self == Algoselect::HmacSha1
    }
    #[doc = "HMAC algorithm with SHA256 Hash processed"]
    #[inline(always)]
    pub fn is_hmac_sha256(&self) -> bool {
        *self == Algoselect::HmacSha256
    }
    #[doc = "HMAC algorithm with SHA384 Hash processed"]
    #[inline(always)]
    pub fn is_hmac_sha384(&self) -> bool {
        *self == Algoselect::HmacSha384
    }
    #[doc = "HMAC algorithm with SHA512 Hash processed"]
    #[inline(always)]
    pub fn is_hmac_sha512(&self) -> bool {
        *self == Algoselect::HmacSha512
    }
    #[doc = "HMAC algorithm with SHA224 Hash processed"]
    #[inline(always)]
    pub fn is_hmac_sha224(&self) -> bool {
        *self == Algoselect::HmacSha224
    }
}
#[doc = "Field `ALGO` writer - SHA Algorithm"]
pub type AlgoW<'a, REG> = crate::FieldWriter<'a, REG, 4, Algoselect>;
impl<'a, REG> AlgoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SHA1 algorithm processed"]
    #[inline(always)]
    pub fn sha1(self) -> &'a mut crate::W<REG> {
        self.variant(Algoselect::Sha1)
    }
    #[doc = "SHA256 algorithm processed"]
    #[inline(always)]
    pub fn sha256(self) -> &'a mut crate::W<REG> {
        self.variant(Algoselect::Sha256)
    }
    #[doc = "SHA384 algorithm processed"]
    #[inline(always)]
    pub fn sha384(self) -> &'a mut crate::W<REG> {
        self.variant(Algoselect::Sha384)
    }
    #[doc = "SHA512 algorithm processed"]
    #[inline(always)]
    pub fn sha512(self) -> &'a mut crate::W<REG> {
        self.variant(Algoselect::Sha512)
    }
    #[doc = "SHA224 algorithm processed"]
    #[inline(always)]
    pub fn sha224(self) -> &'a mut crate::W<REG> {
        self.variant(Algoselect::Sha224)
    }
    #[doc = "HMAC algorithm with SHA1 Hash processed"]
    #[inline(always)]
    pub fn hmac_sha1(self) -> &'a mut crate::W<REG> {
        self.variant(Algoselect::HmacSha1)
    }
    #[doc = "HMAC algorithm with SHA256 Hash processed"]
    #[inline(always)]
    pub fn hmac_sha256(self) -> &'a mut crate::W<REG> {
        self.variant(Algoselect::HmacSha256)
    }
    #[doc = "HMAC algorithm with SHA384 Hash processed"]
    #[inline(always)]
    pub fn hmac_sha384(self) -> &'a mut crate::W<REG> {
        self.variant(Algoselect::HmacSha384)
    }
    #[doc = "HMAC algorithm with SHA512 Hash processed"]
    #[inline(always)]
    pub fn hmac_sha512(self) -> &'a mut crate::W<REG> {
        self.variant(Algoselect::HmacSha512)
    }
    #[doc = "HMAC algorithm with SHA224 Hash processed"]
    #[inline(always)]
    pub fn hmac_sha224(self) -> &'a mut crate::W<REG> {
        self.variant(Algoselect::HmacSha224)
    }
}
#[doc = "Dual Input Buffer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dualbuffselect {
    #[doc = "0: SHA_IDATARx and SHA_IODATARx cannot be written during processing of previous block."]
    Inactive = 0,
    #[doc = "1: SHA_IDATARx and SHA_IODATARx can be written during processing of previous block when SMOD value = 2. It speeds up the overall runtime of large files."]
    Active = 1,
}
impl From<Dualbuffselect> for bool {
    #[inline(always)]
    fn from(variant: Dualbuffselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DUALBUFF` reader - Dual Input Buffer"]
pub type DualbuffR = crate::BitReader<Dualbuffselect>;
impl DualbuffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dualbuffselect {
        match self.bits {
            false => Dualbuffselect::Inactive,
            true => Dualbuffselect::Active,
        }
    }
    #[doc = "SHA_IDATARx and SHA_IODATARx cannot be written during processing of previous block."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Dualbuffselect::Inactive
    }
    #[doc = "SHA_IDATARx and SHA_IODATARx can be written during processing of previous block when SMOD value = 2. It speeds up the overall runtime of large files."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Dualbuffselect::Active
    }
}
#[doc = "Field `DUALBUFF` writer - Dual Input Buffer"]
pub type DualbuffW<'a, REG> = crate::BitWriter<'a, REG, Dualbuffselect>;
impl<'a, REG> DualbuffW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SHA_IDATARx and SHA_IODATARx cannot be written during processing of previous block."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(Dualbuffselect::Inactive)
    }
    #[doc = "SHA_IDATARx and SHA_IODATARx can be written during processing of previous block when SMOD value = 2. It speeds up the overall runtime of large files."]
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(Dualbuffselect::Active)
    }
}
#[doc = "Hash Check\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Checkselect {
    #[doc = "0: No check is performed"]
    NoCheck = 0,
    #[doc = "1: Check is performed with expected hash stored in internal expected hash value registers."]
    CheckEhv = 1,
    #[doc = "2: Check is performed with expected hash provided after the message."]
    CheckMessage = 2,
}
impl From<Checkselect> for u8 {
    #[inline(always)]
    fn from(variant: Checkselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Checkselect {
    type Ux = u8;
}
impl crate::IsEnum for Checkselect {}
#[doc = "Field `CHECK` reader - Hash Check"]
pub type CheckR = crate::FieldReader<Checkselect>;
impl CheckR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Checkselect> {
        match self.bits {
            0 => Some(Checkselect::NoCheck),
            1 => Some(Checkselect::CheckEhv),
            2 => Some(Checkselect::CheckMessage),
            _ => None,
        }
    }
    #[doc = "No check is performed"]
    #[inline(always)]
    pub fn is_no_check(&self) -> bool {
        *self == Checkselect::NoCheck
    }
    #[doc = "Check is performed with expected hash stored in internal expected hash value registers."]
    #[inline(always)]
    pub fn is_check_ehv(&self) -> bool {
        *self == Checkselect::CheckEhv
    }
    #[doc = "Check is performed with expected hash provided after the message."]
    #[inline(always)]
    pub fn is_check_message(&self) -> bool {
        *self == Checkselect::CheckMessage
    }
}
#[doc = "Field `CHECK` writer - Hash Check"]
pub type CheckW<'a, REG> = crate::FieldWriter<'a, REG, 2, Checkselect>;
impl<'a, REG> CheckW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No check is performed"]
    #[inline(always)]
    pub fn no_check(self) -> &'a mut crate::W<REG> {
        self.variant(Checkselect::NoCheck)
    }
    #[doc = "Check is performed with expected hash stored in internal expected hash value registers."]
    #[inline(always)]
    pub fn check_ehv(self) -> &'a mut crate::W<REG> {
        self.variant(Checkselect::CheckEhv)
    }
    #[doc = "Check is performed with expected hash provided after the message."]
    #[inline(always)]
    pub fn check_message(self) -> &'a mut crate::W<REG> {
        self.variant(Checkselect::CheckMessage)
    }
}
#[doc = "Field `CHKCNT` reader - Check Counter"]
pub type ChkcntR = crate::FieldReader;
#[doc = "Field `CHKCNT` writer - Check Counter"]
pub type ChkcntW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - Start Mode"]
    #[inline(always)]
    pub fn smod(&self) -> SmodR {
        SmodR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 5 - User Initial Hash Value Registers"]
    #[inline(always)]
    pub fn uihv(&self) -> UihvR {
        UihvR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - User Initial or Expected Hash Value Registers"]
    #[inline(always)]
    pub fn uiehv(&self) -> UiehvR {
        UiehvR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:11 - SHA Algorithm"]
    #[inline(always)]
    pub fn algo(&self) -> AlgoR {
        AlgoR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Dual Input Buffer"]
    #[inline(always)]
    pub fn dualbuff(&self) -> DualbuffR {
        DualbuffR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Hash Check"]
    #[inline(always)]
    pub fn check(&self) -> CheckR {
        CheckR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:31 - Check Counter"]
    #[inline(always)]
    pub fn chkcnt(&self) -> ChkcntR {
        ChkcntR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Start Mode"]
    #[inline(always)]
    pub fn smod(&mut self) -> SmodW<MrSpec> {
        SmodW::new(self, 0)
    }
    #[doc = "Bit 5 - User Initial Hash Value Registers"]
    #[inline(always)]
    pub fn uihv(&mut self) -> UihvW<MrSpec> {
        UihvW::new(self, 5)
    }
    #[doc = "Bit 6 - User Initial or Expected Hash Value Registers"]
    #[inline(always)]
    pub fn uiehv(&mut self) -> UiehvW<MrSpec> {
        UiehvW::new(self, 6)
    }
    #[doc = "Bits 8:11 - SHA Algorithm"]
    #[inline(always)]
    pub fn algo(&mut self) -> AlgoW<MrSpec> {
        AlgoW::new(self, 8)
    }
    #[doc = "Bit 16 - Dual Input Buffer"]
    #[inline(always)]
    pub fn dualbuff(&mut self) -> DualbuffW<MrSpec> {
        DualbuffW::new(self, 16)
    }
    #[doc = "Bits 24:25 - Hash Check"]
    #[inline(always)]
    pub fn check(&mut self) -> CheckW<MrSpec> {
        CheckW::new(self, 24)
    }
    #[doc = "Bits 28:31 - Check Counter"]
    #[inline(always)]
    pub fn chkcnt(&mut self) -> ChkcntW<MrSpec> {
        ChkcntW::new(self, 28)
    }
}
#[doc = "Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MrSpec;
impl crate::RegisterSpec for MrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mr::R`](R) reader structure"]
impl crate::Readable for MrSpec {}
#[doc = "`write(|w| ..)` method takes [`mr::W`](W) writer structure"]
impl crate::Writable for MrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MR to value 0"]
impl crate::Resettable for MrSpec {}
