#[doc = "Register `SCU86C` reader"]
pub type R = crate::R<Scu86cSpec>;
#[doc = "Register `SCU86C` writer"]
pub type W = crate::W<Scu86cSpec>;
#[doc = "Field `SCUSCRATCHMCU27` reader - SCU_SCRATCH_MCU_27"]
pub type Scuscratchmcu27R = crate::FieldReader<u32>;
#[doc = "Field `SCUSCRATCHMCU27` writer - SCU_SCRATCH_MCU_27"]
pub type Scuscratchmcu27W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_27"]
    #[inline(always)]
    pub fn scuscratchmcu27(&self) -> Scuscratchmcu27R {
        Scuscratchmcu27R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_27"]
    #[inline(always)]
    pub fn scuscratchmcu27(&mut self) -> Scuscratchmcu27W<Scu86cSpec> {
        Scuscratchmcu27W::new(self, 0)
    }
}
#[doc = "Scratch register for MCU 27\n\nYou can [`read`](crate::Reg::read) this register and get [`scu86c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu86c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu86cSpec;
impl crate::RegisterSpec for Scu86cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu86c::R`](R) reader structure"]
impl crate::Readable for Scu86cSpec {}
#[doc = "`write(|w| ..)` method takes [`scu86c::W`](W) writer structure"]
impl crate::Writable for Scu86cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU86C to value 0"]
impl crate::Resettable for Scu86cSpec {}
