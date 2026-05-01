#[doc = "Register `SCU904` reader"]
pub type R = crate::R<Scu904Spec>;
#[doc = "Register `SCU904` writer"]
pub type W = crate::W<Scu904Spec>;
#[doc = "Field `SCUPSPREMAPAHBBASE` reader - SCU_PSP_REMAP_AHB_BASE"]
pub type ScupspremapahbbaseR = crate::FieldReader<u32>;
#[doc = "Field `SCUPSPREMAPAHBBASE` writer - SCU_PSP_REMAP_AHB_BASE"]
pub type ScupspremapahbbaseW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_PSP_REMAP_AHB_BASE"]
    #[inline(always)]
    pub fn scupspremapahbbase(&self) -> ScupspremapahbbaseR {
        ScupspremapahbbaseR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_PSP_REMAP_AHB_BASE"]
    #[inline(always)]
    pub fn scupspremapahbbase(&mut self) -> ScupspremapahbbaseW<Scu904Spec> {
        ScupspremapahbbaseW::new(self, 0)
    }
}
#[doc = "\\PSP\\ Service Processor Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`scu904::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu904::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu904Spec;
impl crate::RegisterSpec for Scu904Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu904::R`](R) reader structure"]
impl crate::Readable for Scu904Spec {}
#[doc = "`write(|w| ..)` method takes [`scu904::W`](W) writer structure"]
impl crate::Writable for Scu904Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU904 to value 0"]
impl crate::Resettable for Scu904Spec {}
