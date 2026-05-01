#[doc = "Register `OTP_REG068` reader"]
pub type R = crate::R<OtpReg068Spec>;
#[doc = "Register `OTP_REG068` writer"]
pub type W = crate::W<OtpReg068Spec>;
#[doc = "Field `REGOTPWDATAM30` reader - REG_OTP_WDATA_M3_0"]
pub type Regotpwdatam30R = crate::FieldReader<u32>;
#[doc = "Field `REGOTPWDATAM30` writer - REG_OTP_WDATA_M3_0"]
pub type Regotpwdatam30W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_OTP_WDATA_M3_0"]
    #[inline(always)]
    pub fn regotpwdatam30(&self) -> Regotpwdatam30R {
        Regotpwdatam30R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_OTP_WDATA_M3_0"]
    #[inline(always)]
    pub fn regotpwdatam30(&mut self) -> Regotpwdatam30W<OtpReg068Spec> {
        Regotpwdatam30W::new(self, 0)
    }
}
#[doc = "otp\\_wdata\\_m3\\_0\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg068::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg068::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg068Spec;
impl crate::RegisterSpec for OtpReg068Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg068::R`](R) reader structure"]
impl crate::Readable for OtpReg068Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg068::W`](W) writer structure"]
impl crate::Writable for OtpReg068Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG068 to value 0"]
impl crate::Resettable for OtpReg068Spec {}
