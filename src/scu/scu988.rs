#[doc = "Register `SCU988` reader"]
pub type R = crate::R<Scu988Spec>;
#[doc = "Register `SCU988` writer"]
pub type W = crate::W<Scu988Spec>;
#[doc = "Field `SCUEFUSECMD` reader - SCU_EFUSE_CMD"]
pub type ScuefusecmdR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_EFUSE_CMD"]
    #[inline(always)]
    pub fn scuefusecmd(&self) -> ScuefusecmdR {
        ScuefusecmdR::new(self.bits)
    }
}
impl W {}
#[doc = "EFUSE Command Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu988::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu988::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu988Spec;
impl crate::RegisterSpec for Scu988Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu988::R`](R) reader structure"]
impl crate::Readable for Scu988Spec {}
#[doc = "`write(|w| ..)` method takes [`scu988::W`](W) writer structure"]
impl crate::Writable for Scu988Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU988 to value 0"]
impl crate::Resettable for Scu988Spec {}
