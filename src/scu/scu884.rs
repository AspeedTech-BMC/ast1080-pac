#[doc = "Register `SCU884` reader"]
pub type R = crate::R<Scu884Spec>;
#[doc = "Register `SCU884` writer"]
pub type W = crate::W<Scu884Spec>;
#[doc = "Field `SCUSCRATCHMCU33` reader - SCU_SCRATCH_MCU_33"]
pub type Scuscratchmcu33R = crate::FieldReader<u32>;
#[doc = "Field `SCUSCRATCHMCU33` writer - SCU_SCRATCH_MCU_33"]
pub type Scuscratchmcu33W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_33"]
    #[inline(always)]
    pub fn scuscratchmcu33(&self) -> Scuscratchmcu33R {
        Scuscratchmcu33R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_33"]
    #[inline(always)]
    pub fn scuscratchmcu33(&mut self) -> Scuscratchmcu33W<Scu884Spec> {
        Scuscratchmcu33W::new(self, 0)
    }
}
#[doc = "Scratch register for MCU 33\n\nYou can [`read`](crate::Reg::read) this register and get [`scu884::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu884::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu884Spec;
impl crate::RegisterSpec for Scu884Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu884::R`](R) reader structure"]
impl crate::Readable for Scu884Spec {}
#[doc = "`write(|w| ..)` method takes [`scu884::W`](W) writer structure"]
impl crate::Writable for Scu884Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU884 to value 0"]
impl crate::Resettable for Scu884Spec {}
