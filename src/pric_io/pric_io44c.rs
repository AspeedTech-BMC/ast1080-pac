#[doc = "Register `PRIC_IO44C` reader"]
pub type R = crate::R<PricIo44cSpec>;
#[doc = "Register `PRIC_IO44C` writer"]
pub type W = crate::W<PricIo44cSpec>;
#[doc = "Field `RegionNReadMasters4` reader - Region #N Read Masters"]
pub type RegionNreadMasters4R = crate::FieldReader<u32>;
#[doc = "Field `RegionNReadMasters4` writer - Region #N Read Masters"]
pub type RegionNreadMasters4W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Region #N Read Masters"]
    #[inline(always)]
    pub fn region_nread_masters4(&self) -> RegionNreadMasters4R {
        RegionNreadMasters4R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Region #N Read Masters"]
    #[inline(always)]
    pub fn region_nread_masters4(&mut self) -> RegionNreadMasters4W<PricIo44cSpec> {
        RegionNreadMasters4W::new(self, 0)
    }
}
#[doc = "Memory Region Proection for AHB Masters Register \\#19\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io44c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io44c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo44cSpec;
impl crate::RegisterSpec for PricIo44cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io44c::R`](R) reader structure"]
impl crate::Readable for PricIo44cSpec {}
#[doc = "`write(|w| ..)` method takes [`pric_io44c::W`](W) writer structure"]
impl crate::Writable for PricIo44cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO44C to value 0"]
impl crate::Resettable for PricIo44cSpec {}
