#[doc = "Register `OTP_REG210` reader"]
pub type R = crate::R<OtpReg210Spec>;
#[doc = "Register `OTP_REG210` writer"]
pub type W = crate::W<OtpReg210Spec>;
#[doc = "Field `REGINTRINFOMASTER` reader - REG_INTR_INFO_MASTER"]
pub type RegintrinfomasterR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - REG_INTR_INFO_MASTER"]
    #[inline(always)]
    pub fn regintrinfomaster(&self) -> RegintrinfomasterR {
        RegintrinfomasterR::new(self.bits)
    }
}
impl W {}
#[doc = "OTP\\_INTR\\_M\\_INFO\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg210::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg210::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg210Spec;
impl crate::RegisterSpec for OtpReg210Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg210::R`](R) reader structure"]
impl crate::Readable for OtpReg210Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg210::W`](W) writer structure"]
impl crate::Writable for OtpReg210Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG210 to value 0"]
impl crate::Resettable for OtpReg210Spec {}
