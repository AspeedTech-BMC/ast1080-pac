#[doc = "Register `PRIC_IO49C` reader"]
pub type R = crate::R<PricIo49cSpec>;
#[doc = "Register `PRIC_IO49C` writer"]
pub type W = crate::W<PricIo49cSpec>;
#[doc = "Field `RegionNReadMasters9` reader - Region #N Read Masters"]
pub type RegionNreadMasters9R = crate::FieldReader<u32>;
#[doc = "Field `RegionNReadMasters9` writer - Region #N Read Masters"]
pub type RegionNreadMasters9W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Region #N Read Masters"]
    #[inline(always)]
    pub fn region_nread_masters9(&self) -> RegionNreadMasters9R {
        RegionNreadMasters9R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Region #N Read Masters"]
    #[inline(always)]
    pub fn region_nread_masters9(&mut self) -> RegionNreadMasters9W<PricIo49cSpec> {
        RegionNreadMasters9W::new(self, 0)
    }
}
#[doc = "Memory Region Proection for AHB Masters Register \\#39\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io49c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io49c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo49cSpec;
impl crate::RegisterSpec for PricIo49cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io49c::R`](R) reader structure"]
impl crate::Readable for PricIo49cSpec {}
#[doc = "`write(|w| ..)` method takes [`pric_io49c::W`](W) writer structure"]
impl crate::Writable for PricIo49cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO49C to value 0"]
impl crate::Resettable for PricIo49cSpec {}
