#[doc = "Register `SCU838` reader"]
pub type R = crate::R<Scu838Spec>;
#[doc = "Register `SCU838` writer"]
pub type W = crate::W<Scu838Spec>;
#[doc = "Field `SCUSCRATCHMCU14` reader - SCU_SCRATCH_MCU_14"]
pub type Scuscratchmcu14R = crate::FieldReader<u32>;
#[doc = "Field `SCUSCRATCHMCU14` writer - SCU_SCRATCH_MCU_14"]
pub type Scuscratchmcu14W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_14"]
    #[inline(always)]
    pub fn scuscratchmcu14(&self) -> Scuscratchmcu14R {
        Scuscratchmcu14R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_14"]
    #[inline(always)]
    pub fn scuscratchmcu14(&mut self) -> Scuscratchmcu14W<Scu838Spec> {
        Scuscratchmcu14W::new(self, 0)
    }
}
#[doc = "Scratch register for MCU 14\n\nYou can [`read`](crate::Reg::read) this register and get [`scu838::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu838::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu838Spec;
impl crate::RegisterSpec for Scu838Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu838::R`](R) reader structure"]
impl crate::Readable for Scu838Spec {}
#[doc = "`write(|w| ..)` method takes [`scu838::W`](W) writer structure"]
impl crate::Writable for Scu838Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU838 to value 0"]
impl crate::Resettable for Scu838Spec {}
