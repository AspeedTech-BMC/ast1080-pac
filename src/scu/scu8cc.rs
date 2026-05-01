#[doc = "Register `SCU8CC` reader"]
pub type R = crate::R<Scu8ccSpec>;
#[doc = "Register `SCU8CC` writer"]
pub type W = crate::W<Scu8ccSpec>;
#[doc = "Field `SCUSCRATCHMCU51` reader - SCU_SCRATCH_MCU_51"]
pub type Scuscratchmcu51R = crate::FieldReader<u32>;
#[doc = "Field `SCUSCRATCHMCU51` writer - SCU_SCRATCH_MCU_51"]
pub type Scuscratchmcu51W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_51"]
    #[inline(always)]
    pub fn scuscratchmcu51(&self) -> Scuscratchmcu51R {
        Scuscratchmcu51R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_51"]
    #[inline(always)]
    pub fn scuscratchmcu51(&mut self) -> Scuscratchmcu51W<Scu8ccSpec> {
        Scuscratchmcu51W::new(self, 0)
    }
}
#[doc = "Scratch register for MCU 51\n\nYou can [`read`](crate::Reg::read) this register and get [`scu8cc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu8cc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu8ccSpec;
impl crate::RegisterSpec for Scu8ccSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu8cc::R`](R) reader structure"]
impl crate::Readable for Scu8ccSpec {}
#[doc = "`write(|w| ..)` method takes [`scu8cc::W`](W) writer structure"]
impl crate::Writable for Scu8ccSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU8CC to value 0"]
impl crate::Resettable for Scu8ccSpec {}
