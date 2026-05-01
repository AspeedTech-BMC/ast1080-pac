#[doc = "Register `SCU848` reader"]
pub type R = crate::R<Scu848Spec>;
#[doc = "Register `SCU848` writer"]
pub type W = crate::W<Scu848Spec>;
#[doc = "Field `SCUSCRATCHMCU18` reader - SCU_SCRATCH_MCU_18"]
pub type Scuscratchmcu18R = crate::FieldReader<u32>;
#[doc = "Field `SCUSCRATCHMCU18` writer - SCU_SCRATCH_MCU_18"]
pub type Scuscratchmcu18W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_18"]
    #[inline(always)]
    pub fn scuscratchmcu18(&self) -> Scuscratchmcu18R {
        Scuscratchmcu18R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_18"]
    #[inline(always)]
    pub fn scuscratchmcu18(&mut self) -> Scuscratchmcu18W<Scu848Spec> {
        Scuscratchmcu18W::new(self, 0)
    }
}
#[doc = "Scratch register for MCU 18\n\nYou can [`read`](crate::Reg::read) this register and get [`scu848::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu848::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu848Spec;
impl crate::RegisterSpec for Scu848Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu848::R`](R) reader structure"]
impl crate::Readable for Scu848Spec {}
#[doc = "`write(|w| ..)` method takes [`scu848::W`](W) writer structure"]
impl crate::Writable for Scu848Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU848 to value 0"]
impl crate::Resettable for Scu848Spec {}
