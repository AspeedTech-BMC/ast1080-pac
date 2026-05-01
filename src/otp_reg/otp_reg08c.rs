#[doc = "Register `OTP_REG08C` reader"]
pub type R = crate::R<OtpReg08cSpec>;
#[doc = "Register `OTP_REG08C` writer"]
pub type W = crate::W<OtpReg08cSpec>;
#[doc = "Field `REGOTPWDATAM41` reader - REG_OTP_WDATA_M4_1"]
pub type Regotpwdatam41R = crate::FieldReader<u32>;
#[doc = "Field `REGOTPWDATAM41` writer - REG_OTP_WDATA_M4_1"]
pub type Regotpwdatam41W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_OTP_WDATA_M4_1"]
    #[inline(always)]
    pub fn regotpwdatam41(&self) -> Regotpwdatam41R {
        Regotpwdatam41R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_OTP_WDATA_M4_1"]
    #[inline(always)]
    pub fn regotpwdatam41(&mut self) -> Regotpwdatam41W<OtpReg08cSpec> {
        Regotpwdatam41W::new(self, 0)
    }
}
#[doc = "otp\\_wdata\\_m4\\_1\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg08c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg08c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg08cSpec;
impl crate::RegisterSpec for OtpReg08cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg08c::R`](R) reader structure"]
impl crate::Readable for OtpReg08cSpec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg08c::W`](W) writer structure"]
impl crate::Writable for OtpReg08cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG08C to value 0"]
impl crate::Resettable for OtpReg08cSpec {}
