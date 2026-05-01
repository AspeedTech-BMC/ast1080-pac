#[doc = "Register `OTP_REG1DC` reader"]
pub type R = crate::R<OtpReg1dcSpec>;
#[doc = "Register `OTP_REG1DC` writer"]
pub type W = crate::W<OtpReg1dcSpec>;
#[doc = "Field `REGCALIPTRAFMCSVN` reader - REG_CALIPTRA_FMC_SVN"]
pub type RegcaliptrafmcsvnR = crate::FieldReader<u32>;
#[doc = "Field `REGCALIPTRAFMCSVN` writer - REG_CALIPTRA_FMC_SVN"]
pub type RegcaliptrafmcsvnW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_CALIPTRA_FMC_SVN"]
    #[inline(always)]
    pub fn regcaliptrafmcsvn(&self) -> RegcaliptrafmcsvnR {
        RegcaliptrafmcsvnR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_CALIPTRA_FMC_SVN"]
    #[inline(always)]
    pub fn regcaliptrafmcsvn(&mut self) -> RegcaliptrafmcsvnW<OtpReg1dcSpec> {
        RegcaliptrafmcsvnW::new(self, 0)
    }
}
#[doc = "OTP\\_CALIP\\_FMC\\_SVN\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg1dc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg1dc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg1dcSpec;
impl crate::RegisterSpec for OtpReg1dcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg1dc::R`](R) reader structure"]
impl crate::Readable for OtpReg1dcSpec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg1dc::W`](W) writer structure"]
impl crate::Writable for OtpReg1dcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG1DC to value 0"]
impl crate::Resettable for OtpReg1dcSpec {}
