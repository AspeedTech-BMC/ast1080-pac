#[doc = "Register `OTP_REG0CC` reader"]
pub type R = crate::R<OtpReg0ccSpec>;
#[doc = "Register `OTP_REG0CC` writer"]
pub type W = crate::W<OtpReg0ccSpec>;
#[doc = "Field `REGECCRLOCK` reader - REG_ECC_RLOCK"]
pub type RegeccrlockR = crate::BitReader;
#[doc = "Field `REGECCRLOCK` writer - REG_ECC_RLOCK"]
pub type RegeccrlockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - REG_ECC_RLOCK"]
    #[inline(always)]
    pub fn regeccrlock(&self) -> RegeccrlockR {
        RegeccrlockR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - REG_ECC_RLOCK"]
    #[inline(always)]
    pub fn regeccrlock(&mut self) -> RegeccrlockW<OtpReg0ccSpec> {
        RegeccrlockW::new(self, 0)
    }
}
#[doc = "ecc\\_rlock\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg0cc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg0cc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg0ccSpec;
impl crate::RegisterSpec for OtpReg0ccSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg0cc::R`](R) reader structure"]
impl crate::Readable for OtpReg0ccSpec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg0cc::W`](W) writer structure"]
impl crate::Writable for OtpReg0ccSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG0CC to value 0"]
impl crate::Resettable for OtpReg0ccSpec {}
