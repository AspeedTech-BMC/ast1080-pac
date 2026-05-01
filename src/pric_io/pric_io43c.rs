#[doc = "Register `PRIC_IO43C` reader"]
pub type R = crate::R<PricIo43cSpec>;
#[doc = "Register `PRIC_IO43C` writer"]
pub type W = crate::W<PricIo43cSpec>;
#[doc = "Field `RegionNReadMasters3` reader - Region #N Read Masters"]
pub type RegionNreadMasters3R = crate::FieldReader<u32>;
#[doc = "Field `RegionNReadMasters3` writer - Region #N Read Masters"]
pub type RegionNreadMasters3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Region #N Read Masters"]
    #[inline(always)]
    pub fn region_nread_masters3(&self) -> RegionNreadMasters3R {
        RegionNreadMasters3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Region #N Read Masters"]
    #[inline(always)]
    pub fn region_nread_masters3(&mut self) -> RegionNreadMasters3W<PricIo43cSpec> {
        RegionNreadMasters3W::new(self, 0)
    }
}
#[doc = "Memory Region Proection for AHB Masters Register \\#15\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io43c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io43c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo43cSpec;
impl crate::RegisterSpec for PricIo43cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io43c::R`](R) reader structure"]
impl crate::Readable for PricIo43cSpec {}
#[doc = "`write(|w| ..)` method takes [`pric_io43c::W`](W) writer structure"]
impl crate::Writable for PricIo43cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO43C to value 0"]
impl crate::Resettable for PricIo43cSpec {}
