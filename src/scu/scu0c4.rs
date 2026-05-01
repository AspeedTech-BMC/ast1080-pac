#[doc = "Register `SCU0C4` reader"]
pub type R = crate::R<Scu0c4Spec>;
#[doc = "Register `SCU0C4` writer"]
pub type W = crate::W<Scu0c4Spec>;
#[doc = "Field `SCUDBGSEL` reader - SCU_DBG_SEL"]
pub type ScudbgselR = crate::FieldReader;
#[doc = "Field `SCUDBGSEL` writer - SCU_DBG_SEL"]
pub type ScudbgselW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SCURINGSEL` reader - SCU_RING_SEL"]
pub type ScuringselR = crate::FieldReader;
#[doc = "Field `SCURINGSEL` writer - SCU_RING_SEL"]
pub type ScuringselW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - SCU_DBG_SEL"]
    #[inline(always)]
    pub fn scudbgsel(&self) -> ScudbgselR {
        ScudbgselR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - SCU_RING_SEL"]
    #[inline(always)]
    pub fn scuringsel(&self) -> ScuringselR {
        ScuringselR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SCU_DBG_SEL"]
    #[inline(always)]
    pub fn scudbgsel(&mut self) -> ScudbgselW<Scu0c4Spec> {
        ScudbgselW::new(self, 0)
    }
    #[doc = "Bits 8:15 - SCU_RING_SEL"]
    #[inline(always)]
    pub fn scuringsel(&mut self) -> ScuringselW<Scu0c4Spec> {
        ScuringselW::new(self, 8)
    }
}
#[doc = "Debug Selection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu0c4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu0c4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu0c4Spec;
impl crate::RegisterSpec for Scu0c4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu0c4::R`](R) reader structure"]
impl crate::Readable for Scu0c4Spec {}
#[doc = "`write(|w| ..)` method takes [`scu0c4::W`](W) writer structure"]
impl crate::Writable for Scu0c4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU0C4 to value 0xff"]
impl crate::Resettable for Scu0c4Spec {
    const RESET_VALUE: u32 = 0xff;
}
