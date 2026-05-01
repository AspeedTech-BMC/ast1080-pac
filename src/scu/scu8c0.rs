#[doc = "Register `SCU8C0` reader"]
pub type R = crate::R<Scu8c0Spec>;
#[doc = "Register `SCU8C0` writer"]
pub type W = crate::W<Scu8c0Spec>;
#[doc = "Field `SCUSCRATCHMCU48` reader - SCU_SCRATCH_MCU_48"]
pub type Scuscratchmcu48R = crate::FieldReader<u32>;
#[doc = "Field `SCUSCRATCHMCU48` writer - SCU_SCRATCH_MCU_48"]
pub type Scuscratchmcu48W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_48"]
    #[inline(always)]
    pub fn scuscratchmcu48(&self) -> Scuscratchmcu48R {
        Scuscratchmcu48R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_48"]
    #[inline(always)]
    pub fn scuscratchmcu48(&mut self) -> Scuscratchmcu48W<Scu8c0Spec> {
        Scuscratchmcu48W::new(self, 0)
    }
}
#[doc = "Scratch register for MCU 48\n\nYou can [`read`](crate::Reg::read) this register and get [`scu8c0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu8c0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu8c0Spec;
impl crate::RegisterSpec for Scu8c0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu8c0::R`](R) reader structure"]
impl crate::Readable for Scu8c0Spec {}
#[doc = "`write(|w| ..)` method takes [`scu8c0::W`](W) writer structure"]
impl crate::Writable for Scu8c0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU8C0 to value 0"]
impl crate::Resettable for Scu8c0Spec {}
