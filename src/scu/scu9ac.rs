#[doc = "Register `SCU9AC` reader"]
pub type R = crate::R<Scu9acSpec>;
#[doc = "Register `SCU9AC` writer"]
pub type W = crate::W<Scu9acSpec>;
#[doc = "Field `SCUEFUSERSVDID5` reader - SCU_EFUSE_RSVD_ID5"]
pub type Scuefusersvdid5R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_EFUSE_RSVD_ID5"]
    #[inline(always)]
    pub fn scuefusersvdid5(&self) -> Scuefusersvdid5R {
        Scuefusersvdid5R::new(self.bits)
    }
}
impl W {}
#[doc = "Reserved Read Only ID 5\n\nYou can [`read`](crate::Reg::read) this register and get [`scu9ac::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu9ac::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu9acSpec;
impl crate::RegisterSpec for Scu9acSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu9ac::R`](R) reader structure"]
impl crate::Readable for Scu9acSpec {}
#[doc = "`write(|w| ..)` method takes [`scu9ac::W`](W) writer structure"]
impl crate::Writable for Scu9acSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU9AC to value 0"]
impl crate::Resettable for Scu9acSpec {}
