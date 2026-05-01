#[doc = "Register `SCU8AC` reader"]
pub type R = crate::R<Scu8acSpec>;
#[doc = "Register `SCU8AC` writer"]
pub type W = crate::W<Scu8acSpec>;
#[doc = "Field `SCUSCRATCHMCU43` reader - SCU_SCRATCH_MCU_43"]
pub type Scuscratchmcu43R = crate::FieldReader<u32>;
#[doc = "Field `SCUSCRATCHMCU43` writer - SCU_SCRATCH_MCU_43"]
pub type Scuscratchmcu43W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_43"]
    #[inline(always)]
    pub fn scuscratchmcu43(&self) -> Scuscratchmcu43R {
        Scuscratchmcu43R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_43"]
    #[inline(always)]
    pub fn scuscratchmcu43(&mut self) -> Scuscratchmcu43W<Scu8acSpec> {
        Scuscratchmcu43W::new(self, 0)
    }
}
#[doc = "Scratch register for MCU 43\n\nYou can [`read`](crate::Reg::read) this register and get [`scu8ac::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu8ac::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu8acSpec;
impl crate::RegisterSpec for Scu8acSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu8ac::R`](R) reader structure"]
impl crate::Readable for Scu8acSpec {}
#[doc = "`write(|w| ..)` method takes [`scu8ac::W`](W) writer structure"]
impl crate::Writable for Scu8acSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU8AC to value 0"]
impl crate::Resettable for Scu8acSpec {}
