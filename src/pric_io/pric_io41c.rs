#[doc = "Register `PRIC_IO41C` reader"]
pub type R = crate::R<PricIo41cSpec>;
#[doc = "Register `PRIC_IO41C` writer"]
pub type W = crate::W<PricIo41cSpec>;
#[doc = "Field `RegionNReadMasters1` reader - Region #N Read Masters"]
pub type RegionNreadMasters1R = crate::FieldReader<u32>;
#[doc = "Field `RegionNReadMasters1` writer - Region #N Read Masters"]
pub type RegionNreadMasters1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Region #N Read Masters"]
    #[inline(always)]
    pub fn region_nread_masters1(&self) -> RegionNreadMasters1R {
        RegionNreadMasters1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Region #N Read Masters"]
    #[inline(always)]
    pub fn region_nread_masters1(&mut self) -> RegionNreadMasters1W<PricIo41cSpec> {
        RegionNreadMasters1W::new(self, 0)
    }
}
#[doc = "Memory Region Proection for AHB Masters Register \\#7\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io41c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io41c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo41cSpec;
impl crate::RegisterSpec for PricIo41cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io41c::R`](R) reader structure"]
impl crate::Readable for PricIo41cSpec {}
#[doc = "`write(|w| ..)` method takes [`pric_io41c::W`](W) writer structure"]
impl crate::Writable for PricIo41cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO41C to value 0"]
impl crate::Resettable for PricIo41cSpec {}
