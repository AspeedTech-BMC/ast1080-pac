#[doc = "Register `SCU8F4` reader"]
pub type R = crate::R<Scu8f4Spec>;
#[doc = "Register `SCU8F4` writer"]
pub type W = crate::W<Scu8f4Spec>;
#[doc = "Field `SCUSCRATCHMCU61` reader - SCU_SCRATCH_MCU_61"]
pub type Scuscratchmcu61R = crate::FieldReader<u32>;
#[doc = "Field `SCUSCRATCHMCU61` writer - SCU_SCRATCH_MCU_61"]
pub type Scuscratchmcu61W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_61"]
    #[inline(always)]
    pub fn scuscratchmcu61(&self) -> Scuscratchmcu61R {
        Scuscratchmcu61R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_61"]
    #[inline(always)]
    pub fn scuscratchmcu61(&mut self) -> Scuscratchmcu61W<Scu8f4Spec> {
        Scuscratchmcu61W::new(self, 0)
    }
}
#[doc = "Scratch register for MCU 61\n\nYou can [`read`](crate::Reg::read) this register and get [`scu8f4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu8f4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu8f4Spec;
impl crate::RegisterSpec for Scu8f4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu8f4::R`](R) reader structure"]
impl crate::Readable for Scu8f4Spec {}
#[doc = "`write(|w| ..)` method takes [`scu8f4::W`](W) writer structure"]
impl crate::Writable for Scu8f4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU8F4 to value 0"]
impl crate::Resettable for Scu8f4Spec {}
