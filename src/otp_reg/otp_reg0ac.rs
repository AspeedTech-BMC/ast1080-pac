#[doc = "Register `OTP_REG0AC` reader"]
pub type R = crate::R<OtpReg0acSpec>;
#[doc = "Register `OTP_REG0AC` writer"]
pub type W = crate::W<OtpReg0acSpec>;
#[doc = "Field `REGOTPWDATAM51` reader - REG_OTP_WDATA_M5_1"]
pub type Regotpwdatam51R = crate::FieldReader<u32>;
#[doc = "Field `REGOTPWDATAM51` writer - REG_OTP_WDATA_M5_1"]
pub type Regotpwdatam51W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_OTP_WDATA_M5_1"]
    #[inline(always)]
    pub fn regotpwdatam51(&self) -> Regotpwdatam51R {
        Regotpwdatam51R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_OTP_WDATA_M5_1"]
    #[inline(always)]
    pub fn regotpwdatam51(&mut self) -> Regotpwdatam51W<OtpReg0acSpec> {
        Regotpwdatam51W::new(self, 0)
    }
}
#[doc = "otp\\_wdata\\_m5\\_1\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg0ac::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg0ac::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg0acSpec;
impl crate::RegisterSpec for OtpReg0acSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg0ac::R`](R) reader structure"]
impl crate::Readable for OtpReg0acSpec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg0ac::W`](W) writer structure"]
impl crate::Writable for OtpReg0acSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG0AC to value 0"]
impl crate::Resettable for OtpReg0acSpec {}
