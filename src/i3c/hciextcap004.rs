#[doc = "Register `HCIEXTCAP004` reader"]
pub type R = crate::R<Hciextcap004Spec>;
#[doc = "Register `HCIEXTCAP004` writer"]
pub type W = crate::W<Hciextcap004Spec>;
#[doc = "Field `REGHWIDMIPIVENDORID` reader - REG_HW_ID_MIPI_VENDOR_ID"]
pub type ReghwidmipivendoridR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - REG_HW_ID_MIPI_VENDOR_ID"]
    #[inline(always)]
    pub fn reghwidmipivendorid(&self) -> ReghwidmipivendoridR {
        ReghwidmipivendoridR::new(self.bits)
    }
}
impl W {}
#[doc = "HW\\_ID\\_MIPI\\_VENDOR\n\nYou can [`read`](crate::Reg::read) this register and get [`hciextcap004::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hciextcap004::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hciextcap004Spec;
impl crate::RegisterSpec for Hciextcap004Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hciextcap004::R`](R) reader structure"]
impl crate::Readable for Hciextcap004Spec {}
#[doc = "`write(|w| ..)` method takes [`hciextcap004::W`](W) writer structure"]
impl crate::Writable for Hciextcap004Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCIEXTCAP004 to value 0x03f6"]
impl crate::Resettable for Hciextcap004Spec {
    const RESET_VALUE: u32 = 0x03f6;
}
