#[doc = "Register `OTP_REG20C` reader"]
pub type R = crate::R<OtpReg20cSpec>;
#[doc = "Register `OTP_REG20C` writer"]
pub type W = crate::W<OtpReg20cSpec>;
#[doc = "Field `REGINTRINFOFUNC` reader - REG_INTR_INFO_FUNC"]
pub type RegintrinfofuncR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - REG_INTR_INFO_FUNC"]
    #[inline(always)]
    pub fn regintrinfofunc(&self) -> RegintrinfofuncR {
        RegintrinfofuncR::new(self.bits)
    }
}
impl W {}
#[doc = "OTP\\_INTR\\_FUNC\\_INFO\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg20c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg20c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg20cSpec;
impl crate::RegisterSpec for OtpReg20cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg20c::R`](R) reader structure"]
impl crate::Readable for OtpReg20cSpec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg20c::W`](W) writer structure"]
impl crate::Writable for OtpReg20cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG20C to value 0"]
impl crate::Resettable for OtpReg20cSpec {}
