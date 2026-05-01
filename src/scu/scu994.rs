#[doc = "Register `SCU994` reader"]
pub type R = crate::R<Scu994Spec>;
#[doc = "Register `SCU994` writer"]
pub type W = crate::W<Scu994Spec>;
#[doc = "Field `SCUEFUSECHIPID1` reader - SCU_EFUSE_CHIPID1"]
pub type Scuefusechipid1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_EFUSE_CHIPID1"]
    #[inline(always)]
    pub fn scuefusechipid1(&self) -> Scuefusechipid1R {
        Scuefusechipid1R::new(self.bits)
    }
}
impl W {}
#[doc = "Chip Unique ID 1\n\nYou can [`read`](crate::Reg::read) this register and get [`scu994::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu994::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu994Spec;
impl crate::RegisterSpec for Scu994Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu994::R`](R) reader structure"]
impl crate::Readable for Scu994Spec {}
#[doc = "`write(|w| ..)` method takes [`scu994::W`](W) writer structure"]
impl crate::Writable for Scu994Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU994 to value 0"]
impl crate::Resettable for Scu994Spec {}
