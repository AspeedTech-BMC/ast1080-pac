#[doc = "Register `HCICAPABILITY064` reader"]
pub type R = crate::R<Hcicapability064Spec>;
#[doc = "Register `HCICAPABILITY064` writer"]
pub type W = crate::W<Hcicapability064Spec>;
#[doc = "Field `REGDEVCTXBASEHI` reader - REG_DEV_CTX_BASE_HI"]
pub type RegdevctxbasehiR = crate::FieldReader<u32>;
#[doc = "Field `REGDEVCTXBASEHI` writer - REG_DEV_CTX_BASE_HI"]
pub type RegdevctxbasehiW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_DEV_CTX_BASE_HI"]
    #[inline(always)]
    pub fn regdevctxbasehi(&self) -> RegdevctxbasehiR {
        RegdevctxbasehiR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_DEV_CTX_BASE_HI"]
    #[inline(always)]
    pub fn regdevctxbasehi(&mut self) -> RegdevctxbasehiW<Hcicapability064Spec> {
        RegdevctxbasehiW::new(self, 0)
    }
}
#[doc = "DEV\\_CTX\\_BASE\\_HI\n\nYou can [`read`](crate::Reg::read) this register and get [`hcicapability064::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcicapability064::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hcicapability064Spec;
impl crate::RegisterSpec for Hcicapability064Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcicapability064::R`](R) reader structure"]
impl crate::Readable for Hcicapability064Spec {}
#[doc = "`write(|w| ..)` method takes [`hcicapability064::W`](W) writer structure"]
impl crate::Writable for Hcicapability064Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCICAPABILITY064 to value 0"]
impl crate::Resettable for Hcicapability064Spec {}
