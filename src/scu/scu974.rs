#[doc = "Register `SCU974` reader"]
pub type R = crate::R<Scu974Spec>;
#[doc = "Register `SCU974` writer"]
pub type W = crate::W<Scu974Spec>;
#[doc = "Field `SCUSSPMAPMEMSIZE` reader - SCU_SSP_MAP_MEM_SIZE"]
pub type ScusspmapmemsizeR = crate::FieldReader<u32>;
#[doc = "Field `SCUSSPMAPMEMSIZE` writer - SCU_SSP_MAP_MEM_SIZE"]
pub type ScusspmapmemsizeW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:29 - SCU_SSP_MAP_MEM_SIZE"]
    #[inline(always)]
    pub fn scusspmapmemsize(&self) -> ScusspmapmemsizeR {
        ScusspmapmemsizeR::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29 - SCU_SSP_MAP_MEM_SIZE"]
    #[inline(always)]
    pub fn scusspmapmemsize(&mut self) -> ScusspmapmemsizeW<Scu974Spec> {
        ScusspmapmemsizeW::new(self, 0)
    }
}
#[doc = "\\SSP\\ Service Processor REMAP Size Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`scu974::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu974::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu974Spec;
impl crate::RegisterSpec for Scu974Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu974::R`](R) reader structure"]
impl crate::Readable for Scu974Spec {}
#[doc = "`write(|w| ..)` method takes [`scu974::W`](W) writer structure"]
impl crate::Writable for Scu974Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU974 to value 0x0010_0000"]
impl crate::Resettable for Scu974Spec {
    const RESET_VALUE: u32 = 0x0010_0000;
}
