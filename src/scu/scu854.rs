#[doc = "Register `SCU854` reader"]
pub type R = crate::R<Scu854Spec>;
#[doc = "Register `SCU854` writer"]
pub type W = crate::W<Scu854Spec>;
#[doc = "Field `SCUSCRATCHMCU21` reader - SCU_SCRATCH_MCU_21"]
pub type Scuscratchmcu21R = crate::FieldReader<u32>;
#[doc = "Field `SCUSCRATCHMCU21` writer - SCU_SCRATCH_MCU_21"]
pub type Scuscratchmcu21W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_21"]
    #[inline(always)]
    pub fn scuscratchmcu21(&self) -> Scuscratchmcu21R {
        Scuscratchmcu21R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_21"]
    #[inline(always)]
    pub fn scuscratchmcu21(&mut self) -> Scuscratchmcu21W<Scu854Spec> {
        Scuscratchmcu21W::new(self, 0)
    }
}
#[doc = "Scratch register for MCU 21\n\nYou can [`read`](crate::Reg::read) this register and get [`scu854::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu854::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu854Spec;
impl crate::RegisterSpec for Scu854Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu854::R`](R) reader structure"]
impl crate::Readable for Scu854Spec {}
#[doc = "`write(|w| ..)` method takes [`scu854::W`](W) writer structure"]
impl crate::Writable for Scu854Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU854 to value 0"]
impl crate::Resettable for Scu854Spec {}
