#[doc = "Register `HCIEXTCAP01C` reader"]
pub type R = crate::R<Hciextcap01cSpec>;
#[doc = "Register `HCIEXTCAP01C` writer"]
pub type W = crate::W<Hciextcap01cSpec>;
#[doc = "Field `REGEXTCAPCTRL` reader - REG_EXTCAP_CTRL"]
pub type RegextcapctrlR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - REG_EXTCAP_CTRL"]
    #[inline(always)]
    pub fn regextcapctrl(&self) -> RegextcapctrlR {
        RegextcapctrlR::new(self.bits)
    }
}
impl W {}
#[doc = "EXTCAP\\_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`hciextcap01c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hciextcap01c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hciextcap01cSpec;
impl crate::RegisterSpec for Hciextcap01cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hciextcap01c::R`](R) reader structure"]
impl crate::Readable for Hciextcap01cSpec {}
#[doc = "`write(|w| ..)` method takes [`hciextcap01c::W`](W) writer structure"]
impl crate::Writable for Hciextcap01cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCIEXTCAP01C to value 0x0d00"]
impl crate::Resettable for Hciextcap01cSpec {
    const RESET_VALUE: u32 = 0x0d00;
}
