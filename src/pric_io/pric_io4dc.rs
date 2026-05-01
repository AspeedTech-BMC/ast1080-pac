#[doc = "Register `PRIC_IO4DC` reader"]
pub type R = crate::R<PricIo4dcSpec>;
#[doc = "Register `PRIC_IO4DC` writer"]
pub type W = crate::W<PricIo4dcSpec>;
#[doc = "Field `RegionNReadMasters13` reader - Region #N Read Masters"]
pub type RegionNreadMasters13R = crate::FieldReader<u32>;
#[doc = "Field `RegionNReadMasters13` writer - Region #N Read Masters"]
pub type RegionNreadMasters13W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Region #N Read Masters"]
    #[inline(always)]
    pub fn region_nread_masters13(&self) -> RegionNreadMasters13R {
        RegionNreadMasters13R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Region #N Read Masters"]
    #[inline(always)]
    pub fn region_nread_masters13(&mut self) -> RegionNreadMasters13W<PricIo4dcSpec> {
        RegionNreadMasters13W::new(self, 0)
    }
}
#[doc = "Memory Region Proection for AHB Masters Register \\#55\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io4dc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io4dc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo4dcSpec;
impl crate::RegisterSpec for PricIo4dcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io4dc::R`](R) reader structure"]
impl crate::Readable for PricIo4dcSpec {}
#[doc = "`write(|w| ..)` method takes [`pric_io4dc::W`](W) writer structure"]
impl crate::Writable for PricIo4dcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO4DC to value 0"]
impl crate::Resettable for PricIo4dcSpec {}
