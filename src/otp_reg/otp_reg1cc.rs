#[doc = "Register `OTP_REG1CC` reader"]
pub type R = crate::R<OtpReg1ccSpec>;
#[doc = "Register `OTP_REG1CC` writer"]
pub type W = crate::W<OtpReg1ccSpec>;
#[doc = "Field `REGCALIPTRAMANUKEY` reader - REG_CALIPTRA_MANU_KEY"]
pub type RegcaliptramanukeyR = crate::FieldReader<u16>;
#[doc = "Field `REGCALIPTRAMANUKEY` writer - REG_CALIPTRA_MANU_KEY"]
pub type RegcaliptramanukeyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `REGCALIPTRAMANUWLOCK` reader - REG_CALIPTRA_MANU_WLOCK"]
pub type RegcaliptramanuwlockR = crate::BitReader;
#[doc = "Field `REGCALIPTRAMANUWLOCK` writer - REG_CALIPTRA_MANU_WLOCK"]
pub type RegcaliptramanuwlockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - REG_CALIPTRA_MANU_KEY"]
    #[inline(always)]
    pub fn regcaliptramanukey(&self) -> RegcaliptramanukeyR {
        RegcaliptramanukeyR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - REG_CALIPTRA_MANU_WLOCK"]
    #[inline(always)]
    pub fn regcaliptramanuwlock(&self) -> RegcaliptramanuwlockR {
        RegcaliptramanuwlockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - REG_CALIPTRA_MANU_KEY"]
    #[inline(always)]
    pub fn regcaliptramanukey(&mut self) -> RegcaliptramanukeyW<OtpReg1ccSpec> {
        RegcaliptramanukeyW::new(self, 0)
    }
    #[doc = "Bit 31 - REG_CALIPTRA_MANU_WLOCK"]
    #[inline(always)]
    pub fn regcaliptramanuwlock(&mut self) -> RegcaliptramanuwlockW<OtpReg1ccSpec> {
        RegcaliptramanuwlockW::new(self, 31)
    }
}
#[doc = "OTP\\_CALPITRA\\_MANU\\_KEY\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg1cc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg1cc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg1ccSpec;
impl crate::RegisterSpec for OtpReg1ccSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg1cc::R`](R) reader structure"]
impl crate::Readable for OtpReg1ccSpec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg1cc::W`](W) writer structure"]
impl crate::Writable for OtpReg1ccSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG1CC to value 0"]
impl crate::Resettable for OtpReg1ccSpec {}
