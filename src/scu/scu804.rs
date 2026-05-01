#[doc = "Register `SCU804` reader"]
pub type R = crate::R<Scu804Spec>;
#[doc = "Register `SCU804` writer"]
pub type W = crate::W<Scu804Spec>;
#[doc = "Field `SCUSCRATCHMCU1` reader - SCU_SCRATCH_MCU_1"]
pub type Scuscratchmcu1R = crate::FieldReader<u32>;
#[doc = "Field `SCUSCRATCHMCU1` writer - SCU_SCRATCH_MCU_1"]
pub type Scuscratchmcu1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_1"]
    #[inline(always)]
    pub fn scuscratchmcu1(&self) -> Scuscratchmcu1R {
        Scuscratchmcu1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_1"]
    #[inline(always)]
    pub fn scuscratchmcu1(&mut self) -> Scuscratchmcu1W<Scu804Spec> {
        Scuscratchmcu1W::new(self, 0)
    }
}
#[doc = "Scratch register for MCU 1\n\nYou can [`read`](crate::Reg::read) this register and get [`scu804::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu804::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu804Spec;
impl crate::RegisterSpec for Scu804Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu804::R`](R) reader structure"]
impl crate::Readable for Scu804Spec {}
#[doc = "`write(|w| ..)` method takes [`scu804::W`](W) writer structure"]
impl crate::Writable for Scu804Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU804 to value 0"]
impl crate::Resettable for Scu804Spec {}
