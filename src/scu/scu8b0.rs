#[doc = "Register `SCU8B0` reader"]
pub type R = crate::R<Scu8b0Spec>;
#[doc = "Register `SCU8B0` writer"]
pub type W = crate::W<Scu8b0Spec>;
#[doc = "Field `SCUSCRATCHMCU44` reader - SCU_SCRATCH_MCU_44"]
pub type Scuscratchmcu44R = crate::FieldReader<u32>;
#[doc = "Field `SCUSCRATCHMCU44` writer - SCU_SCRATCH_MCU_44"]
pub type Scuscratchmcu44W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_44"]
    #[inline(always)]
    pub fn scuscratchmcu44(&self) -> Scuscratchmcu44R {
        Scuscratchmcu44R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_44"]
    #[inline(always)]
    pub fn scuscratchmcu44(&mut self) -> Scuscratchmcu44W<Scu8b0Spec> {
        Scuscratchmcu44W::new(self, 0)
    }
}
#[doc = "Scratch register for MCU 44\n\nYou can [`read`](crate::Reg::read) this register and get [`scu8b0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu8b0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu8b0Spec;
impl crate::RegisterSpec for Scu8b0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu8b0::R`](R) reader structure"]
impl crate::Readable for Scu8b0Spec {}
#[doc = "`write(|w| ..)` method takes [`scu8b0::W`](W) writer structure"]
impl crate::Writable for Scu8b0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU8B0 to value 0"]
impl crate::Resettable for Scu8b0Spec {}
