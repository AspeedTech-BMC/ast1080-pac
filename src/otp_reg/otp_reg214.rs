#[doc = "Register `OTP_REG214` reader"]
pub type R = crate::R<OtpReg214Spec>;
#[doc = "Register `OTP_REG214` writer"]
pub type W = crate::W<OtpReg214Spec>;
#[doc = "Field `REGINTRINFOREGION` reader - REG_INTR_INFO_REGION"]
pub type RegintrinforegionR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - REG_INTR_INFO_REGION"]
    #[inline(always)]
    pub fn regintrinforegion(&self) -> RegintrinforegionR {
        RegintrinforegionR::new(self.bits)
    }
}
impl W {}
#[doc = "OTP\\_INTR\\_R\\_INFO\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg214::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg214::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg214Spec;
impl crate::RegisterSpec for OtpReg214Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg214::R`](R) reader structure"]
impl crate::Readable for OtpReg214Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg214::W`](W) writer structure"]
impl crate::Writable for OtpReg214Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG214 to value 0"]
impl crate::Resettable for OtpReg214Spec {}
