#[doc = "Register `OTP_REG0F8` reader"]
pub type R = crate::R<OtpReg0f8Spec>;
#[doc = "Register `OTP_REG0F8` writer"]
pub type W = crate::W<OtpReg0f8Spec>;
#[doc = "Field `REGPUFRTSTS` reader - REG_PUFRT_STS"]
pub type RegpufrtstsR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - REG_PUFRT_STS"]
    #[inline(always)]
    pub fn regpufrtsts(&self) -> RegpufrtstsR {
        RegpufrtstsR::new(self.bits)
    }
}
impl W {}
#[doc = "otp\\_puf\\_sts\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg0f8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg0f8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg0f8Spec;
impl crate::RegisterSpec for OtpReg0f8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg0f8::R`](R) reader structure"]
impl crate::Readable for OtpReg0f8Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg0f8::W`](W) writer structure"]
impl crate::Writable for OtpReg0f8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG0F8 to value 0"]
impl crate::Resettable for OtpReg0f8Spec {}
