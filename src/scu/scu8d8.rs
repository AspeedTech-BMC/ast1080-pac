#[doc = "Register `SCU8D8` reader"]
pub type R = crate::R<Scu8d8Spec>;
#[doc = "Register `SCU8D8` writer"]
pub type W = crate::W<Scu8d8Spec>;
#[doc = "Field `SCUSCRATCHMCU54` reader - SCU_SCRATCH_MCU_54"]
pub type Scuscratchmcu54R = crate::FieldReader<u32>;
#[doc = "Field `SCUSCRATCHMCU54` writer - SCU_SCRATCH_MCU_54"]
pub type Scuscratchmcu54W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_54"]
    #[inline(always)]
    pub fn scuscratchmcu54(&self) -> Scuscratchmcu54R {
        Scuscratchmcu54R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_54"]
    #[inline(always)]
    pub fn scuscratchmcu54(&mut self) -> Scuscratchmcu54W<Scu8d8Spec> {
        Scuscratchmcu54W::new(self, 0)
    }
}
#[doc = "Scratch register for MCU 54\n\nYou can [`read`](crate::Reg::read) this register and get [`scu8d8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu8d8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu8d8Spec;
impl crate::RegisterSpec for Scu8d8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu8d8::R`](R) reader structure"]
impl crate::Readable for Scu8d8Spec {}
#[doc = "`write(|w| ..)` method takes [`scu8d8::W`](W) writer structure"]
impl crate::Writable for Scu8d8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU8D8 to value 0"]
impl crate::Resettable for Scu8d8Spec {}
