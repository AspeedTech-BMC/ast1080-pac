#[doc = "Register `OTP_REG1C8` reader"]
pub type R = crate::R<OtpReg1c8Spec>;
#[doc = "Register `OTP_REG1C8` writer"]
pub type W = crate::W<OtpReg1c8Spec>;
#[doc = "Field `REGSOCKEY` reader - REG_SOC_KEY"]
pub type RegsockeyR = crate::FieldReader<u16>;
#[doc = "Field `REGSOCKEY` writer - REG_SOC_KEY"]
pub type RegsockeyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `REGSOCKEYWLOCK` reader - REG_SOC_KEY_WLOCK"]
pub type RegsockeywlockR = crate::BitReader;
#[doc = "Field `REGSOCKEYWLOCK` writer - REG_SOC_KEY_WLOCK"]
pub type RegsockeywlockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - REG_SOC_KEY"]
    #[inline(always)]
    pub fn regsockey(&self) -> RegsockeyR {
        RegsockeyR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - REG_SOC_KEY_WLOCK"]
    #[inline(always)]
    pub fn regsockeywlock(&self) -> RegsockeywlockR {
        RegsockeywlockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - REG_SOC_KEY"]
    #[inline(always)]
    pub fn regsockey(&mut self) -> RegsockeyW<OtpReg1c8Spec> {
        RegsockeyW::new(self, 0)
    }
    #[doc = "Bit 31 - REG_SOC_KEY_WLOCK"]
    #[inline(always)]
    pub fn regsockeywlock(&mut self) -> RegsockeywlockW<OtpReg1c8Spec> {
        RegsockeywlockW::new(self, 31)
    }
}
#[doc = "OTP\\_SOC\\_KEY\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg1c8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg1c8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg1c8Spec;
impl crate::RegisterSpec for OtpReg1c8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg1c8::R`](R) reader structure"]
impl crate::Readable for OtpReg1c8Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg1c8::W`](W) writer structure"]
impl crate::Writable for OtpReg1c8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG1C8 to value 0"]
impl crate::Resettable for OtpReg1c8Spec {}
