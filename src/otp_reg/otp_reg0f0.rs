#[doc = "Register `OTP_REG0F0` reader"]
pub type R = crate::R<OtpReg0f0Spec>;
#[doc = "Register `OTP_REG0F0` writer"]
pub type W = crate::W<OtpReg0f0Spec>;
#[doc = "Field `REGOTPROMCLRADDR` reader - REG_OTPROM_CLR_ADDR"]
pub type RegotpromclraddrR = crate::FieldReader<u16>;
#[doc = "Field `REGOTPROMCLRADDR` writer - REG_OTPROM_CLR_ADDR"]
pub type RegotpromclraddrW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `REGOTPROMCLR` reader - REG_OTPROM_CLR"]
pub type RegotpromclrR = crate::BitReader;
#[doc = "Field `REGOTPROMCLR` writer - REG_OTPROM_CLR"]
pub type RegotpromclrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:12 - REG_OTPROM_CLR_ADDR"]
    #[inline(always)]
    pub fn regotpromclraddr(&self) -> RegotpromclraddrR {
        RegotpromclraddrR::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bit 31 - REG_OTPROM_CLR"]
    #[inline(always)]
    pub fn regotpromclr(&self) -> RegotpromclrR {
        RegotpromclrR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:12 - REG_OTPROM_CLR_ADDR"]
    #[inline(always)]
    pub fn regotpromclraddr(&mut self) -> RegotpromclraddrW<OtpReg0f0Spec> {
        RegotpromclraddrW::new(self, 0)
    }
    #[doc = "Bit 31 - REG_OTPROM_CLR"]
    #[inline(always)]
    pub fn regotpromclr(&mut self) -> RegotpromclrW<OtpReg0f0Spec> {
        RegotpromclrW::new(self, 31)
    }
}
#[doc = "otp\\_rom\\_clr\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg0f0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg0f0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg0f0Spec;
impl crate::RegisterSpec for OtpReg0f0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg0f0::R`](R) reader structure"]
impl crate::Readable for OtpReg0f0Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg0f0::W`](W) writer structure"]
impl crate::Writable for OtpReg0f0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG0F0 to value 0"]
impl crate::Resettable for OtpReg0f0Spec {}
