#[doc = "Register `EPP08` reader"]
pub type R = crate::R<Epp08Spec>;
#[doc = "Register `EPP08` writer"]
pub type W = crate::W<Epp08Spec>;
#[doc = "Field `DMABaseAddr310` reader - DMA Base address\\[31:0\\]"]
pub type DmabaseAddr310R = crate::FieldReader<u32>;
#[doc = "Field `DMABaseAddr310` writer - DMA Base address\\[31:0\\]"]
pub type DmabaseAddr310W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DMA Base address\\[31:0\\]"]
    #[inline(always)]
    pub fn dmabase_addr310(&self) -> DmabaseAddr310R {
        DmabaseAddr310R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DMA Base address\\[31:0\\]"]
    #[inline(always)]
    pub fn dmabase_addr310(&mut self) -> DmabaseAddr310W<Epp08Spec> {
        DmabaseAddr310W::new(self, 0)
    }
}
#[doc = "DMA Descriptor/Buffer Base Address\n\nYou can [`read`](crate::Reg::read) this register and get [`epp08::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`epp08::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Epp08Spec;
impl crate::RegisterSpec for Epp08Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`epp08::R`](R) reader structure"]
impl crate::Readable for Epp08Spec {}
#[doc = "`write(|w| ..)` method takes [`epp08::W`](W) writer structure"]
impl crate::Writable for Epp08Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EPP08 to value 0"]
impl crate::Resettable for Epp08Spec {}
