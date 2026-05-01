#[doc = "Register `OTP_REG06C` reader"]
pub type R = crate::R<OtpReg06cSpec>;
#[doc = "Register `OTP_REG06C` writer"]
pub type W = crate::W<OtpReg06cSpec>;
#[doc = "Field `REGOTPWDATAM31` reader - REG_OTP_WDATA_M3_1"]
pub type Regotpwdatam31R = crate::FieldReader<u32>;
#[doc = "Field `REGOTPWDATAM31` writer - REG_OTP_WDATA_M3_1"]
pub type Regotpwdatam31W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_OTP_WDATA_M3_1"]
    #[inline(always)]
    pub fn regotpwdatam31(&self) -> Regotpwdatam31R {
        Regotpwdatam31R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_OTP_WDATA_M3_1"]
    #[inline(always)]
    pub fn regotpwdatam31(&mut self) -> Regotpwdatam31W<OtpReg06cSpec> {
        Regotpwdatam31W::new(self, 0)
    }
}
#[doc = "otp\\_wdata\\_m3\\_1\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg06c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg06c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg06cSpec;
impl crate::RegisterSpec for OtpReg06cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg06c::R`](R) reader structure"]
impl crate::Readable for OtpReg06cSpec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg06c::W`](W) writer structure"]
impl crate::Writable for OtpReg06cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG06C to value 0"]
impl crate::Resettable for OtpReg06cSpec {}
