#[doc = "Register `SCU8A0` reader"]
pub type R = crate::R<Scu8a0Spec>;
#[doc = "Register `SCU8A0` writer"]
pub type W = crate::W<Scu8a0Spec>;
#[doc = "Field `SCUSCRATCHMCU40` reader - SCU_SCRATCH_MCU_40"]
pub type Scuscratchmcu40R = crate::FieldReader<u32>;
#[doc = "Field `SCUSCRATCHMCU40` writer - SCU_SCRATCH_MCU_40"]
pub type Scuscratchmcu40W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_40"]
    #[inline(always)]
    pub fn scuscratchmcu40(&self) -> Scuscratchmcu40R {
        Scuscratchmcu40R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_40"]
    #[inline(always)]
    pub fn scuscratchmcu40(&mut self) -> Scuscratchmcu40W<Scu8a0Spec> {
        Scuscratchmcu40W::new(self, 0)
    }
}
#[doc = "Scratch register for MCU 40\n\nYou can [`read`](crate::Reg::read) this register and get [`scu8a0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu8a0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu8a0Spec;
impl crate::RegisterSpec for Scu8a0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu8a0::R`](R) reader structure"]
impl crate::Readable for Scu8a0Spec {}
#[doc = "`write(|w| ..)` method takes [`scu8a0::W`](W) writer structure"]
impl crate::Writable for Scu8a0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU8A0 to value 0"]
impl crate::Resettable for Scu8a0Spec {}
