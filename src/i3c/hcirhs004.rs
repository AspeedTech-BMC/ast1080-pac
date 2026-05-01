#[doc = "Register `HCIRHS004` reader"]
pub type R = crate::R<Hcirhs004Spec>;
#[doc = "Register `HCIRHS004` writer"]
pub type W = crate::W<Hcirhs004Spec>;
#[doc = "Field `REGRH0OFFSET` reader - REG_RH0_OFFSET"]
pub type Regrh0offsetR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - REG_RH0_OFFSET"]
    #[inline(always)]
    pub fn regrh0offset(&self) -> Regrh0offsetR {
        Regrh0offsetR::new(self.bits)
    }
}
impl W {}
#[doc = "RH0\\_OFFSET\n\nYou can [`read`](crate::Reg::read) this register and get [`hcirhs004::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcirhs004::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hcirhs004Spec;
impl crate::RegisterSpec for Hcirhs004Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcirhs004::R`](R) reader structure"]
impl crate::Readable for Hcirhs004Spec {}
#[doc = "`write(|w| ..)` method takes [`hcirhs004::W`](W) writer structure"]
impl crate::Writable for Hcirhs004Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCIRHS004 to value 0x0830"]
impl crate::Resettable for Hcirhs004Spec {
    const RESET_VALUE: u32 = 0x0830;
}
