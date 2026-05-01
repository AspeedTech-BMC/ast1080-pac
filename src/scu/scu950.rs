#[doc = "Register `SCU950` reader"]
pub type R = crate::R<Scu950Spec>;
#[doc = "Register `SCU950` writer"]
pub type W = crate::W<Scu950Spec>;
#[doc = "Field `SCUSSPDCACHEABLE` reader - SCU_SSP_D_CACHEABLE"]
pub type ScusspdcacheableR = crate::FieldReader<u32>;
#[doc = "Field `SCUSSPDCACHEABLE` writer - SCU_SSP_D_CACHEABLE"]
pub type ScusspdcacheableW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SSP_D_CACHEABLE"]
    #[inline(always)]
    pub fn scusspdcacheable(&self) -> ScusspdcacheableR {
        ScusspdcacheableR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_SSP_D_CACHEABLE"]
    #[inline(always)]
    pub fn scusspdcacheable(&mut self) -> ScusspdcacheableW<Scu950Spec> {
        ScusspdcacheableW::new(self, 0)
    }
}
#[doc = "\\SSP\\ Service Processor Control Register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`scu950::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu950::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu950Spec;
impl crate::RegisterSpec for Scu950Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu950::R`](R) reader structure"]
impl crate::Readable for Scu950Spec {}
#[doc = "`write(|w| ..)` method takes [`scu950::W`](W) writer structure"]
impl crate::Writable for Scu950Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU950 to value 0"]
impl crate::Resettable for Scu950Spec {}
