#[doc = "Register `OTP_REG0D8` reader"]
pub type R = crate::R<OtpReg0d8Spec>;
#[doc = "Register `OTP_REG0D8` writer"]
pub type W = crate::W<OtpReg0d8Spec>;
#[doc = "Field `REGCMPLOCK` reader - REG_CMP_LOCK"]
pub type RegcmplockR = crate::BitReader;
#[doc = "Field `REGCMPLOCK` writer - REG_CMP_LOCK"]
pub type RegcmplockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGPROGLOCK` reader - REG_PROG_LOCK"]
pub type RegproglockR = crate::BitReader;
#[doc = "Field `REGPROGLOCK` writer - REG_PROG_LOCK"]
pub type RegproglockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - REG_CMP_LOCK"]
    #[inline(always)]
    pub fn regcmplock(&self) -> RegcmplockR {
        RegcmplockR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - REG_PROG_LOCK"]
    #[inline(always)]
    pub fn regproglock(&self) -> RegproglockR {
        RegproglockR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - REG_CMP_LOCK"]
    #[inline(always)]
    pub fn regcmplock(&mut self) -> RegcmplockW<OtpReg0d8Spec> {
        RegcmplockW::new(self, 0)
    }
    #[doc = "Bit 1 - REG_PROG_LOCK"]
    #[inline(always)]
    pub fn regproglock(&mut self) -> RegproglockW<OtpReg0d8Spec> {
        RegproglockW::new(self, 1)
    }
}
#[doc = "OTP\\_CMD\\_LOCK\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg0d8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg0d8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg0d8Spec;
impl crate::RegisterSpec for OtpReg0d8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg0d8::R`](R) reader structure"]
impl crate::Readable for OtpReg0d8Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg0d8::W`](W) writer structure"]
impl crate::Writable for OtpReg0d8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG0D8 to value 0"]
impl crate::Resettable for OtpReg0d8Spec {}
