#[doc = "Register `SCU8BC` reader"]
pub type R = crate::R<Scu8bcSpec>;
#[doc = "Register `SCU8BC` writer"]
pub type W = crate::W<Scu8bcSpec>;
#[doc = "Field `SCUSCRATCHMCU47` reader - SCU_SCRATCH_MCU_47"]
pub type Scuscratchmcu47R = crate::FieldReader<u32>;
#[doc = "Field `SCUSCRATCHMCU47` writer - SCU_SCRATCH_MCU_47"]
pub type Scuscratchmcu47W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_47"]
    #[inline(always)]
    pub fn scuscratchmcu47(&self) -> Scuscratchmcu47R {
        Scuscratchmcu47R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_47"]
    #[inline(always)]
    pub fn scuscratchmcu47(&mut self) -> Scuscratchmcu47W<Scu8bcSpec> {
        Scuscratchmcu47W::new(self, 0)
    }
}
#[doc = "Scratch register for MCU 47\n\nYou can [`read`](crate::Reg::read) this register and get [`scu8bc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu8bc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu8bcSpec;
impl crate::RegisterSpec for Scu8bcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu8bc::R`](R) reader structure"]
impl crate::Readable for Scu8bcSpec {}
#[doc = "`write(|w| ..)` method takes [`scu8bc::W`](W) writer structure"]
impl crate::Writable for Scu8bcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU8BC to value 0"]
impl crate::Resettable for Scu8bcSpec {}
