#[doc = "Register `SCU894` reader"]
pub type R = crate::R<Scu894Spec>;
#[doc = "Register `SCU894` writer"]
pub type W = crate::W<Scu894Spec>;
#[doc = "Field `SCUSCRATCHMCU37` reader - SCU_SCRATCH_MCU_37"]
pub type Scuscratchmcu37R = crate::FieldReader<u32>;
#[doc = "Field `SCUSCRATCHMCU37` writer - SCU_SCRATCH_MCU_37"]
pub type Scuscratchmcu37W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_37"]
    #[inline(always)]
    pub fn scuscratchmcu37(&self) -> Scuscratchmcu37R {
        Scuscratchmcu37R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_37"]
    #[inline(always)]
    pub fn scuscratchmcu37(&mut self) -> Scuscratchmcu37W<Scu894Spec> {
        Scuscratchmcu37W::new(self, 0)
    }
}
#[doc = "Scratch register for MCU 37\n\nYou can [`read`](crate::Reg::read) this register and get [`scu894::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu894::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu894Spec;
impl crate::RegisterSpec for Scu894Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu894::R`](R) reader structure"]
impl crate::Readable for Scu894Spec {}
#[doc = "`write(|w| ..)` method takes [`scu894::W`](W) writer structure"]
impl crate::Writable for Scu894Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU894 to value 0"]
impl crate::Resettable for Scu894Spec {}
