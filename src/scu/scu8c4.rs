#[doc = "Register `SCU8C4` reader"]
pub type R = crate::R<Scu8c4Spec>;
#[doc = "Register `SCU8C4` writer"]
pub type W = crate::W<Scu8c4Spec>;
#[doc = "Field `SCUSCRATCHMCU49` reader - SCU_SCRATCH_MCU_49"]
pub type Scuscratchmcu49R = crate::FieldReader<u32>;
#[doc = "Field `SCUSCRATCHMCU49` writer - SCU_SCRATCH_MCU_49"]
pub type Scuscratchmcu49W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_49"]
    #[inline(always)]
    pub fn scuscratchmcu49(&self) -> Scuscratchmcu49R {
        Scuscratchmcu49R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_49"]
    #[inline(always)]
    pub fn scuscratchmcu49(&mut self) -> Scuscratchmcu49W<Scu8c4Spec> {
        Scuscratchmcu49W::new(self, 0)
    }
}
#[doc = "Scratch register for MCU 49\n\nYou can [`read`](crate::Reg::read) this register and get [`scu8c4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu8c4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu8c4Spec;
impl crate::RegisterSpec for Scu8c4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu8c4::R`](R) reader structure"]
impl crate::Readable for Scu8c4Spec {}
#[doc = "`write(|w| ..)` method takes [`scu8c4::W`](W) writer structure"]
impl crate::Writable for Scu8c4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU8C4 to value 0"]
impl crate::Resettable for Scu8c4Spec {}
