#[doc = "Register `SCU80C` reader"]
pub type R = crate::R<Scu80cSpec>;
#[doc = "Register `SCU80C` writer"]
pub type W = crate::W<Scu80cSpec>;
#[doc = "Field `SCUSCRATCHMCU3` reader - SCU_SCRATCH_MCU_3"]
pub type Scuscratchmcu3R = crate::FieldReader<u32>;
#[doc = "Field `SCUSCRATCHMCU3` writer - SCU_SCRATCH_MCU_3"]
pub type Scuscratchmcu3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_3"]
    #[inline(always)]
    pub fn scuscratchmcu3(&self) -> Scuscratchmcu3R {
        Scuscratchmcu3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_3"]
    #[inline(always)]
    pub fn scuscratchmcu3(&mut self) -> Scuscratchmcu3W<Scu80cSpec> {
        Scuscratchmcu3W::new(self, 0)
    }
}
#[doc = "Scratch register for MCU 3\n\nYou can [`read`](crate::Reg::read) this register and get [`scu80c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu80c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu80cSpec;
impl crate::RegisterSpec for Scu80cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu80c::R`](R) reader structure"]
impl crate::Readable for Scu80cSpec {}
#[doc = "`write(|w| ..)` method takes [`scu80c::W`](W) writer structure"]
impl crate::Writable for Scu80cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU80C to value 0"]
impl crate::Resettable for Scu80cSpec {}
