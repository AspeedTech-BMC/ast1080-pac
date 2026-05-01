#[doc = "Register `OTP_REG108` reader"]
pub type R = crate::R<OtpReg108Spec>;
#[doc = "Register `OTP_REG108` writer"]
pub type W = crate::W<OtpReg108Spec>;
#[doc = "Field `REGREGIONOTPSTRAPREN` reader - REG_REGION_OTPSTRAP_REN"]
pub type RegregionotpstraprenR = crate::FieldReader;
#[doc = "Field `REGREGIONOTPSTRAPREN` writer - REG_REGION_OTPSTRAP_REN"]
pub type RegregionotpstraprenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGREGIONOTPSTRAPWEN` reader - REG_REGION_OTPSTRAP_WEN"]
pub type RegregionotpstrapwenR = crate::FieldReader;
#[doc = "Field `REGREGIONOTPSTRAPWEN` writer - REG_REGION_OTPSTRAP_WEN"]
pub type RegregionotpstrapwenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGREGIONOTPSTRAPLOCK` reader - REG_REGION_OTPSTRAP_LOCK"]
pub type RegregionotpstraplockR = crate::BitReader;
#[doc = "Field `REGREGIONOTPSTRAPLOCK` writer - REG_REGION_OTPSTRAP_LOCK"]
pub type RegregionotpstraplockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - REG_REGION_OTPSTRAP_REN"]
    #[inline(always)]
    pub fn regregionotpstrapren(&self) -> RegregionotpstraprenR {
        RegregionotpstraprenR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - REG_REGION_OTPSTRAP_WEN"]
    #[inline(always)]
    pub fn regregionotpstrapwen(&self) -> RegregionotpstrapwenR {
        RegregionotpstrapwenR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 31 - REG_REGION_OTPSTRAP_LOCK"]
    #[inline(always)]
    pub fn regregionotpstraplock(&self) -> RegregionotpstraplockR {
        RegregionotpstraplockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - REG_REGION_OTPSTRAP_REN"]
    #[inline(always)]
    pub fn regregionotpstrapren(&mut self) -> RegregionotpstraprenW<OtpReg108Spec> {
        RegregionotpstraprenW::new(self, 0)
    }
    #[doc = "Bits 8:15 - REG_REGION_OTPSTRAP_WEN"]
    #[inline(always)]
    pub fn regregionotpstrapwen(&mut self) -> RegregionotpstrapwenW<OtpReg108Spec> {
        RegregionotpstrapwenW::new(self, 8)
    }
    #[doc = "Bit 31 - REG_REGION_OTPSTRAP_LOCK"]
    #[inline(always)]
    pub fn regregionotpstraplock(&mut self) -> RegregionotpstraplockW<OtpReg108Spec> {
        RegregionotpstraplockW::new(self, 31)
    }
}
#[doc = "OTP\\_REGION\\_OTPSTRAP\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg108::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg108::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg108Spec;
impl crate::RegisterSpec for OtpReg108Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg108::R`](R) reader structure"]
impl crate::Readable for OtpReg108Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg108::W`](W) writer structure"]
impl crate::Writable for OtpReg108Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG108 to value 0xffff"]
impl crate::Resettable for OtpReg108Spec {
    const RESET_VALUE: u32 = 0xffff;
}
