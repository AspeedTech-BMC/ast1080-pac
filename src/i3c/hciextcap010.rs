#[doc = "Register `HCIEXTCAP010` reader"]
pub type R = crate::R<Hciextcap010Spec>;
#[doc = "Register `HCIEXTCAP010` writer"]
pub type W = crate::W<Hciextcap010Spec>;
#[doc = "Field `REGCTLCFGID` reader - REG_CTL_CFG_ID"]
pub type RegctlcfgidR = crate::FieldReader;
#[doc = "Field `REGCTLCFGLENGTH` reader - REG_CTL_CFG_LENGTH"]
pub type RegctlcfglengthR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:7 - REG_CTL_CFG_ID"]
    #[inline(always)]
    pub fn regctlcfgid(&self) -> RegctlcfgidR {
        RegctlcfgidR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:23 - REG_CTL_CFG_LENGTH"]
    #[inline(always)]
    pub fn regctlcfglength(&self) -> RegctlcfglengthR {
        RegctlcfglengthR::new(((self.bits >> 8) & 0xffff) as u16)
    }
}
impl W {}
#[doc = "CTL\\_CFG\\_HEADER\n\nYou can [`read`](crate::Reg::read) this register and get [`hciextcap010::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hciextcap010::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hciextcap010Spec;
impl crate::RegisterSpec for Hciextcap010Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hciextcap010::R`](R) reader structure"]
impl crate::Readable for Hciextcap010Spec {}
#[doc = "`write(|w| ..)` method takes [`hciextcap010::W`](W) writer structure"]
impl crate::Writable for Hciextcap010Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCIEXTCAP010 to value 0x0202"]
impl crate::Resettable for Hciextcap010Spec {
    const RESET_VALUE: u32 = 0x0202;
}
