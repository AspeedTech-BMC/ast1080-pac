#[doc = "Register `SCU94C` reader"]
pub type R = crate::R<Scu94cSpec>;
#[doc = "Register `SCU94C` writer"]
pub type W = crate::W<Scu94cSpec>;
#[doc = "Field `SCUSSPICACHEABLE` reader - SCU_SSP_I_CACHEABLE"]
pub type ScusspicacheableR = crate::FieldReader<u32>;
#[doc = "Field `SCUSSPICACHEABLE` writer - SCU_SSP_I_CACHEABLE"]
pub type ScusspicacheableW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SSP_I_CACHEABLE"]
    #[inline(always)]
    pub fn scusspicacheable(&self) -> ScusspicacheableR {
        ScusspicacheableR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_SSP_I_CACHEABLE"]
    #[inline(always)]
    pub fn scusspicacheable(&mut self) -> ScusspicacheableW<Scu94cSpec> {
        ScusspicacheableW::new(self, 0)
    }
}
#[doc = "\\SSP\\ Service Processor Control Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`scu94c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu94c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu94cSpec;
impl crate::RegisterSpec for Scu94cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu94c::R`](R) reader structure"]
impl crate::Readable for Scu94cSpec {}
#[doc = "`write(|w| ..)` method takes [`scu94c::W`](W) writer structure"]
impl crate::Writable for Scu94cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU94C to value 0"]
impl crate::Resettable for Scu94cSpec {}
