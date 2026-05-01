#[doc = "Register `HCICAPABILITY008` reader"]
pub type R = crate::R<Hcicapability008Spec>;
#[doc = "Register `HCICAPABILITY008` writer"]
pub type W = crate::W<Hcicapability008Spec>;
#[doc = "Field `REGDYNAMICADDR` reader - REG_DYNAMIC_ADDR"]
pub type RegdynamicaddrR = crate::FieldReader;
#[doc = "Field `REGDYNAMICADDR` writer - REG_DYNAMIC_ADDR"]
pub type RegdynamicaddrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `REGDYNAMICADDRVALID` reader - REG_DYNAMIC_ADDR_VALID"]
pub type RegdynamicaddrvalidR = crate::BitReader;
#[doc = "Field `REGDYNAMICADDRVALID` writer - REG_DYNAMIC_ADDR_VALID"]
pub type RegdynamicaddrvalidW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 16:22 - REG_DYNAMIC_ADDR"]
    #[inline(always)]
    pub fn regdynamicaddr(&self) -> RegdynamicaddrR {
        RegdynamicaddrR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - REG_DYNAMIC_ADDR_VALID"]
    #[inline(always)]
    pub fn regdynamicaddrvalid(&self) -> RegdynamicaddrvalidR {
        RegdynamicaddrvalidR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 16:22 - REG_DYNAMIC_ADDR"]
    #[inline(always)]
    pub fn regdynamicaddr(&mut self) -> RegdynamicaddrW<Hcicapability008Spec> {
        RegdynamicaddrW::new(self, 16)
    }
    #[doc = "Bit 31 - REG_DYNAMIC_ADDR_VALID"]
    #[inline(always)]
    pub fn regdynamicaddrvalid(&mut self) -> RegdynamicaddrvalidW<Hcicapability008Spec> {
        RegdynamicaddrvalidW::new(self, 31)
    }
}
#[doc = "CONTROLLER\\_DEVICE\\_ADDR\n\nYou can [`read`](crate::Reg::read) this register and get [`hcicapability008::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcicapability008::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hcicapability008Spec;
impl crate::RegisterSpec for Hcicapability008Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcicapability008::R`](R) reader structure"]
impl crate::Readable for Hcicapability008Spec {}
#[doc = "`write(|w| ..)` method takes [`hcicapability008::W`](W) writer structure"]
impl crate::Writable for Hcicapability008Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCICAPABILITY008 to value 0"]
impl crate::Resettable for Hcicapability008Spec {}
