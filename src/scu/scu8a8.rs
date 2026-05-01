#[doc = "Register `SCU8A8` reader"]
pub type R = crate::R<Scu8a8Spec>;
#[doc = "Register `SCU8A8` writer"]
pub type W = crate::W<Scu8a8Spec>;
#[doc = "Field `SCUSCRATCHMCU42` reader - SCU_SCRATCH_MCU_42"]
pub type Scuscratchmcu42R = crate::FieldReader<u32>;
#[doc = "Field `SCUSCRATCHMCU42` writer - SCU_SCRATCH_MCU_42"]
pub type Scuscratchmcu42W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_42"]
    #[inline(always)]
    pub fn scuscratchmcu42(&self) -> Scuscratchmcu42R {
        Scuscratchmcu42R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_42"]
    #[inline(always)]
    pub fn scuscratchmcu42(&mut self) -> Scuscratchmcu42W<Scu8a8Spec> {
        Scuscratchmcu42W::new(self, 0)
    }
}
#[doc = "Scratch register for MCU 42\n\nYou can [`read`](crate::Reg::read) this register and get [`scu8a8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu8a8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu8a8Spec;
impl crate::RegisterSpec for Scu8a8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu8a8::R`](R) reader structure"]
impl crate::Readable for Scu8a8Spec {}
#[doc = "`write(|w| ..)` method takes [`scu8a8::W`](W) writer structure"]
impl crate::Writable for Scu8a8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU8A8 to value 0"]
impl crate::Resettable for Scu8a8Spec {}
