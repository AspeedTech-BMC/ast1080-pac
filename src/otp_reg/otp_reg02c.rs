#[doc = "Register `OTP_REG02C` reader"]
pub type R = crate::R<OtpReg02cSpec>;
#[doc = "Register `OTP_REG02C` writer"]
pub type W = crate::W<OtpReg02cSpec>;
#[doc = "Field `REGOTPWDATAM11` reader - REG_OTP_WDATA_M1_1"]
pub type Regotpwdatam11R = crate::FieldReader<u32>;
#[doc = "Field `REGOTPWDATAM11` writer - REG_OTP_WDATA_M1_1"]
pub type Regotpwdatam11W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_OTP_WDATA_M1_1"]
    #[inline(always)]
    pub fn regotpwdatam11(&self) -> Regotpwdatam11R {
        Regotpwdatam11R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_OTP_WDATA_M1_1"]
    #[inline(always)]
    pub fn regotpwdatam11(&mut self) -> Regotpwdatam11W<OtpReg02cSpec> {
        Regotpwdatam11W::new(self, 0)
    }
}
#[doc = "otp\\_wdata\\_m1\\_1\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg02c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg02c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg02cSpec;
impl crate::RegisterSpec for OtpReg02cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg02c::R`](R) reader structure"]
impl crate::Readable for OtpReg02cSpec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg02c::W`](W) writer structure"]
impl crate::Writable for OtpReg02cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG02C to value 0"]
impl crate::Resettable for OtpReg02cSpec {}
