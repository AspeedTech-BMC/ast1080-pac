#[doc = "Register `PRIC_IO4BC` reader"]
pub type R = crate::R<PricIo4bcSpec>;
#[doc = "Register `PRIC_IO4BC` writer"]
pub type W = crate::W<PricIo4bcSpec>;
#[doc = "Field `RegionNReadMasters11` reader - Region #N Read Masters"]
pub type RegionNreadMasters11R = crate::FieldReader<u32>;
#[doc = "Field `RegionNReadMasters11` writer - Region #N Read Masters"]
pub type RegionNreadMasters11W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Region #N Read Masters"]
    #[inline(always)]
    pub fn region_nread_masters11(&self) -> RegionNreadMasters11R {
        RegionNreadMasters11R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Region #N Read Masters"]
    #[inline(always)]
    pub fn region_nread_masters11(&mut self) -> RegionNreadMasters11W<PricIo4bcSpec> {
        RegionNreadMasters11W::new(self, 0)
    }
}
#[doc = "Memory Region Proection for AHB Masters Register \\#47\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io4bc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io4bc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo4bcSpec;
impl crate::RegisterSpec for PricIo4bcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io4bc::R`](R) reader structure"]
impl crate::Readable for PricIo4bcSpec {}
#[doc = "`write(|w| ..)` method takes [`pric_io4bc::W`](W) writer structure"]
impl crate::Writable for PricIo4bcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO4BC to value 0"]
impl crate::Resettable for PricIo4bcSpec {}
