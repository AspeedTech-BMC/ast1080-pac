#[doc = "Register `OTP_REG1F0` reader"]
pub type R = crate::R<OtpReg1f0Spec>;
#[doc = "Register `OTP_REG1F0` writer"]
pub type W = crate::W<OtpReg1f0Spec>;
#[doc = "Field `REGFWHRIDWLOCK` reader - REG_FW_HRID_WLOCK"]
pub type RegfwhridwlockR = crate::BitReader;
#[doc = "Field `REGFWHRIDWLOCK` writer - REG_FW_HRID_WLOCK"]
pub type RegfwhridwlockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGCALIPTRAFMCSVNWLOCK` reader - REG_CALIPTRA_FMC_SVN_WLOCK"]
pub type RegcaliptrafmcsvnwlockR = crate::BitReader;
#[doc = "Field `REGCALIPTRAFMCSVNWLOCK` writer - REG_CALIPTRA_FMC_SVN_WLOCK"]
pub type RegcaliptrafmcsvnwlockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGCALIPTRARUNTIMESVNWLOCK` reader - REG_CALIPTRA_RUNTIME_SVN_WLOCK"]
pub type RegcaliptraruntimesvnwlockR = crate::BitReader;
#[doc = "Field `REGCALIPTRARUNTIMESVNWLOCK` writer - REG_CALIPTRA_RUNTIME_SVN_WLOCK"]
pub type RegcaliptraruntimesvnwlockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - REG_FW_HRID_WLOCK"]
    #[inline(always)]
    pub fn regfwhridwlock(&self) -> RegfwhridwlockR {
        RegfwhridwlockR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - REG_CALIPTRA_FMC_SVN_WLOCK"]
    #[inline(always)]
    pub fn regcaliptrafmcsvnwlock(&self) -> RegcaliptrafmcsvnwlockR {
        RegcaliptrafmcsvnwlockR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - REG_CALIPTRA_RUNTIME_SVN_WLOCK"]
    #[inline(always)]
    pub fn regcaliptraruntimesvnwlock(&self) -> RegcaliptraruntimesvnwlockR {
        RegcaliptraruntimesvnwlockR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - REG_FW_HRID_WLOCK"]
    #[inline(always)]
    pub fn regfwhridwlock(&mut self) -> RegfwhridwlockW<OtpReg1f0Spec> {
        RegfwhridwlockW::new(self, 0)
    }
    #[doc = "Bit 1 - REG_CALIPTRA_FMC_SVN_WLOCK"]
    #[inline(always)]
    pub fn regcaliptrafmcsvnwlock(&mut self) -> RegcaliptrafmcsvnwlockW<OtpReg1f0Spec> {
        RegcaliptrafmcsvnwlockW::new(self, 1)
    }
    #[doc = "Bit 2 - REG_CALIPTRA_RUNTIME_SVN_WLOCK"]
    #[inline(always)]
    pub fn regcaliptraruntimesvnwlock(&mut self) -> RegcaliptraruntimesvnwlockW<OtpReg1f0Spec> {
        RegcaliptraruntimesvnwlockW::new(self, 2)
    }
}
#[doc = "OTP\\_SVN\\_WLOCK\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg1f0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg1f0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg1f0Spec;
impl crate::RegisterSpec for OtpReg1f0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg1f0::R`](R) reader structure"]
impl crate::Readable for OtpReg1f0Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg1f0::W`](W) writer structure"]
impl crate::Writable for OtpReg1f0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG1F0 to value 0"]
impl crate::Resettable for OtpReg1f0Spec {}
