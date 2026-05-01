#[doc = "Register `SCU850` reader"]
pub type R = crate::R<Scu850Spec>;
#[doc = "Register `SCU850` writer"]
pub type W = crate::W<Scu850Spec>;
#[doc = "Field `SCUSCRATCHMCU20` reader - SCU_SCRATCH_MCU_20"]
pub type Scuscratchmcu20R = crate::FieldReader<u32>;
#[doc = "Field `SCUSCRATCHMCU20` writer - SCU_SCRATCH_MCU_20"]
pub type Scuscratchmcu20W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_20"]
    #[inline(always)]
    pub fn scuscratchmcu20(&self) -> Scuscratchmcu20R {
        Scuscratchmcu20R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_20"]
    #[inline(always)]
    pub fn scuscratchmcu20(&mut self) -> Scuscratchmcu20W<Scu850Spec> {
        Scuscratchmcu20W::new(self, 0)
    }
}
#[doc = "Scratch register for MCU 20\n\nYou can [`read`](crate::Reg::read) this register and get [`scu850::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu850::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu850Spec;
impl crate::RegisterSpec for Scu850Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu850::R`](R) reader structure"]
impl crate::Readable for Scu850Spec {}
#[doc = "`write(|w| ..)` method takes [`scu850::W`](W) writer structure"]
impl crate::Writable for Scu850Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU850 to value 0"]
impl crate::Resettable for Scu850Spec {}
