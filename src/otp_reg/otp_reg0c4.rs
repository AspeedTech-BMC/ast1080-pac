#[doc = "Register `OTP_REG0C4` reader"]
pub type R = crate::R<OtpReg0c4Spec>;
#[doc = "Register `OTP_REG0C4` writer"]
pub type W = crate::W<OtpReg0c4Spec>;
#[doc = "Field `REGDBG0` reader - REG_DBG0"]
pub type Regdbg0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - REG_DBG0"]
    #[inline(always)]
    pub fn regdbg0(&self) -> Regdbg0R {
        Regdbg0R::new(self.bits)
    }
}
impl W {}
#[doc = "otp\\_dbg\\_00\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg0c4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg0c4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg0c4Spec;
impl crate::RegisterSpec for OtpReg0c4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg0c4::R`](R) reader structure"]
impl crate::Readable for OtpReg0c4Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg0c4::W`](W) writer structure"]
impl crate::Writable for OtpReg0c4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG0C4 to value 0"]
impl crate::Resettable for OtpReg0c4Spec {}
