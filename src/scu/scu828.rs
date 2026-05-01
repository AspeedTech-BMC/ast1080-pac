#[doc = "Register `SCU828` reader"]
pub type R = crate::R<Scu828Spec>;
#[doc = "Register `SCU828` writer"]
pub type W = crate::W<Scu828Spec>;
#[doc = "Field `SCUSCRATCHMCU10` reader - SCU_SCRATCH_MCU_10"]
pub type Scuscratchmcu10R = crate::FieldReader<u32>;
#[doc = "Field `SCUSCRATCHMCU10` writer - SCU_SCRATCH_MCU_10"]
pub type Scuscratchmcu10W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_10"]
    #[inline(always)]
    pub fn scuscratchmcu10(&self) -> Scuscratchmcu10R {
        Scuscratchmcu10R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_10"]
    #[inline(always)]
    pub fn scuscratchmcu10(&mut self) -> Scuscratchmcu10W<Scu828Spec> {
        Scuscratchmcu10W::new(self, 0)
    }
}
#[doc = "Scratch register for MCU 10\n\nYou can [`read`](crate::Reg::read) this register and get [`scu828::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu828::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu828Spec;
impl crate::RegisterSpec for Scu828Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu828::R`](R) reader structure"]
impl crate::Readable for Scu828Spec {}
#[doc = "`write(|w| ..)` method takes [`scu828::W`](W) writer structure"]
impl crate::Writable for Scu828Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU828 to value 0"]
impl crate::Resettable for Scu828Spec {}
