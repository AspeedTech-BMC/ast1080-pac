#[doc = "Register `HCIEXTCAP014` reader"]
pub type R = crate::R<Hciextcap014Spec>;
#[doc = "Register `HCIEXTCAP014` writer"]
pub type W = crate::W<Hciextcap014Spec>;
#[doc = "Field `REGCTLCFGOPERATIONMODE` reader - REG_CTL_CFG_OPERATION_MODE"]
pub type RegctlcfgoperationmodeR = crate::FieldReader;
impl R {
    #[doc = "Bits 4:5 - REG_CTL_CFG_OPERATION_MODE"]
    #[inline(always)]
    pub fn regctlcfgoperationmode(&self) -> RegctlcfgoperationmodeR {
        RegctlcfgoperationmodeR::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {}
#[doc = "CTL\\_CFG\\_OPERATION\\_MODE\n\nYou can [`read`](crate::Reg::read) this register and get [`hciextcap014::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hciextcap014::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hciextcap014Spec;
impl crate::RegisterSpec for Hciextcap014Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hciextcap014::R`](R) reader structure"]
impl crate::Readable for Hciextcap014Spec {}
#[doc = "`write(|w| ..)` method takes [`hciextcap014::W`](W) writer structure"]
impl crate::Writable for Hciextcap014Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCIEXTCAP014 to value 0x30"]
impl crate::Resettable for Hciextcap014Spec {
    const RESET_VALUE: u32 = 0x30;
}
