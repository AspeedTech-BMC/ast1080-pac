#[doc = "Register `SCU8C8` reader"]
pub type R = crate::R<Scu8c8Spec>;
#[doc = "Register `SCU8C8` writer"]
pub type W = crate::W<Scu8c8Spec>;
#[doc = "Field `SCUSCRATCHMCU50` reader - SCU_SCRATCH_MCU_50"]
pub type Scuscratchmcu50R = crate::FieldReader<u32>;
#[doc = "Field `SCUSCRATCHMCU50` writer - SCU_SCRATCH_MCU_50"]
pub type Scuscratchmcu50W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_50"]
    #[inline(always)]
    pub fn scuscratchmcu50(&self) -> Scuscratchmcu50R {
        Scuscratchmcu50R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_50"]
    #[inline(always)]
    pub fn scuscratchmcu50(&mut self) -> Scuscratchmcu50W<Scu8c8Spec> {
        Scuscratchmcu50W::new(self, 0)
    }
}
#[doc = "Scratch register for MCU 50\n\nYou can [`read`](crate::Reg::read) this register and get [`scu8c8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu8c8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu8c8Spec;
impl crate::RegisterSpec for Scu8c8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu8c8::R`](R) reader structure"]
impl crate::Readable for Scu8c8Spec {}
#[doc = "`write(|w| ..)` method takes [`scu8c8::W`](W) writer structure"]
impl crate::Writable for Scu8c8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU8C8 to value 0"]
impl crate::Resettable for Scu8c8Spec {}
