#[doc = "Register `UR` reader"]
pub type R = crate::R<UrSpec>;
#[doc = "Register `UR` writer"]
pub type W = crate::W<UrSpec>;
#[doc = "Field `RMII` reader - Reduced MII Mode"]
pub type RmiiR = crate::BitReader;
#[doc = "Field `RMII` writer - Reduced MII Mode"]
pub type RmiiW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Reduced MII Mode"]
    #[inline(always)]
    pub fn rmii(&self) -> RmiiR {
        RmiiR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reduced MII Mode"]
    #[inline(always)]
    pub fn rmii(&mut self) -> RmiiW<UrSpec> {
        RmiiW::new(self, 0)
    }
}
#[doc = "User Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ur::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ur::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UrSpec;
impl crate::RegisterSpec for UrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ur::R`](R) reader structure"]
impl crate::Readable for UrSpec {}
#[doc = "`write(|w| ..)` method takes [`ur::W`](W) writer structure"]
impl crate::Writable for UrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UR to value 0"]
impl crate::Resettable for UrSpec {}
