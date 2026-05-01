#[doc = "Register `OTP_REG0B4` reader"]
pub type R = crate::R<OtpReg0b4Spec>;
#[doc = "Register `OTP_REG0B4` writer"]
pub type W = crate::W<OtpReg0b4Spec>;
#[doc = "Field `REGOTPWDATAM53` reader - REG_OTP_WDATA_M5_3"]
pub type Regotpwdatam53R = crate::FieldReader<u32>;
#[doc = "Field `REGOTPWDATAM53` writer - REG_OTP_WDATA_M5_3"]
pub type Regotpwdatam53W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_OTP_WDATA_M5_3"]
    #[inline(always)]
    pub fn regotpwdatam53(&self) -> Regotpwdatam53R {
        Regotpwdatam53R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_OTP_WDATA_M5_3"]
    #[inline(always)]
    pub fn regotpwdatam53(&mut self) -> Regotpwdatam53W<OtpReg0b4Spec> {
        Regotpwdatam53W::new(self, 0)
    }
}
#[doc = "otp\\_wdata\\_m5\\_3\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg0b4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg0b4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg0b4Spec;
impl crate::RegisterSpec for OtpReg0b4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg0b4::R`](R) reader structure"]
impl crate::Readable for OtpReg0b4Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg0b4::W`](W) writer structure"]
impl crate::Writable for OtpReg0b4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG0B4 to value 0"]
impl crate::Resettable for OtpReg0b4Spec {}
