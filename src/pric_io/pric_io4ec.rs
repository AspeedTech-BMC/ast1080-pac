#[doc = "Register `PRIC_IO4EC` reader"]
pub type R = crate::R<PricIo4ecSpec>;
#[doc = "Register `PRIC_IO4EC` writer"]
pub type W = crate::W<PricIo4ecSpec>;
#[doc = "Field `RegionNReadMasters14` reader - Region #N Read Masters"]
pub type RegionNreadMasters14R = crate::FieldReader<u32>;
#[doc = "Field `RegionNReadMasters14` writer - Region #N Read Masters"]
pub type RegionNreadMasters14W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Region #N Read Masters"]
    #[inline(always)]
    pub fn region_nread_masters14(&self) -> RegionNreadMasters14R {
        RegionNreadMasters14R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Region #N Read Masters"]
    #[inline(always)]
    pub fn region_nread_masters14(&mut self) -> RegionNreadMasters14W<PricIo4ecSpec> {
        RegionNreadMasters14W::new(self, 0)
    }
}
#[doc = "Memory Region Proection for AHB Masters Register \\#59\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io4ec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io4ec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo4ecSpec;
impl crate::RegisterSpec for PricIo4ecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io4ec::R`](R) reader structure"]
impl crate::Readable for PricIo4ecSpec {}
#[doc = "`write(|w| ..)` method takes [`pric_io4ec::W`](W) writer structure"]
impl crate::Writable for PricIo4ecSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO4EC to value 0"]
impl crate::Resettable for PricIo4ecSpec {}
