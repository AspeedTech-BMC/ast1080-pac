#[doc = "Register `SCUBDC` reader"]
pub type R = crate::R<ScubdcSpec>;
#[doc = "Register `SCUBDC` writer"]
pub type W = crate::W<ScubdcSpec>;
#[doc = "Field `SCUHWPUF23` reader - SCU_HW_PUF_23"]
pub type Scuhwpuf23R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_HW_PUF_23"]
    #[inline(always)]
    pub fn scuhwpuf23(&self) -> Scuhwpuf23R {
        Scuhwpuf23R::new(self.bits)
    }
}
impl W {}
#[doc = "HW PUF Register 23\n\nYou can [`read`](crate::Reg::read) this register and get [`scubdc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scubdc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScubdcSpec;
impl crate::RegisterSpec for ScubdcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scubdc::R`](R) reader structure"]
impl crate::Readable for ScubdcSpec {}
#[doc = "`write(|w| ..)` method takes [`scubdc::W`](W) writer structure"]
impl crate::Writable for ScubdcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUBDC to value 0"]
impl crate::Resettable for ScubdcSpec {}
