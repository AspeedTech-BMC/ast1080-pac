#[doc = "Register `OTP_REG000` reader"]
pub type R = crate::R<OtpReg000Spec>;
#[doc = "Register `OTP_REG000` writer"]
pub type W = crate::W<OtpReg000Spec>;
#[doc = "Field `REGOTPENABLEID` reader - REG_OTP_ENABLE_ID"]
pub type RegotpenableidR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - REG_OTP_ENABLE_ID"]
    #[inline(always)]
    pub fn regotpenableid(&self) -> RegotpenableidR {
        RegotpenableidR::new(self.bits)
    }
}
impl W {}
#[doc = "otp key\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg000::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg000::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg000Spec;
impl crate::RegisterSpec for OtpReg000Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg000::R`](R) reader structure"]
impl crate::Readable for OtpReg000Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg000::W`](W) writer structure"]
impl crate::Writable for OtpReg000Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG000 to value 0"]
impl crate::Resettable for OtpReg000Spec {}
