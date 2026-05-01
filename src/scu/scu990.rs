#[doc = "Register `SCU990` reader"]
pub type R = crate::R<Scu990Spec>;
#[doc = "Register `SCU990` writer"]
pub type W = crate::W<Scu990Spec>;
#[doc = "Field `SCUEFUSECHIPID0` reader - SCU_EFUSE_CHIPID0"]
pub type Scuefusechipid0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_EFUSE_CHIPID0"]
    #[inline(always)]
    pub fn scuefusechipid0(&self) -> Scuefusechipid0R {
        Scuefusechipid0R::new(self.bits)
    }
}
impl W {}
#[doc = "Chip Unique ID 0\n\nYou can [`read`](crate::Reg::read) this register and get [`scu990::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu990::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu990Spec;
impl crate::RegisterSpec for Scu990Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu990::R`](R) reader structure"]
impl crate::Readable for Scu990Spec {}
#[doc = "`write(|w| ..)` method takes [`scu990::W`](W) writer structure"]
impl crate::Writable for Scu990Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU990 to value 0"]
impl crate::Resettable for Scu990Spec {}
