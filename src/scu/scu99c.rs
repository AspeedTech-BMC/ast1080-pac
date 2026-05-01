#[doc = "Register `SCU99C` reader"]
pub type R = crate::R<Scu99cSpec>;
#[doc = "Register `SCU99C` writer"]
pub type W = crate::W<Scu99cSpec>;
#[doc = "Field `SCUEFUSERSVDID1` reader - SCU_EFUSE_RSVD_ID1"]
pub type Scuefusersvdid1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_EFUSE_RSVD_ID1"]
    #[inline(always)]
    pub fn scuefusersvdid1(&self) -> Scuefusersvdid1R {
        Scuefusersvdid1R::new(self.bits)
    }
}
impl W {}
#[doc = "Reserved Read Only ID 1\n\nYou can [`read`](crate::Reg::read) this register and get [`scu99c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu99c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu99cSpec;
impl crate::RegisterSpec for Scu99cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu99c::R`](R) reader structure"]
impl crate::Readable for Scu99cSpec {}
#[doc = "`write(|w| ..)` method takes [`scu99c::W`](W) writer structure"]
impl crate::Writable for Scu99cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU99C to value 0"]
impl crate::Resettable for Scu99cSpec {}
