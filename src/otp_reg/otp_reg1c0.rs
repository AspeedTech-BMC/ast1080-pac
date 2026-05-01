#[doc = "Register `OTP_REG1C0` reader"]
pub type R = crate::R<OtpReg1c0Spec>;
#[doc = "Register `OTP_REG1C0` writer"]
pub type W = crate::W<OtpReg1c0Spec>;
#[doc = "Field `REGSOCECCKEY` reader - REG_SOC_ECCKEY"]
pub type RegsocecckeyR = crate::FieldReader<u16>;
#[doc = "Field `REGSOCECCKEY` writer - REG_SOC_ECCKEY"]
pub type RegsocecckeyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `REGSOCECCKEYWLOCK` reader - REG_SOC_ECCKEY_WLOCK"]
pub type RegsocecckeywlockR = crate::BitReader;
#[doc = "Field `REGSOCECCKEYWLOCK` writer - REG_SOC_ECCKEY_WLOCK"]
pub type RegsocecckeywlockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - REG_SOC_ECCKEY"]
    #[inline(always)]
    pub fn regsocecckey(&self) -> RegsocecckeyR {
        RegsocecckeyR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - REG_SOC_ECCKEY_WLOCK"]
    #[inline(always)]
    pub fn regsocecckeywlock(&self) -> RegsocecckeywlockR {
        RegsocecckeywlockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - REG_SOC_ECCKEY"]
    #[inline(always)]
    pub fn regsocecckey(&mut self) -> RegsocecckeyW<OtpReg1c0Spec> {
        RegsocecckeyW::new(self, 0)
    }
    #[doc = "Bit 31 - REG_SOC_ECCKEY_WLOCK"]
    #[inline(always)]
    pub fn regsocecckeywlock(&mut self) -> RegsocecckeywlockW<OtpReg1c0Spec> {
        RegsocecckeywlockW::new(self, 31)
    }
}
#[doc = "OTP\\_SOC\\_ECCKEY\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg1c0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg1c0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg1c0Spec;
impl crate::RegisterSpec for OtpReg1c0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg1c0::R`](R) reader structure"]
impl crate::Readable for OtpReg1c0Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg1c0::W`](W) writer structure"]
impl crate::Writable for OtpReg1c0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG1C0 to value 0"]
impl crate::Resettable for OtpReg1c0Spec {}
