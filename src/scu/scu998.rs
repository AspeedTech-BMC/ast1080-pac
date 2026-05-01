#[doc = "Register `SCU998` reader"]
pub type R = crate::R<Scu998Spec>;
#[doc = "Register `SCU998` writer"]
pub type W = crate::W<Scu998Spec>;
#[doc = "Field `SCUEFUSERSVDID0` reader - SCU_EFUSE_RSVD_ID0"]
pub type Scuefusersvdid0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_EFUSE_RSVD_ID0"]
    #[inline(always)]
    pub fn scuefusersvdid0(&self) -> Scuefusersvdid0R {
        Scuefusersvdid0R::new(self.bits)
    }
}
impl W {}
#[doc = "Reserved Read Only ID 0\n\nYou can [`read`](crate::Reg::read) this register and get [`scu998::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu998::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu998Spec;
impl crate::RegisterSpec for Scu998Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu998::R`](R) reader structure"]
impl crate::Readable for Scu998Spec {}
#[doc = "`write(|w| ..)` method takes [`scu998::W`](W) writer structure"]
impl crate::Writable for Scu998Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU998 to value 0"]
impl crate::Resettable for Scu998Spec {}
