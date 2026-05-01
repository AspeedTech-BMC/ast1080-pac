#[doc = "Register `SCU934` reader"]
pub type R = crate::R<Scu934Spec>;
#[doc = "Register `SCU934` writer"]
pub type W = crate::W<Scu934Spec>;
#[doc = "Field `SCUPSPMAPMEMSIZE` reader - SCU_PSP_MAP_MEM_SIZE"]
pub type ScupspmapmemsizeR = crate::FieldReader<u32>;
#[doc = "Field `SCUPSPMAPMEMSIZE` writer - SCU_PSP_MAP_MEM_SIZE"]
pub type ScupspmapmemsizeW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:29 - SCU_PSP_MAP_MEM_SIZE"]
    #[inline(always)]
    pub fn scupspmapmemsize(&self) -> ScupspmapmemsizeR {
        ScupspmapmemsizeR::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29 - SCU_PSP_MAP_MEM_SIZE"]
    #[inline(always)]
    pub fn scupspmapmemsize(&mut self) -> ScupspmapmemsizeW<Scu934Spec> {
        ScupspmapmemsizeW::new(self, 0)
    }
}
#[doc = "\\PSP\\ Service Processor REMAP Size Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`scu934::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu934::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu934Spec;
impl crate::RegisterSpec for Scu934Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu934::R`](R) reader structure"]
impl crate::Readable for Scu934Spec {}
#[doc = "`write(|w| ..)` method takes [`scu934::W`](W) writer structure"]
impl crate::Writable for Scu934Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU934 to value 0"]
impl crate::Resettable for Scu934Spec {}
