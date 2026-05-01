#[doc = "Register `SCU948` reader"]
pub type R = crate::R<Scu948Spec>;
#[doc = "Register `SCU948` writer"]
pub type W = crate::W<Scu948Spec>;
#[doc = "Field `SCUSSPREMAPMEMBASE` reader - SCU_SSP_REMAP_MEM_BASE"]
pub type ScusspremapmembaseR = crate::FieldReader<u32>;
#[doc = "Field `SCUSSPREMAPMEMBASE` writer - SCU_SSP_REMAP_MEM_BASE"]
pub type ScusspremapmembaseW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SSP_REMAP_MEM_BASE"]
    #[inline(always)]
    pub fn scusspremapmembase(&self) -> ScusspremapmembaseR {
        ScusspremapmembaseR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_SSP_REMAP_MEM_BASE"]
    #[inline(always)]
    pub fn scusspremapmembase(&mut self) -> ScusspremapmembaseW<Scu948Spec> {
        ScusspremapmembaseW::new(self, 0)
    }
}
#[doc = "\\SSP\\ Service Processor Control Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`scu948::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu948::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu948Spec;
impl crate::RegisterSpec for Scu948Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu948::R`](R) reader structure"]
impl crate::Readable for Scu948Spec {}
#[doc = "`write(|w| ..)` method takes [`scu948::W`](W) writer structure"]
impl crate::Writable for Scu948Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU948 to value 0x1000_0000"]
impl crate::Resettable for Scu948Spec {
    const RESET_VALUE: u32 = 0x1000_0000;
}
