#[doc = "Register `SCU84C` reader"]
pub type R = crate::R<Scu84cSpec>;
#[doc = "Register `SCU84C` writer"]
pub type W = crate::W<Scu84cSpec>;
#[doc = "Field `SCUSCRATCHMCU19` reader - SCU_SCRATCH_MCU_19"]
pub type Scuscratchmcu19R = crate::FieldReader<u32>;
#[doc = "Field `SCUSCRATCHMCU19` writer - SCU_SCRATCH_MCU_19"]
pub type Scuscratchmcu19W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_19"]
    #[inline(always)]
    pub fn scuscratchmcu19(&self) -> Scuscratchmcu19R {
        Scuscratchmcu19R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_19"]
    #[inline(always)]
    pub fn scuscratchmcu19(&mut self) -> Scuscratchmcu19W<Scu84cSpec> {
        Scuscratchmcu19W::new(self, 0)
    }
}
#[doc = "Scratch register for MCU 19\n\nYou can [`read`](crate::Reg::read) this register and get [`scu84c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu84c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu84cSpec;
impl crate::RegisterSpec for Scu84cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu84c::R`](R) reader structure"]
impl crate::Readable for Scu84cSpec {}
#[doc = "`write(|w| ..)` method takes [`scu84c::W`](W) writer structure"]
impl crate::Writable for Scu84cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU84C to value 0"]
impl crate::Resettable for Scu84cSpec {}
