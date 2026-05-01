#[doc = "Register `HCICAPABILITY060` reader"]
pub type R = crate::R<Hcicapability060Spec>;
#[doc = "Register `HCICAPABILITY060` writer"]
pub type W = crate::W<Hcicapability060Spec>;
#[doc = "Field `REGDEVCTXBASELO` reader - REG_DEV_CTX_BASE_LO"]
pub type RegdevctxbaseloR = crate::FieldReader<u32>;
#[doc = "Field `REGDEVCTXBASELO` writer - REG_DEV_CTX_BASE_LO"]
pub type RegdevctxbaseloW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_DEV_CTX_BASE_LO"]
    #[inline(always)]
    pub fn regdevctxbaselo(&self) -> RegdevctxbaseloR {
        RegdevctxbaseloR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_DEV_CTX_BASE_LO"]
    #[inline(always)]
    pub fn regdevctxbaselo(&mut self) -> RegdevctxbaseloW<Hcicapability060Spec> {
        RegdevctxbaseloW::new(self, 0)
    }
}
#[doc = "DEV\\_CTX\\_BASE\\_LO\n\nYou can [`read`](crate::Reg::read) this register and get [`hcicapability060::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcicapability060::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hcicapability060Spec;
impl crate::RegisterSpec for Hcicapability060Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcicapability060::R`](R) reader structure"]
impl crate::Readable for Hcicapability060Spec {}
#[doc = "`write(|w| ..)` method takes [`hcicapability060::W`](W) writer structure"]
impl crate::Writable for Hcicapability060Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCICAPABILITY060 to value 0"]
impl crate::Resettable for Hcicapability060Spec {}
