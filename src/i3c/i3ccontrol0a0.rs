#[doc = "Register `I3CCONTROL0A0` reader"]
pub type R = crate::R<I3ccontrol0a0Spec>;
#[doc = "Register `I3CCONTROL0A0` writer"]
pub type W = crate::W<I3ccontrol0a0Spec>;
#[doc = "Field `REGSTATICADDR` reader - REG_STATIC_ADDR"]
pub type RegstaticaddrR = crate::FieldReader;
#[doc = "Field `REGSTATICADDR` writer - REG_STATIC_ADDR"]
pub type RegstaticaddrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `REGSTATICADDREN` reader - REG_STATIC_ADDR_EN"]
pub type RegstaticaddrenR = crate::BitReader;
#[doc = "Field `REGSTATICADDREN` writer - REG_STATIC_ADDR_EN"]
pub type RegstaticaddrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGSLVBCR` reader - REG_SLV_BCR"]
pub type RegslvbcrR = crate::FieldReader;
#[doc = "Field `REGSLVBCR` writer - REG_SLV_BCR"]
pub type RegslvbcrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGSLVDCR` reader - REG_SLV_DCR"]
pub type RegslvdcrR = crate::FieldReader;
#[doc = "Field `REGSLVDCR` writer - REG_SLV_DCR"]
pub type RegslvdcrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:6 - REG_STATIC_ADDR"]
    #[inline(always)]
    pub fn regstaticaddr(&self) -> RegstaticaddrR {
        RegstaticaddrR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - REG_STATIC_ADDR_EN"]
    #[inline(always)]
    pub fn regstaticaddren(&self) -> RegstaticaddrenR {
        RegstaticaddrenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - REG_SLV_BCR"]
    #[inline(always)]
    pub fn regslvbcr(&self) -> RegslvbcrR {
        RegslvbcrR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - REG_SLV_DCR"]
    #[inline(always)]
    pub fn regslvdcr(&self) -> RegslvdcrR {
        RegslvdcrR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - REG_STATIC_ADDR"]
    #[inline(always)]
    pub fn regstaticaddr(&mut self) -> RegstaticaddrW<I3ccontrol0a0Spec> {
        RegstaticaddrW::new(self, 0)
    }
    #[doc = "Bit 7 - REG_STATIC_ADDR_EN"]
    #[inline(always)]
    pub fn regstaticaddren(&mut self) -> RegstaticaddrenW<I3ccontrol0a0Spec> {
        RegstaticaddrenW::new(self, 7)
    }
    #[doc = "Bits 8:15 - REG_SLV_BCR"]
    #[inline(always)]
    pub fn regslvbcr(&mut self) -> RegslvbcrW<I3ccontrol0a0Spec> {
        RegslvbcrW::new(self, 8)
    }
    #[doc = "Bits 16:23 - REG_SLV_DCR"]
    #[inline(always)]
    pub fn regslvdcr(&mut self) -> RegslvdcrW<I3ccontrol0a0Spec> {
        RegslvdcrW::new(self, 16)
    }
}
#[doc = "I3C\\_SLV\\_CTL\\_0A0\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol0a0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol0a0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3ccontrol0a0Spec;
impl crate::RegisterSpec for I3ccontrol0a0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3ccontrol0a0::R`](R) reader structure"]
impl crate::Readable for I3ccontrol0a0Spec {}
#[doc = "`write(|w| ..)` method takes [`i3ccontrol0a0::W`](W) writer structure"]
impl crate::Writable for I3ccontrol0a0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CCONTROL0A0 to value 0"]
impl crate::Resettable for I3ccontrol0a0Spec {}
