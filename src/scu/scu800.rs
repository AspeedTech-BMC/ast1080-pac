#[doc = "Register `SCU800` reader"]
pub type R = crate::R<Scu800Spec>;
#[doc = "Register `SCU800` writer"]
pub type W = crate::W<Scu800Spec>;
#[doc = "Field `SCUSCRATCHMCU0` reader - SCU_SCRATCH_MCU_0"]
pub type Scuscratchmcu0R = crate::FieldReader<u32>;
#[doc = "Field `SCUSCRATCHMCU0` writer - SCU_SCRATCH_MCU_0"]
pub type Scuscratchmcu0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_0"]
    #[inline(always)]
    pub fn scuscratchmcu0(&self) -> Scuscratchmcu0R {
        Scuscratchmcu0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_0"]
    #[inline(always)]
    pub fn scuscratchmcu0(&mut self) -> Scuscratchmcu0W<Scu800Spec> {
        Scuscratchmcu0W::new(self, 0)
    }
}
#[doc = "Scratch register for MCU 0\n\nYou can [`read`](crate::Reg::read) this register and get [`scu800::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu800::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu800Spec;
impl crate::RegisterSpec for Scu800Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu800::R`](R) reader structure"]
impl crate::Readable for Scu800Spec {}
#[doc = "`write(|w| ..)` method takes [`scu800::W`](W) writer structure"]
impl crate::Writable for Scu800Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU800 to value 0"]
impl crate::Resettable for Scu800Spec {}
