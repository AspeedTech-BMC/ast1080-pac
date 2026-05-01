#[doc = "Register `SCU870` reader"]
pub type R = crate::R<Scu870Spec>;
#[doc = "Register `SCU870` writer"]
pub type W = crate::W<Scu870Spec>;
#[doc = "Field `SCUSCRATCHMCU28` reader - SCU_SCRATCH_MCU_28"]
pub type Scuscratchmcu28R = crate::FieldReader<u32>;
#[doc = "Field `SCUSCRATCHMCU28` writer - SCU_SCRATCH_MCU_28"]
pub type Scuscratchmcu28W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_28"]
    #[inline(always)]
    pub fn scuscratchmcu28(&self) -> Scuscratchmcu28R {
        Scuscratchmcu28R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_28"]
    #[inline(always)]
    pub fn scuscratchmcu28(&mut self) -> Scuscratchmcu28W<Scu870Spec> {
        Scuscratchmcu28W::new(self, 0)
    }
}
#[doc = "Scratch register for MCU 28\n\nYou can [`read`](crate::Reg::read) this register and get [`scu870::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu870::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu870Spec;
impl crate::RegisterSpec for Scu870Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu870::R`](R) reader structure"]
impl crate::Readable for Scu870Spec {}
#[doc = "`write(|w| ..)` method takes [`scu870::W`](W) writer structure"]
impl crate::Writable for Scu870Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU870 to value 0"]
impl crate::Resettable for Scu870Spec {}
