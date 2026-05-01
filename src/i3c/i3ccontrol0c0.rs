#[doc = "Register `I3CCONTROL0C0` reader"]
pub type R = crate::R<I3ccontrol0c0Spec>;
#[doc = "Register `I3CCONTROL0C0` writer"]
pub type W = crate::W<I3ccontrol0c0Spec>;
#[doc = "Field `REGRSTI3CTIME` reader - REG_RST_I3C_TIME"]
pub type Regrsti3ctimeR = crate::FieldReader;
#[doc = "Field `REGRSTI3CTIME` writer - REG_RST_I3C_TIME"]
pub type Regrsti3ctimeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGRSTWHOLECHIPTIME` reader - REG_RST_WHOLE_CHIP_TIME"]
pub type RegrstwholechiptimeR = crate::FieldReader;
#[doc = "Field `REGRSTWHOLECHIPTIME` writer - REG_RST_WHOLE_CHIP_TIME"]
pub type RegrstwholechiptimeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGRSTDBGNETWORKTIME` reader - REG_RST_DBG_NETWORK_TIME"]
pub type RegrstdbgnetworktimeR = crate::FieldReader;
#[doc = "Field `REGRSTDBGNETWORKTIME` writer - REG_RST_DBG_NETWORK_TIME"]
pub type RegrstdbgnetworktimeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - REG_RST_I3C_TIME"]
    #[inline(always)]
    pub fn regrsti3ctime(&self) -> Regrsti3ctimeR {
        Regrsti3ctimeR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - REG_RST_WHOLE_CHIP_TIME"]
    #[inline(always)]
    pub fn regrstwholechiptime(&self) -> RegrstwholechiptimeR {
        RegrstwholechiptimeR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - REG_RST_DBG_NETWORK_TIME"]
    #[inline(always)]
    pub fn regrstdbgnetworktime(&self) -> RegrstdbgnetworktimeR {
        RegrstdbgnetworktimeR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - REG_RST_I3C_TIME"]
    #[inline(always)]
    pub fn regrsti3ctime(&mut self) -> Regrsti3ctimeW<I3ccontrol0c0Spec> {
        Regrsti3ctimeW::new(self, 0)
    }
    #[doc = "Bits 8:15 - REG_RST_WHOLE_CHIP_TIME"]
    #[inline(always)]
    pub fn regrstwholechiptime(&mut self) -> RegrstwholechiptimeW<I3ccontrol0c0Spec> {
        RegrstwholechiptimeW::new(self, 8)
    }
    #[doc = "Bits 16:23 - REG_RST_DBG_NETWORK_TIME"]
    #[inline(always)]
    pub fn regrstdbgnetworktime(&mut self) -> RegrstdbgnetworktimeW<I3ccontrol0c0Spec> {
        RegrstdbgnetworktimeW::new(self, 16)
    }
}
#[doc = "I3C\\_SLV\\_CTL\\_0C0\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol0c0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol0c0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3ccontrol0c0Spec;
impl crate::RegisterSpec for I3ccontrol0c0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3ccontrol0c0::R`](R) reader structure"]
impl crate::Readable for I3ccontrol0c0Spec {}
#[doc = "`write(|w| ..)` method takes [`i3ccontrol0c0::W`](W) writer structure"]
impl crate::Writable for I3ccontrol0c0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CCONTROL0C0 to value 0"]
impl crate::Resettable for I3ccontrol0c0Spec {}
