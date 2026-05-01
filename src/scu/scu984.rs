#[doc = "Register `SCU984` reader"]
pub type R = crate::R<Scu984Spec>;
#[doc = "Register `SCU984` writer"]
pub type W = crate::W<Scu984Spec>;
#[doc = "Field `SCUEFUSEDATA` reader - SCU_EFUSE_DATA"]
pub type ScuefusedataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_EFUSE_DATA"]
    #[inline(always)]
    pub fn scuefusedata(&self) -> ScuefusedataR {
        ScuefusedataR::new(self.bits)
    }
}
impl W {}
#[doc = "EFUSE Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu984::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu984::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu984Spec;
impl crate::RegisterSpec for Scu984Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu984::R`](R) reader structure"]
impl crate::Readable for Scu984Spec {}
#[doc = "`write(|w| ..)` method takes [`scu984::W`](W) writer structure"]
impl crate::Writable for Scu984Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU984 to value 0"]
impl crate::Resettable for Scu984Spec {}
