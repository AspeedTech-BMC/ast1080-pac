#[doc = "Register `SCU830` reader"]
pub type R = crate::R<Scu830Spec>;
#[doc = "Register `SCU830` writer"]
pub type W = crate::W<Scu830Spec>;
#[doc = "Field `SCUSCRATCHMCU12` reader - SCU_SCRATCH_MCU_12"]
pub type Scuscratchmcu12R = crate::FieldReader<u32>;
#[doc = "Field `SCUSCRATCHMCU12` writer - SCU_SCRATCH_MCU_12"]
pub type Scuscratchmcu12W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_12"]
    #[inline(always)]
    pub fn scuscratchmcu12(&self) -> Scuscratchmcu12R {
        Scuscratchmcu12R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_12"]
    #[inline(always)]
    pub fn scuscratchmcu12(&mut self) -> Scuscratchmcu12W<Scu830Spec> {
        Scuscratchmcu12W::new(self, 0)
    }
}
#[doc = "Scratch register for MCU 12\n\nYou can [`read`](crate::Reg::read) this register and get [`scu830::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu830::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu830Spec;
impl crate::RegisterSpec for Scu830Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu830::R`](R) reader structure"]
impl crate::Readable for Scu830Spec {}
#[doc = "`write(|w| ..)` method takes [`scu830::W`](W) writer structure"]
impl crate::Writable for Scu830Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU830 to value 0"]
impl crate::Resettable for Scu830Spec {}
