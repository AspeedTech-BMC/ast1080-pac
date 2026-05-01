#[doc = "Register `OTP_REG00C` reader"]
pub type R = crate::R<OtpReg00cSpec>;
#[doc = "Register `OTP_REG00C` writer"]
pub type W = crate::W<OtpReg00cSpec>;
#[doc = "Field `REGOTPWDATAM01` reader - REG_OTP_WDATA_M0_1"]
pub type Regotpwdatam01R = crate::FieldReader<u32>;
#[doc = "Field `REGOTPWDATAM01` writer - REG_OTP_WDATA_M0_1"]
pub type Regotpwdatam01W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_OTP_WDATA_M0_1"]
    #[inline(always)]
    pub fn regotpwdatam01(&self) -> Regotpwdatam01R {
        Regotpwdatam01R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_OTP_WDATA_M0_1"]
    #[inline(always)]
    pub fn regotpwdatam01(&mut self) -> Regotpwdatam01W<OtpReg00cSpec> {
        Regotpwdatam01W::new(self, 0)
    }
}
#[doc = "otp\\_wdata\\_m0\\_1\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg00c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg00c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg00cSpec;
impl crate::RegisterSpec for OtpReg00cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg00c::R`](R) reader structure"]
impl crate::Readable for OtpReg00cSpec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg00c::W`](W) writer structure"]
impl crate::Writable for OtpReg00cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG00C to value 0"]
impl crate::Resettable for OtpReg00cSpec {}
