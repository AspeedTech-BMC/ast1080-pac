#[doc = "Register `OTP_REG04C` reader"]
pub type R = crate::R<OtpReg04cSpec>;
#[doc = "Register `OTP_REG04C` writer"]
pub type W = crate::W<OtpReg04cSpec>;
#[doc = "Field `REGOTPWDATAM21` reader - REG_OTP_WDATA_M2_1"]
pub type Regotpwdatam21R = crate::FieldReader<u32>;
#[doc = "Field `REGOTPWDATAM21` writer - REG_OTP_WDATA_M2_1"]
pub type Regotpwdatam21W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_OTP_WDATA_M2_1"]
    #[inline(always)]
    pub fn regotpwdatam21(&self) -> Regotpwdatam21R {
        Regotpwdatam21R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_OTP_WDATA_M2_1"]
    #[inline(always)]
    pub fn regotpwdatam21(&mut self) -> Regotpwdatam21W<OtpReg04cSpec> {
        Regotpwdatam21W::new(self, 0)
    }
}
#[doc = "otp\\_wdata\\_m2\\_1\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg04c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg04c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg04cSpec;
impl crate::RegisterSpec for OtpReg04cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg04c::R`](R) reader structure"]
impl crate::Readable for OtpReg04cSpec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg04c::W`](W) writer structure"]
impl crate::Writable for OtpReg04cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG04C to value 0"]
impl crate::Resettable for OtpReg04cSpec {}
