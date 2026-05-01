#[doc = "Register `HCIEXTCAP008` reader"]
pub type R = crate::R<Hciextcap008Spec>;
#[doc = "Register `HCIEXTCAP008` writer"]
pub type W = crate::W<Hciextcap008Spec>;
#[doc = "Field `REGHWIDI3CVERID` reader - REG_HW_ID_I3C_VER_ID"]
pub type Reghwidi3cveridR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - REG_HW_ID_I3C_VER_ID"]
    #[inline(always)]
    pub fn reghwidi3cverid(&self) -> Reghwidi3cveridR {
        Reghwidi3cveridR::new(self.bits)
    }
}
impl W {}
#[doc = "HW\\_ID\\_I3C\\_VER\n\nYou can [`read`](crate::Reg::read) this register and get [`hciextcap008::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hciextcap008::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hciextcap008Spec;
impl crate::RegisterSpec for Hciextcap008Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hciextcap008::R`](R) reader structure"]
impl crate::Readable for Hciextcap008Spec {}
#[doc = "`write(|w| ..)` method takes [`hciextcap008::W`](W) writer structure"]
impl crate::Writable for Hciextcap008Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCIEXTCAP008 to value 0x02"]
impl crate::Resettable for Hciextcap008Spec {
    const RESET_VALUE: u32 = 0x02;
}
