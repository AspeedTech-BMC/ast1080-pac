#[doc = "Register `SCU8F0` reader"]
pub type R = crate::R<Scu8f0Spec>;
#[doc = "Register `SCU8F0` writer"]
pub type W = crate::W<Scu8f0Spec>;
#[doc = "Field `SCUSCRATCHMCU60` reader - SCU_SCRATCH_MCU_60"]
pub type Scuscratchmcu60R = crate::FieldReader<u32>;
#[doc = "Field `SCUSCRATCHMCU60` writer - SCU_SCRATCH_MCU_60"]
pub type Scuscratchmcu60W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_60"]
    #[inline(always)]
    pub fn scuscratchmcu60(&self) -> Scuscratchmcu60R {
        Scuscratchmcu60R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_60"]
    #[inline(always)]
    pub fn scuscratchmcu60(&mut self) -> Scuscratchmcu60W<Scu8f0Spec> {
        Scuscratchmcu60W::new(self, 0)
    }
}
#[doc = "Scratch register for MCU 60\n\nYou can [`read`](crate::Reg::read) this register and get [`scu8f0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu8f0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu8f0Spec;
impl crate::RegisterSpec for Scu8f0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu8f0::R`](R) reader structure"]
impl crate::Readable for Scu8f0Spec {}
#[doc = "`write(|w| ..)` method takes [`scu8f0::W`](W) writer structure"]
impl crate::Writable for Scu8f0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU8F0 to value 0"]
impl crate::Resettable for Scu8f0Spec {}
