#[doc = "Register `OTP_REG0E0` reader"]
pub type R = crate::R<OtpReg0e0Spec>;
#[doc = "Register `OTP_REG0E0` writer"]
pub type W = crate::W<OtpReg0e0Spec>;
#[doc = "Field `REGDBGTRNG` reader - REG_DBG_TRNG"]
pub type RegdbgtrngR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - REG_DBG_TRNG"]
    #[inline(always)]
    pub fn regdbgtrng(&self) -> RegdbgtrngR {
        RegdbgtrngR::new(self.bits)
    }
}
impl W {}
#[doc = "otp\\_trng\\_dbg\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg0e0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg0e0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg0e0Spec;
impl crate::RegisterSpec for OtpReg0e0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg0e0::R`](R) reader structure"]
impl crate::Readable for OtpReg0e0Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg0e0::W`](W) writer structure"]
impl crate::Writable for OtpReg0e0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG0E0 to value 0"]
impl crate::Resettable for OtpReg0e0Spec {}
