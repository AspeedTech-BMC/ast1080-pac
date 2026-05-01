#[doc = "Register `PRIC_IO48C` reader"]
pub type R = crate::R<PricIo48cSpec>;
#[doc = "Register `PRIC_IO48C` writer"]
pub type W = crate::W<PricIo48cSpec>;
#[doc = "Field `RegionNReadMasters8` reader - Region #N Read Masters"]
pub type RegionNreadMasters8R = crate::FieldReader<u32>;
#[doc = "Field `RegionNReadMasters8` writer - Region #N Read Masters"]
pub type RegionNreadMasters8W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Region #N Read Masters"]
    #[inline(always)]
    pub fn region_nread_masters8(&self) -> RegionNreadMasters8R {
        RegionNreadMasters8R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Region #N Read Masters"]
    #[inline(always)]
    pub fn region_nread_masters8(&mut self) -> RegionNreadMasters8W<PricIo48cSpec> {
        RegionNreadMasters8W::new(self, 0)
    }
}
#[doc = "Memory Region Proection for AHB Masters Register \\#35\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io48c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io48c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo48cSpec;
impl crate::RegisterSpec for PricIo48cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io48c::R`](R) reader structure"]
impl crate::Readable for PricIo48cSpec {}
#[doc = "`write(|w| ..)` method takes [`pric_io48c::W`](W) writer structure"]
impl crate::Writable for PricIo48cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO48C to value 0"]
impl crate::Resettable for PricIo48cSpec {}
