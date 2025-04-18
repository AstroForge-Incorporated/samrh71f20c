#[doc = "Register `BCR` writer"]
pub type W = crate::W<BcrSpec>;
#[doc = "Field `SYNC` writer - Synchro Command"]
pub type SyncW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Synchro Command"]
    #[inline(always)]
    pub fn sync(&mut self) -> SyncW<BcrSpec> {
        SyncW::new(self, 0)
    }
}
#[doc = "Block Control Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BcrSpec;
impl crate::RegisterSpec for BcrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`bcr::W`](W) writer structure"]
impl crate::Writable for BcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BCR to value 0"]
impl crate::Resettable for BcrSpec {}
