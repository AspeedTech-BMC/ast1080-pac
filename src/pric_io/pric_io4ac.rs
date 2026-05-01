#[doc = "Register `PRIC_IO4AC` reader"]
pub type R = crate::R<PricIo4acSpec>;
#[doc = "Register `PRIC_IO4AC` writer"]
pub type W = crate::W<PricIo4acSpec>;
#[doc = "Field `RegionNReadMasters10` reader - Region #N Read Masters"]
pub type RegionNreadMasters10R = crate::FieldReader<u32>;
#[doc = "Field `RegionNReadMasters10` writer - Region #N Read Masters"]
pub type RegionNreadMasters10W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Region #N Read Masters"]
    #[inline(always)]
    pub fn region_nread_masters10(&self) -> RegionNreadMasters10R {
        RegionNreadMasters10R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Region #N Read Masters"]
    #[inline(always)]
    pub fn region_nread_masters10(&mut self) -> RegionNreadMasters10W<PricIo4acSpec> {
        RegionNreadMasters10W::new(self, 0)
    }
}
#[doc = "Memory Region Proection for AHB Masters Register \\#43\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io4ac::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io4ac::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo4acSpec;
impl crate::RegisterSpec for PricIo4acSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io4ac::R`](R) reader structure"]
impl crate::Readable for PricIo4acSpec {}
#[doc = "`write(|w| ..)` method takes [`pric_io4ac::W`](W) writer structure"]
impl crate::Writable for PricIo4acSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO4AC to value 0"]
impl crate::Resettable for PricIo4acSpec {}
