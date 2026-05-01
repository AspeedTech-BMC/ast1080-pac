#[doc = "Register `OTP_REG1D0` reader"]
pub type R = crate::R<OtpReg1d0Spec>;
#[doc = "Register `OTP_REG1D0` writer"]
pub type W = crate::W<OtpReg1d0Spec>;
#[doc = "Field `REGCALIPTRAOWNERKEY` reader - REG_CALIPTRA_OWNER_KEY"]
pub type RegcaliptraownerkeyR = crate::FieldReader<u16>;
#[doc = "Field `REGCALIPTRAOWNERKEY` writer - REG_CALIPTRA_OWNER_KEY"]
pub type RegcaliptraownerkeyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `REGCALIPTRAOWNERWLOCK` reader - REG_CALIPTRA_OWNER_WLOCK"]
pub type RegcaliptraownerwlockR = crate::BitReader;
#[doc = "Field `REGCALIPTRAOWNERWLOCK` writer - REG_CALIPTRA_OWNER_WLOCK"]
pub type RegcaliptraownerwlockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - REG_CALIPTRA_OWNER_KEY"]
    #[inline(always)]
    pub fn regcaliptraownerkey(&self) -> RegcaliptraownerkeyR {
        RegcaliptraownerkeyR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - REG_CALIPTRA_OWNER_WLOCK"]
    #[inline(always)]
    pub fn regcaliptraownerwlock(&self) -> RegcaliptraownerwlockR {
        RegcaliptraownerwlockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - REG_CALIPTRA_OWNER_KEY"]
    #[inline(always)]
    pub fn regcaliptraownerkey(&mut self) -> RegcaliptraownerkeyW<OtpReg1d0Spec> {
        RegcaliptraownerkeyW::new(self, 0)
    }
    #[doc = "Bit 31 - REG_CALIPTRA_OWNER_WLOCK"]
    #[inline(always)]
    pub fn regcaliptraownerwlock(&mut self) -> RegcaliptraownerwlockW<OtpReg1d0Spec> {
        RegcaliptraownerwlockW::new(self, 31)
    }
}
#[doc = "OTP\\_CALPITRA\\_OWNER\\_KEY\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg1d0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg1d0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg1d0Spec;
impl crate::RegisterSpec for OtpReg1d0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg1d0::R`](R) reader structure"]
impl crate::Readable for OtpReg1d0Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg1d0::W`](W) writer structure"]
impl crate::Writable for OtpReg1d0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG1D0 to value 0"]
impl crate::Resettable for OtpReg1d0Spec {}
