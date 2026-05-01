#[doc = "Register `SCU000` reader"]
pub type R = crate::R<Scu000Spec>;
#[doc = "Register `SCU000` writer"]
pub type W = crate::W<Scu000Spec>;
#[doc = "Field `SCUIDBACK` reader - SCU_ID_BACK"]
pub type ScuidbackR = crate::FieldReader;
#[doc = "Field `SCUIDFUSE` reader - SCU_ID_FUSE"]
pub type ScuidfuseR = crate::FieldReader;
#[doc = "Field `SCUIDREV` reader - SCU_ID_REV"]
pub type ScuidrevR = crate::FieldReader;
#[doc = "Field `SCUIDGEN` reader - SCU_ID_GEN"]
pub type ScuidgenR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - SCU_ID_BACK"]
    #[inline(always)]
    pub fn scuidback(&self) -> ScuidbackR {
        ScuidbackR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - SCU_ID_FUSE"]
    #[inline(always)]
    pub fn scuidfuse(&self) -> ScuidfuseR {
        ScuidfuseR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - SCU_ID_REV"]
    #[inline(always)]
    pub fn scuidrev(&self) -> ScuidrevR {
        ScuidrevR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - SCU_ID_GEN"]
    #[inline(always)]
    pub fn scuidgen(&self) -> ScuidgenR {
        ScuidgenR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {}
#[doc = "Silicon Revision ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu000::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu000::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu000Spec;
impl crate::RegisterSpec for Scu000Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu000::R`](R) reader structure"]
impl crate::Readable for Scu000Spec {}
#[doc = "`write(|w| ..)` method takes [`scu000::W`](W) writer structure"]
impl crate::Writable for Scu000Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU000 to value 0x8100_0000"]
impl crate::Resettable for Scu000Spec {
    const RESET_VALUE: u32 = 0x8100_0000;
}
