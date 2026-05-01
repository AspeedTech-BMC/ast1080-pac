#[doc = "Register `SCU868` reader"]
pub type R = crate::R<Scu868Spec>;
#[doc = "Register `SCU868` writer"]
pub type W = crate::W<Scu868Spec>;
#[doc = "Field `SCUSCRATCHMCU26` reader - SCU_SCRATCH_MCU_26"]
pub type Scuscratchmcu26R = crate::FieldReader<u32>;
#[doc = "Field `SCUSCRATCHMCU26` writer - SCU_SCRATCH_MCU_26"]
pub type Scuscratchmcu26W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_26"]
    #[inline(always)]
    pub fn scuscratchmcu26(&self) -> Scuscratchmcu26R {
        Scuscratchmcu26R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_26"]
    #[inline(always)]
    pub fn scuscratchmcu26(&mut self) -> Scuscratchmcu26W<Scu868Spec> {
        Scuscratchmcu26W::new(self, 0)
    }
}
#[doc = "Scratch register for MCU 26\n\nYou can [`read`](crate::Reg::read) this register and get [`scu868::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu868::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu868Spec;
impl crate::RegisterSpec for Scu868Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu868::R`](R) reader structure"]
impl crate::Readable for Scu868Spec {}
#[doc = "`write(|w| ..)` method takes [`scu868::W`](W) writer structure"]
impl crate::Writable for Scu868Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU868 to value 0"]
impl crate::Resettable for Scu868Spec {}
