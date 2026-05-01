#[doc = "Register `SCU908` reader"]
pub type R = crate::R<Scu908Spec>;
#[doc = "Register `SCU908` writer"]
pub type W = crate::W<Scu908Spec>;
#[doc = "Field `SCUPSPREMAPMEMBASE` reader - SCU_PSP_REMAP_MEM_BASE"]
pub type ScupspremapmembaseR = crate::FieldReader<u32>;
#[doc = "Field `SCUPSPREMAPMEMBASE` writer - SCU_PSP_REMAP_MEM_BASE"]
pub type ScupspremapmembaseW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_PSP_REMAP_MEM_BASE"]
    #[inline(always)]
    pub fn scupspremapmembase(&self) -> ScupspremapmembaseR {
        ScupspremapmembaseR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_PSP_REMAP_MEM_BASE"]
    #[inline(always)]
    pub fn scupspremapmembase(&mut self) -> ScupspremapmembaseW<Scu908Spec> {
        ScupspremapmembaseW::new(self, 0)
    }
}
#[doc = "\\PSP\\ Service Processor Control Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`scu908::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu908::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu908Spec;
impl crate::RegisterSpec for Scu908Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu908::R`](R) reader structure"]
impl crate::Readable for Scu908Spec {}
#[doc = "`write(|w| ..)` method takes [`scu908::W`](W) writer structure"]
impl crate::Writable for Scu908Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU908 to value 0"]
impl crate::Resettable for Scu908Spec {}
