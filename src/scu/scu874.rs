#[doc = "Register `SCU874` reader"]
pub type R = crate::R<Scu874Spec>;
#[doc = "Register `SCU874` writer"]
pub type W = crate::W<Scu874Spec>;
#[doc = "Field `SCUSCRATCHMCU29` reader - SCU_SCRATCH_MCU_29"]
pub type Scuscratchmcu29R = crate::FieldReader<u32>;
#[doc = "Field `SCUSCRATCHMCU29` writer - SCU_SCRATCH_MCU_29"]
pub type Scuscratchmcu29W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_29"]
    #[inline(always)]
    pub fn scuscratchmcu29(&self) -> Scuscratchmcu29R {
        Scuscratchmcu29R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_SCRATCH_MCU_29"]
    #[inline(always)]
    pub fn scuscratchmcu29(&mut self) -> Scuscratchmcu29W<Scu874Spec> {
        Scuscratchmcu29W::new(self, 0)
    }
}
#[doc = "Scratch register for MCU 29\n\nYou can [`read`](crate::Reg::read) this register and get [`scu874::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu874::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu874Spec;
impl crate::RegisterSpec for Scu874Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu874::R`](R) reader structure"]
impl crate::Readable for Scu874Spec {}
#[doc = "`write(|w| ..)` method takes [`scu874::W`](W) writer structure"]
impl crate::Writable for Scu874Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU874 to value 0"]
impl crate::Resettable for Scu874Spec {}
