#[doc = "Register `I3CPHYCTRLREG000` reader"]
pub type R = crate::R<I3cphyctrlreg000Spec>;
#[doc = "Register `I3CPHYCTRLREG000` writer"]
pub type W = crate::W<I3cphyctrlreg000Spec>;
#[doc = "Field `REGI3CPHYCAPID` reader - REG_I3C_PHY_CAP_ID"]
pub type Regi3cphycapidR = crate::FieldReader;
#[doc = "Field `REGI3CPHYCAPLENGTH` reader - REG_I3C_PHY_CAP_LENGTH"]
pub type Regi3cphycaplengthR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:7 - REG_I3C_PHY_CAP_ID"]
    #[inline(always)]
    pub fn regi3cphycapid(&self) -> Regi3cphycapidR {
        Regi3cphycapidR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:23 - REG_I3C_PHY_CAP_LENGTH"]
    #[inline(always)]
    pub fn regi3cphycaplength(&self) -> Regi3cphycaplengthR {
        Regi3cphycaplengthR::new(((self.bits >> 8) & 0xffff) as u16)
    }
}
impl W {}
#[doc = "EXT\\_CAP\\_OFFSET\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg000::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg000::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cphyctrlreg000Spec;
impl crate::RegisterSpec for I3cphyctrlreg000Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cphyctrlreg000::R`](R) reader structure"]
impl crate::Readable for I3cphyctrlreg000Spec {}
#[doc = "`write(|w| ..)` method takes [`i3cphyctrlreg000::W`](W) writer structure"]
impl crate::Writable for I3cphyctrlreg000Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CPHYCTRLREG000 to value 0x0100"]
impl crate::Resettable for I3cphyctrlreg000Spec {
    const RESET_VALUE: u32 = 0x0100;
}
