#[doc = "Register `SCU88C` reader"]
pub type R = crate::R<Scu88cSpec>;
#[doc = "Register `SCU88C` writer"]
pub type W = crate::W<Scu88cSpec>;
#[doc = "Field `SCUSCRATCHMCU35` reader - SCU_SCRATCH_MCU_35"]
pub type Scuscratchmcu35R = crate::FieldReader<u32>;
#[doc = "Field `SCUSCRATCHMCU35` writer - SCU_SCRATCH_MCU_35"]
pub type Scuscratchmcu35W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_35"]
    #[inline(always)]
    pub fn scuscratchmcu35(&self) -> Scuscratchmcu35R {
        Scuscratchmcu35R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_35"]
    #[inline(always)]
    pub fn scuscratchmcu35(&mut self) -> Scuscratchmcu35W<Scu88cSpec> {
        Scuscratchmcu35W::new(self, 0)
    }
}
#[doc = "Scratch register for MCU 35\n\nYou can [`read`](crate::Reg::read) this register and get [`scu88c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu88c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu88cSpec;
impl crate::RegisterSpec for Scu88cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu88c::R`](R) reader structure"]
impl crate::Readable for Scu88cSpec {}
#[doc = "`write(|w| ..)` method takes [`scu88c::W`](W) writer structure"]
impl crate::Writable for Scu88cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU88C to value 0"]
impl crate::Resettable for Scu88cSpec {}
