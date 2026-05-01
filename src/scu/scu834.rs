#[doc = "Register `SCU834` reader"]
pub type R = crate::R<Scu834Spec>;
#[doc = "Register `SCU834` writer"]
pub type W = crate::W<Scu834Spec>;
#[doc = "Field `SCUSCRATCHMCU13` reader - SCU_SCRATCH_MCU_13"]
pub type Scuscratchmcu13R = crate::FieldReader<u32>;
#[doc = "Field `SCUSCRATCHMCU13` writer - SCU_SCRATCH_MCU_13"]
pub type Scuscratchmcu13W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_13"]
    #[inline(always)]
    pub fn scuscratchmcu13(&self) -> Scuscratchmcu13R {
        Scuscratchmcu13R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_13"]
    #[inline(always)]
    pub fn scuscratchmcu13(&mut self) -> Scuscratchmcu13W<Scu834Spec> {
        Scuscratchmcu13W::new(self, 0)
    }
}
#[doc = "Scratch register for MCU 13\n\nYou can [`read`](crate::Reg::read) this register and get [`scu834::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu834::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu834Spec;
impl crate::RegisterSpec for Scu834Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu834::R`](R) reader structure"]
impl crate::Readable for Scu834Spec {}
#[doc = "`write(|w| ..)` method takes [`scu834::W`](W) writer structure"]
impl crate::Writable for Scu834Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU834 to value 0"]
impl crate::Resettable for Scu834Spec {}
