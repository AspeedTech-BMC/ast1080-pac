#[doc = "Register `SCU888` reader"]
pub type R = crate::R<Scu888Spec>;
#[doc = "Register `SCU888` writer"]
pub type W = crate::W<Scu888Spec>;
#[doc = "Field `SCUSCRATCHMCU34` reader - SCU_SCRATCH_MCU_34"]
pub type Scuscratchmcu34R = crate::FieldReader<u32>;
#[doc = "Field `SCUSCRATCHMCU34` writer - SCU_SCRATCH_MCU_34"]
pub type Scuscratchmcu34W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_34"]
    #[inline(always)]
    pub fn scuscratchmcu34(&self) -> Scuscratchmcu34R {
        Scuscratchmcu34R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_34"]
    #[inline(always)]
    pub fn scuscratchmcu34(&mut self) -> Scuscratchmcu34W<Scu888Spec> {
        Scuscratchmcu34W::new(self, 0)
    }
}
#[doc = "Scratch register for MCU 34\n\nYou can [`read`](crate::Reg::read) this register and get [`scu888::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu888::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu888Spec;
impl crate::RegisterSpec for Scu888Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu888::R`](R) reader structure"]
impl crate::Readable for Scu888Spec {}
#[doc = "`write(|w| ..)` method takes [`scu888::W`](W) writer structure"]
impl crate::Writable for Scu888Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU888 to value 0"]
impl crate::Resettable for Scu888Spec {}
