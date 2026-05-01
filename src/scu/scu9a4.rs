#[doc = "Register `SCU9A4` reader"]
pub type R = crate::R<Scu9a4Spec>;
#[doc = "Register `SCU9A4` writer"]
pub type W = crate::W<Scu9a4Spec>;
#[doc = "Field `SCUEFUSERSVDID3` reader - SCU_EFUSE_RSVD_ID3"]
pub type Scuefusersvdid3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_EFUSE_RSVD_ID3"]
    #[inline(always)]
    pub fn scuefusersvdid3(&self) -> Scuefusersvdid3R {
        Scuefusersvdid3R::new(self.bits)
    }
}
impl W {}
#[doc = "Reserved Read Only ID 3\n\nYou can [`read`](crate::Reg::read) this register and get [`scu9a4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu9a4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu9a4Spec;
impl crate::RegisterSpec for Scu9a4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu9a4::R`](R) reader structure"]
impl crate::Readable for Scu9a4Spec {}
#[doc = "`write(|w| ..)` method takes [`scu9a4::W`](W) writer structure"]
impl crate::Writable for Scu9a4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU9A4 to value 0"]
impl crate::Resettable for Scu9a4Spec {}
